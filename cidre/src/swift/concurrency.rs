//! Swift concurrency runtime bindings and shared assembly for hand-written
//! async task trampolines.
//!
//! Rust cannot express Swift's async calling convention. A suspending call
//! requires the caller to allocate the callee's context with `swift_task_alloc`
//! (sized from the callee's async function pointer, read at runtime), pass it
//! in `x22`, store a resume function pointer into it, and then tail-call a
//! callee that never returns: it branches to the resume symbol instead. One
//! logical `await` is therefore several machine-level functions sharing a heap
//! frame, and Rust has no construct that produces multiple entry points for one
//! function. `#[unsafe(naked)]` functions built from the fragments below are the
//! closest fit, since each is a symbol whose body is raw assembly and which
//! never returns normally.
//!
//! Each fragment has a pointer-authentication variant, so a trampoline is
//! written once rather than once per `paca` setting. The variants are
//! interchangeable: [`swift_async_load_parent`] always leaves the parent context
//! in `x9` and [`swift_async_load_resume`] always leaves the resume pointer in
//! `x16`, signed or not.

#![allow(dead_code, unused_macros, unused_imports)]

use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    ptr::NonNull,
};

use core::arch::asm;

use super::{
    SwiftMetadata, abi,
    abi::TypeMetadata,
    value::{AnyValue, DynamicStorage, Optional},
};

/// Medium-priority Swift task that is enqueued immediately, returns no value,
/// and consumes its opaque context without applying Swift ARC to it.
pub(crate) const ENQUEUED_DISCARDING_TASK_FLAGS: usize = 0x15 | (1 << 12) | (1 << 14) | (1 << 15);

#[link(name = "swift_Concurrency")]
unsafe extern "C" {
    /// Only ever referenced as a `sym` operand, so the signature is unused.
    pub(crate) fn swift_task_alloc();
    pub(crate) fn swift_task_dealloc();
    pub(crate) fn swift_task_switch();

    /// Returns the new task and the address of its initial context, which is
    /// two words and so comes back in registers under C's convention as it
    /// does under Swift's.
    fn swift_task_create(
        flags: usize,
        options: *mut (),
        future_result_type: *const TypeMetadata,
        function: *const (),
        context: *mut (),
    ) -> CreatedTask;

    /// Arms a continuation embedded in `context`, returning the handle that
    /// resumes it. Declared `SWIFT_CC(swift)`, which for these arguments is the
    /// C convention.
    fn swift_continuation_init(context: *mut (), flags: usize) -> *mut ();

    /// Resumes a continuation from outside the task. Also effectively C.
    fn swift_continuation_resume(continuation: *mut ());

    /// Flags a task cancelled, which its callee observes at its next suspension
    /// point. One pointer argument, so `SWIFT_CC(swift)` is C here too.
    fn swift_task_cancel(task: *mut ());

    /// `SWIFT_CC(swiftasync)`, and only ever referenced as a `sym` operand.
    pub(crate) fn swift_continuation_await();

    #[link_name = "$sScPMa"]
    pub(crate) fn task_priority_metadata();

    #[link_name = "$sSci17makeAsyncIterator0bC0QzyFTj"]
    pub(crate) fn async_sequence_make_iterator();

    #[link_name = "$sSciTL"]
    pub(crate) static ASYNC_SEQUENCE_ASSOCIATED_TYPES: u8;

    #[link_name = "$sSci13AsyncIteratorSci_ScITn"]
    pub(crate) static ASYNC_ITERATOR_CONFORMANCE: u8;

    #[link_name = "$sScI4next9isolation7ElementQzSgScA_pSgYi_tYa7FailureQzYKFTj"]
    pub(crate) fn async_iterator_next();

    #[link_name = "$sScI4next9isolation7ElementQzSgScA_pSgYi_tYa7FailureQzYKFTjTu"]
    pub(crate) static ASYNC_ITERATOR_NEXT_ASYNC_FN: u8;
}

crate::define_swift_marker!(pub(crate) TaskPriority = accessor task_priority_metadata);

crate::impl_swift_sendable!(TaskPriority);

/// What `swift_task_create` hands back.
#[repr(C)]
struct CreatedTask {
    task: *mut (),
    initial_context: *mut (),
}

/// An owned reference to a running Swift task.
///
/// Worth keeping only for something that will act on the task later: the
/// runtime holds its own reference through the enqueued job, so a task nobody
/// keeps still runs to completion and frees itself.
#[repr(transparent)]
struct SwiftTask(NonNull<()>);

// Tasks are atomically reference-counted by the runtime, and cancelling one is
// defined from any thread, so this may be held wherever the future ends up.
unsafe impl Send for SwiftTask {}
unsafe impl Sync for SwiftTask {}

impl SwiftTask {
    /// Asks the task to stop at its next suspension point.
    ///
    /// Advisory, as it is in Swift: the callee decides what to do about it, and
    /// a call already past its last suspension point finishes regardless. It is
    /// a no-op on a task that has already completed, which is why holding the
    /// reference is what makes this safe to call at any time.
    #[inline]
    fn cancel(&self) {
        unsafe { swift_task_cancel(self.0.as_ptr()) }
    }
}

impl Drop for SwiftTask {
    #[inline]
    fn drop(&mut self) {
        unsafe { abi::object_release(self.0.as_ptr().cast_const()) }
    }
}

/// Starts a task running `descriptor` over `context`, and hands back the
/// reference the runtime returns.
///
/// # Safety
///
/// `descriptor` must be an async function pointer whose entry point takes
/// `context`, and the task takes ownership of whatever `context` points to.
#[inline]
unsafe fn start_task(descriptor: *const u8, context: *mut ()) -> SwiftTask {
    let created = unsafe {
        swift_task_create(
            ENQUEUED_DISCARDING_TASK_FLAGS,
            core::ptr::null_mut(),
            core::ptr::null(),
            descriptor.cast(),
            context,
        )
    };
    SwiftTask(NonNull::new(created.task).expect("Swift task creation failed"))
}

/// Starts a task and lets go of it, which is what Swift's own codegen does for
/// a task whose handle is discarded.
///
/// # Safety
///
/// As [`start_task`].
#[inline]
pub(crate) unsafe fn spawn_task(descriptor: *const u8, context: *mut ()) {
    drop(unsafe { start_task(descriptor, context) });
}

/// A suspending call that is being awaited rather than handed a callback.
///
/// The awaiting state is the task's own tail, so one allocation carries the
/// context Swift runs over, what the call borrowed, and the slot the result
/// lands in. `Arc` is what makes that shareable: the task holds one reference
/// until it completes and the future holds the other until it is dropped, so
/// neither has to outlive the other and whichever finishes last frees.
#[cfg(all(feature = "async", feature = "ns"))]
struct AwaitedCall<O, T> {
    /// Turns what the call produced into the value the future yields.
    ///
    /// A plain function pointer: every binding's conversion is generated and
    /// captures nothing, so there is nothing here to allocate for.
    output: fn(O, *mut ()) -> T,
    state: parking_lot::Mutex<AwaitedState<T>>,
}

#[cfg(all(feature = "async", feature = "ns"))]
struct AwaitedState<T> {
    ready: Option<Result<T, crate::arc::R<crate::ns::Error>>>,
    waker: Option<std::task::Waker>,
}

/// The future a suspending Swift call returns.
///
/// Dropping it before it resolves cancels the Swift task. That is advisory, as
/// it is in Swift — the call stops at its next suspension point, and one with
/// none left runs to completion — but the result is discarded either way and
/// the allocation goes as soon as the task lets go of it.
#[cfg(all(feature = "async", feature = "ns"))]
pub struct CallFuture<O, T> {
    call: std::sync::Arc<AsyncCall<O, AwaitedCall<O, T>>>,
    task: SwiftTask,
}

// The header is touched only by the trampolines and by the completion, which
// run in sequence on the task, and everything the future reads is behind the
// mutex. `O` crosses to the task and `T` back, so both have to be `Send`.
#[cfg(all(feature = "async", feature = "ns"))]
unsafe impl<O: Send, T: Send> Send for CallFuture<O, T> {}
#[cfg(all(feature = "async", feature = "ns"))]
unsafe impl<O: Send, T: Send> Sync for CallFuture<O, T> {}

#[cfg(all(feature = "async", feature = "ns"))]
impl<O, T> Drop for CallFuture<O, T> {
    #[inline]
    fn drop(&mut self) {
        self.task.cancel();
    }
}

#[cfg(all(feature = "async", feature = "ns"))]
impl<O, T> std::future::Future for CallFuture<O, T> {
    type Output = Result<T, crate::arc::R<crate::ns::Error>>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut state = self.call.completion.state.lock();
        match state.ready.take() {
            Some(value) => std::task::Poll::Ready(value),
            None => {
                state.waker = Some(cx.waker().clone());
                std::task::Poll::Pending
            }
        }
    }
}

/// Finishes a call that is being awaited: converts what it produced, leaves it
/// in the slot, and wakes whoever is waiting.
///
/// # Safety
///
/// `task` must begin an `AsyncCall<O, AwaitedCall<O, T>>` held by an `Arc` that
/// the task owns a reference to.
#[cfg(all(feature = "async", feature = "ns"))]
unsafe fn complete_awaited<O, T>(task: *mut AsyncCallTask) {
    // The task's own reference, taken back so it is released at the end of this
    // whether or not a future is still holding the other one.
    let call = unsafe {
        std::sync::Arc::from_raw(task.cast_const().cast::<AsyncCall<O, AwaitedCall<O, T>>>())
    };

    // The completion is the only reader of `owned`, and runs once, so this is
    // the single move it ever sees. It is dropped here even when the call threw
    // and the conversion never runs.
    let owned = unsafe { core::ptr::read(&*call.owned) };
    let value = if call.header.error.is_null() {
        Ok((call.completion.output)(owned, call.header.result))
    } else {
        drop(owned);
        // Bridging hands back the error box itself, so the task's reference
        // becomes the `ns::Error`'s and is not released again.
        Err(unsafe { crate::arc::R::from_raw(abi::error_as_ns_error(call.header.error).cast()) })
    };

    let mut state = call.completion.state.lock();
    state.ready = Some(value);
    let waker = state.waker.take();
    drop(state);

    // Waking under the lock would just make the woken thread contend.
    if let Some(waker) = waker {
        waker.wake();
    }
}

/// Awaits `function` on a Swift task, yielding what it produced or the error it
/// threw, bridged to an [`ns::Error`](crate::ns::Error).
///
/// The future half of [`call_async_result`], and not built on it: going through
/// a completion handler would mean a second allocation for the shared state the
/// handler feeds, where this puts that state in the task's own.
///
/// # Safety
///
/// As [`call_async`].
#[cfg(all(feature = "async", feature = "ns"))]
pub(crate) unsafe fn call_async_future<O, T>(
    function: *const (),
    async_fn: *const u8,
    owned: O,
    args: impl FnOnce(&mut O) -> AsyncCallArgs,
    output: fn(O, *mut ()) -> T,
) -> CallFuture<O, T>
where
    O: Send + 'static,
    T: Send + 'static,
{
    let mut call = std::sync::Arc::new(AsyncCall {
        header: AsyncCallTask::new(function, async_fn, complete_awaited::<O, T>),
        owned: core::mem::ManuallyDrop::new(owned),
        completion: AwaitedCall {
            output,
            state: parking_lot::Mutex::new(AwaitedState {
                ready: None,
                waker: None,
            }),
        },
    });

    // Nothing else holds the allocation yet, so the arguments are built through
    // the one reference there is, into the memory Swift will read them from.
    {
        let call = std::sync::Arc::get_mut(&mut call).expect("the call is unshared here");
        call.header.args = args(&mut call.owned);
    }

    // The task gets a reference of its own, which its completion takes back.
    // `Arc::into_raw` addresses the value rather than the header block, so this
    // is the pointer the trampolines read their offsets from.
    let context = std::sync::Arc::into_raw(call.clone())
        .cast_mut()
        .cast::<()>();
    let task = unsafe {
        start_task(
            (&raw const CIDRE_SWIFT_ASYNC_CALL_TASK_DESCRIPTOR).cast(),
            context,
        )
    };

    CallFuture { call, task }
}

/// Calls `AsyncSequence.makeAsyncIterator()` through its protocol witness.
#[inline]
pub(crate) unsafe fn call_make_async_iterator(
    function: *const (),
    sequence: *mut (),
    sequence_metadata: *const TypeMetadata,
    witness: *const (),
    iterator: *mut (),
) {
    unsafe {
        abi::call::witness_value_to_value(function, sequence, sequence_metadata, witness, iterator)
    }
}

macro_rules! define_async_sequence_runner {
    () => {
/// Runtime entries needed to iterate one concrete Swift `AsyncSequence`.
///
/// Everything the per-element path needs is resolved once here, which is what
/// Swift's own `for await` does: it hoists the metadata and witness tables for
/// `Element` and `Element?` into the async context before the loop and never
/// consults the runtime again while iterating.
#[derive(Clone, Copy)]
pub(crate) struct AsyncSequenceSymbols {
    pub(crate) iterator_metadata: *const TypeMetadata,
    /// `Element`, for the optional's tag and for destroying a delivered value.
    element: abi::ValueWitnesses,
    /// `Element?`, whose metadata is a runtime generic lookup and so is worth
    /// resolving once per sequence rather than once per element.
    optional: abi::ValueWitnesses,
    /// Stride of `Element?`: the buffer each `next()` writes its result into.
    result_stride: usize,
    make_iterator: *const (),
    /// Whether the sequence is a class, taken from its Rust type rather than
    /// stated by the binding.
    sequence_is_class_ref: bool,
    pub(crate) next: *const (),
    pub(crate) next_async_fn: *const u8,
}

unsafe impl Send for AsyncSequenceSymbols {}

impl AsyncSequenceSymbols {
    /// # Safety
    ///
    /// Both metadata pointers must describe the sequence's iterator and element
    /// types, and the three symbols must be that iterator's `makeAsyncIterator`
    /// and `next` entries.
    #[allow(clippy::too_many_arguments)]
    pub(crate) unsafe fn new(
        iterator_metadata: *const TypeMetadata,
        element_metadata: *const TypeMetadata,
        make_iterator: *const (),
        sequence_is_class_ref: bool,
        next: *const (),
        next_async_fn: *const u8,
    ) -> Self {
        unsafe {
            let element = abi::ValueWitnesses::new(element_metadata);
            let optional = abi::ValueWitnesses::new(abi::optional_metadata(element_metadata));
            Self {
                iterator_metadata,
                element,
                optional,
                result_stride: optional.layout().stride.max(1),
                make_iterator,
                sequence_is_class_ref,
                next,
                next_async_fn,
            }
        }
    }

    /// Builds the sequence's iterator into `out`.
    ///
    /// # Safety
    ///
    /// `sequence` must point to storage holding an initialized value of this
    /// sequence's type, and `out` to uninitialized iterator storage.
    unsafe fn call_make_iterator(&self, sequence: *const (), out: *mut ()) {
        unsafe {
            // Swift passes a struct's storage address and only borrows it, but
            // passes a class as the reference itself and consumes it, so hand
            // over a reference of our own and leave the caller's intact.
            let this = if self.sequence_is_class_ref {
                abi::object_retain(sequence.cast::<*const ()>().read())
            } else {
                sequence
            };
            abi::call::value_to_value(self.make_iterator, this, out);
        }
    }
}

/// Drives a callback-based iteration over the same task the pulled iterator
/// uses.
///
/// A callback always wants the next element, so this never parks: `publish`
/// reports a request already in flight and the task resumes itself, which turns
/// the await into a fall-through. That is the only thing separating the two
/// shapes, so one set of trampolines serves both.
struct CallbackControl {
    callback: std::sync::Mutex<Box<dyn FnMut(Option<*const ()>) -> bool + Send>>,
    stop: std::sync::atomic::AtomicBool,
}

impl PullControl for CallbackControl {
    fn publish(&self, _continuation: *mut ()) -> bool {
        true
    }

    fn should_stop(&self) -> bool {
        self.stop.load(std::sync::atomic::Ordering::Relaxed)
    }

    fn deliver(&self, element: Option<*const ()>) -> bool {
        // A callback that panicked poisoned this, and iterating on with one
        // that cannot run would spin, so a poisoned lock winds the task up.
        let keep_going = match self.callback.lock() {
            Ok(mut callback) => callback(element),
            Err(_) => false,
        };
        if !keep_going {
            self.stop
                .store(true, std::sync::atomic::Ordering::Relaxed);
        }
        keep_going
    }
}

/// Iterates a concrete Swift `AsyncSequence` on a Swift task.
///
/// The callback returns whether iteration should continue after an element.
/// It is called once with `None` when the sequence finishes naturally.
pub(crate) fn iterate_async_sequence<F>(
    sequence: AnyValue,
    symbols: AsyncSequenceSymbols,
    callback: F,
) where
    F: FnMut(Option<*const ()>) -> bool + Send + 'static,
{
    start_iteration(
        sequence,
        symbols,
        std::sync::Arc::new(CallbackControl {
            callback: std::sync::Mutex::new(Box::new(callback)),
            stop: std::sync::atomic::AtomicBool::new(false),
        }),
    );
}

/// Builds the sequence's iterator and starts the task that will drive it.
fn start_iteration(
    sequence: AnyValue,
    symbols: AsyncSequenceSymbols,
    control: std::sync::Arc<dyn PullControl>,
) {
    unsafe {
        let mut storage = crate::swift::value::DynamicStorage::new(symbols.iterator_metadata);
        symbols.call_make_iterator(sequence.as_ptr(), storage.as_mut_ptr());

        let task = Box::new(PulledIterTask {
            iterator: storage.assume_init(),
            _sequence: sequence,
            symbols,
            result: None,
            control,
        });
        spawn_task(
            (&raw const CIDRE_SWIFT_PULLED_ITER_TASK_DESCRIPTOR).cast(),
            Box::into_raw(task).cast(),
        );
    }
}

/// The Swift task's side of a [`PulledIter`].
///
/// Kept alive by the task itself: the trampolines hand this pointer to every
/// Rust callback, and the last one drops it.
struct PulledIterTask {
    // Declaration order is drop order, and the iterator borrows the sequence.
    iterator: AnyValue,
    _sequence: AnyValue,
    symbols: AsyncSequenceSymbols,
    result: Option<NonNull<u8>>,
    control: std::sync::Arc<dyn PullControl>,
}

unsafe impl Send for PulledIterTask {}

/// The half of a pulled iterator the Swift task talks to, with the element type
/// erased so one set of trampolines serves every sequence.
pub(crate) trait PullControl: Send + Sync {
    /// Records the continuation the task is about to wait on.
    ///
    /// Returns whether a request is already in flight, in which case the task
    /// resumes itself instead of parking — the continuation's own
    /// synchronization makes that await fall straight through.
    fn publish(&self, continuation: *mut ()) -> bool;

    /// Whether the consumer has gone away and the task should wind up.
    fn should_stop(&self) -> bool;

    /// Hands over one element, or `None` at the end of the sequence. Returns
    /// whether the task should wait for another request.
    fn deliver(&self, element: Option<*const ()>) -> bool;
}

/// A Rust async iterator over a Swift `AsyncSequence`, driven by one Swift task
/// that parks on a continuation between elements.
///
/// This is the shape Swift's own `for await` compiles to: a single task that
/// suspends in place, rather than one task per element. Steady state costs no
/// allocation and no task creation — a `next()` is a continuation resume, and
/// an element is a wake.
#[cfg(feature = "async")]
pub struct PulledIter<T> {
    shared: std::sync::Arc<PullShared<T>>,
}

#[cfg(feature = "async")]
struct PullShared<T> {
    state: parking_lot::Mutex<PullState<T>>,
    /// Copies a borrowed Swift element into its Rust representation.
    copy: unsafe fn(*const ()) -> T,
}

#[cfg(feature = "async")]
struct PullState<T> {
    /// Set only while the Swift task is parked and nobody has resumed it yet;
    /// whoever takes it owns the single resume that continuation allows.
    continuation: Option<*mut ()>,
    /// Whether the task has been asked for an element it has not delivered
    /// yet. At most one request is ever in flight, so the task fetches exactly
    /// one element per [`PullNext`] that asks for one, and `delivered` can
    /// never be overwritten.
    ///
    /// While it is set the task is running rather than parked, so it also
    /// means `continuation` is empty.
    outstanding: bool,
    delivered: Option<Option<T>>,
    waker: Option<std::task::Waker>,
    ended: bool,
    stop: bool,
}

// The continuation handle is a Swift task reference, which the runtime allows
// to be resumed from any thread.
#[cfg(feature = "async")]
unsafe impl<T: Send> Send for PullShared<T> {}
#[cfg(feature = "async")]
unsafe impl<T: Send> Sync for PullShared<T> {}

#[cfg(feature = "async")]
impl<T> PullState<T> {
    /// Asks the Swift task for one more element, unless it is already fetching
    /// one, and returns the continuation that has to be resumed to wake it.
    ///
    /// The caller resumes it after dropping the lock: a resume can run the task
    /// synchronously on this thread, and it re-enters this state as soon as it
    /// has an element.
    fn request(&mut self) -> Option<*mut ()> {
        if self.ended || self.stop || self.outstanding {
            return None;
        }
        self.outstanding = true;
        // Empty whenever the task is mid-fetch, so this parks nothing.
        self.continuation.take()
    }
}

#[cfg(feature = "async")]
impl<T: Send> PullControl for PullShared<T> {
    fn publish(&self, continuation: *mut ()) -> bool {
        let mut state = self.state.lock();
        if state.outstanding || state.stop {
            // Resumed by the task itself, so it must not also be parked here.
            return true;
        }
        state.continuation = Some(continuation);
        false
    }

    fn should_stop(&self) -> bool {
        self.state.lock().stop
    }

    fn deliver(&self, element: Option<*const ()>) -> bool {
        let value = element.map(|element| unsafe { (self.copy)(element) });
        let ended = value.is_none();

        let mut state = self.state.lock();
        state.ended |= ended;
        // The one request this answers is what let the task fetch at all, so
        // there is never an element here waiting to be overwritten.
        debug_assert!(state.delivered.is_none());
        state.outstanding = false;
        state.delivered = Some(value);
        let waker = state.waker.take();
        let keep_going = !ended && !state.stop;
        drop(state);

        // Waking under the lock would just make the woken thread contend.
        if let Some(waker) = waker {
            waker.wake();
        }
        keep_going
    }
}

#[cfg(feature = "async")]
impl<T: Send + 'static> PulledIter<T> {
    /// Awaits the next element, or `None` once the sequence has ended.
    ///
    /// Nothing is asked of the Swift task until the future is polled, so a
    /// `next()` that is dropped before then — the losing branch of a `select!`,
    /// a timeout — costs nothing and consumes no element.
    pub fn next(&mut self) -> PullNext<'_, T> {
        PullNext {
            shared: &self.shared,
        }
    }
}

#[cfg(feature = "async")]
impl<T> Drop for PulledIter<T> {
    fn drop(&mut self) {
        // Tell the task to wind up, and wake it if it is parked, so it finishes
        // and frees itself rather than waiting for a request that never comes.
        let mut state = self.shared.state.lock();
        state.stop = true;
        let continuation = state.continuation.take();
        drop(state);

        if let Some(continuation) = continuation {
            unsafe { swift_continuation_resume(continuation) };
        }
    }
}

/// The future returned by [`PulledIter::next`].
///
/// Asking the task for an element is the first poll's job rather than
/// `next()`'s, so at most one request is ever in flight even if a future is
/// dropped between the request and the element that answers it: that element
/// stays in `delivered` for whoever polls next.
#[cfg(feature = "async")]
pub struct PullNext<'a, T> {
    shared: &'a std::sync::Arc<PullShared<T>>,
}

#[cfg(feature = "async")]
impl<T> std::future::Future for PullNext<'_, T> {
    type Output = Option<T>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut state = self.shared.state.lock();

        if let Some(value) = state.delivered.take() {
            return std::task::Poll::Ready(value);
        }
        if state.ended {
            return std::task::Poll::Ready(None);
        }

        if !state
            .waker
            .as_ref()
            .is_some_and(|waker| waker.will_wake(cx.waker()))
        {
            state.waker = Some(cx.waker().clone());
        }

        // Registered first, so an element delivered from the task's thread
        // between the request and the end of this poll still wakes us.
        let resume = state.request();
        drop(state);

        if let Some(continuation) = resume {
            unsafe { swift_continuation_resume(continuation) };
        }
        std::task::Poll::Pending
    }
}

/// Starts the task that will drive `sequence`, and returns the Rust half.
#[cfg(feature = "async")]
pub(crate) fn pulled_async_iter<T>(
    sequence: AnyValue,
    symbols: AsyncSequenceSymbols,
    copy: unsafe fn(*const ()) -> T,
) -> PulledIter<T>
where
    T: Send + 'static,
{
    let shared = std::sync::Arc::new(PullShared {
        state: parking_lot::Mutex::new(PullState {
            continuation: None,
            outstanding: false,
            delivered: None,
            waker: None,
            ended: false,
            stop: false,
        }),
        copy,
    });

    start_iteration(sequence, symbols, shared.clone());
    PulledIter { shared }
}

// A pulled iterator's task context, beyond the `AsyncContext` header:
//
// ```text
//  16  this context, so the trampolines can find it again
//  24  the Rust `PulledIterTask`
//  32  the Swift iterator
//  40  the `Element?` buffer every `next()` writes into
//  48  the callee context of the `next()` currently in flight
//  56  scratch, only so the continuation has somewhere to point
//  64  the handle that resumes the parked continuation
//  80  the embedded `ContinuationAsyncContext`
// ```
swift_async_task_descriptor!(
    CIDRE_SWIFT_PULLED_ITER_TASK_DESCRIPTOR,
    entry: pulled_iter_task_entry,
    context_size: "192",
);

/// Allocates the element buffer once for the whole iteration, then waits.
#[unsafe(naked)]
unsafe extern "C" fn pulled_iter_task_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "bl {result_size}",
        "bl {task_alloc}",
        "mov x1, x0",
        "str x1, [x22, #40]",
        "ldr x0, [x22, #24]",
        "bl {set_result}",
        "ldr x0, [x22, #24]",
        "bl {iterator}",
        "str x0, [x22, #32]",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {wait}",
        result_size = sym cidre_pulled_iter_result_size,
        set_result = sym cidre_pulled_iter_set_result,
        iterator = sym cidre_pulled_iter_iterator,
        task_alloc = sym swift_task_alloc,
        wait = sym pulled_iter_task_wait,
    );
}

/// Arms the continuation, publishes it, and parks until Rust asks for an
/// element.
///
/// If a request already arrived the task resumes itself before awaiting: the
/// continuation's own synchronization turns that into a fall-through rather
/// than a suspension, which is the whole reason this cannot race.
#[unsafe(naked)]
unsafe extern "C" fn pulled_iter_task_wait() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "add x9, x22, #80",
        // The result slot is never read; the continuation just needs one.
        "add x8, x22, #56",
        "str x8, [x22, #120]",
        swift_async_store_parent!(),
        swift_async_store_resume!("{requested}"),
        "mov x0, x9",
        "mov x1, #0",
        "bl {continuation_init}",
        "str x0, [x22, #64]",
        "ldr x0, [x22, #24]",
        "ldr x1, [x22, #64]",
        "bl {publish}",
        "cbz w0, 1f",
        "ldr x0, [x22, #64]",
        "bl {continuation_resume}",
        "1:",
        "add x0, x22, #80",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {continuation_await}",
        continuation_init = sym swift_continuation_init,
        continuation_resume = sym swift_continuation_resume,
        continuation_await = sym swift_continuation_await,
        publish = sym cidre_pulled_iter_publish,
        requested = sym pulled_iter_task_requested,
    );
}

/// Resumed once Rust wants an element, with the continuation context in `x22`.
#[unsafe(naked)]
unsafe extern "C" fn pulled_iter_task_requested() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        swift_async_load_parent!(),
        "mov x22, x9",
        "ldr x0, [x22, #24]",
        "bl {should_stop}",
        "cbnz w0, 2f",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {fetch}",
        "2:",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {finish}",
        should_stop = sym cidre_pulled_iter_should_stop,
        fetch = sym pulled_iter_task_fetch,
        finish = sym pulled_iter_task_finish,
    );
}

/// Tail-calls the iterator's `next()`.
#[unsafe(naked)]
unsafe extern "C" fn pulled_iter_task_fetch() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {next_async_fn}",
        "ldr w0, [x0, #4]",
        "bl {task_alloc}",
        "mov x9, x0",
        "str x9, [x22, #48]",
        swift_async_store_parent!(),
        swift_async_store_resume!("{resume}"),
        "ldr x0, [x22, #24]",
        "bl {next_fn}",
        "mov x16, x0",
        "ldr x0, [x22, #40]",
        "ldr x20, [x22, #32]",
        "mov x22, x9",
        "mov x21, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        next_async_fn = sym cidre_pulled_iter_next_async_fn,
        next_fn = sym cidre_pulled_iter_next_fn,
        task_alloc = sym swift_task_alloc,
        resume = sym pulled_iter_task_resumed,
    );
}

/// Resumed with the element written into the result buffer.
#[unsafe(naked)]
unsafe extern "C" fn pulled_iter_task_resumed() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        swift_async_load_parent!(),
        "str x9, [sp]",
        "ldr x0, [x9, #48]",
        "bl {task_dealloc}",
        "ldr x22, [sp]",
        swift_async_function_pointer!("{deliver}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {task_switch}",
        task_dealloc = sym swift_task_dealloc,
        task_switch = sym swift_task_switch,
        deliver = sym pulled_iter_task_deliver,
    );
}

/// Hands the element to Rust, then waits for the next request or winds up.
#[unsafe(naked)]
unsafe extern "C" fn pulled_iter_task_deliver() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {process}",
        "ldr x22, [sp, #8]",
        "cbz w0, 3f",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {wait}",
        "3:",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {finish}",
        process = sym cidre_pulled_iter_process,
        wait = sym pulled_iter_task_wait,
        finish = sym pulled_iter_task_finish,
    );
}

/// Frees the element buffer and the Rust task, then returns to whoever created
/// the task.
#[unsafe(naked)]
unsafe extern "C" fn pulled_iter_task_finish() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {take_result}",
        "bl {task_dealloc}",
        "ldr x22, [sp, #8]",
        "ldr x0, [x22, #24]",
        "bl {drop_task}",
        "ldr x22, [sp, #8]",
        "ldr x9, [x22, #16]",
        swift_async_load_resume!(),
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        take_result = sym cidre_pulled_iter_take_result,
        drop_task = sym cidre_pulled_iter_drop,
        task_dealloc = sym swift_task_dealloc,
    );
}

extern "C" fn cidre_pulled_iter_result_size(task: *mut PulledIterTask) -> usize {
    unsafe { (*task).symbols.result_stride }
}

extern "C" fn cidre_pulled_iter_set_result(task: *mut PulledIterTask, result: *mut u8) {
    unsafe {
        let result = NonNull::new(result).expect("Swift task result allocation failed");
        assert!((*task).result.replace(result).is_none());
    }
}

extern "C" fn cidre_pulled_iter_iterator(task: *mut PulledIterTask) -> *mut () {
    unsafe { (*task).iterator.as_mut_ptr() }
}

extern "C" fn cidre_pulled_iter_publish(task: *mut PulledIterTask, continuation: *mut ()) -> bool {
    // Parking on a panic would strand the task forever, so fall through to a
    // fetch instead and let `process` wind the iteration up.
    catch_unwind(AssertUnwindSafe(|| unsafe {
        (*task).control.publish(continuation)
    }))
    .unwrap_or(true)
}

extern "C" fn cidre_pulled_iter_should_stop(task: *mut PulledIterTask) -> bool {
    catch_unwind(AssertUnwindSafe(|| unsafe { (*task).control.should_stop() })).unwrap_or(true)
}

extern "C" fn cidre_pulled_iter_next_async_fn(task: *const PulledIterTask) -> *const u8 {
    unsafe { (*task).symbols.next_async_fn }
}

extern "C" fn cidre_pulled_iter_next_fn(task: *const PulledIterTask) -> *const () {
    unsafe { (*task).symbols.next }
}

extern "C" fn cidre_pulled_iter_process(task: *mut PulledIterTask) -> bool {
    catch_unwind(AssertUnwindSafe(|| unsafe {
        let task = &mut *task;
        let result = task.result.expect("Swift task result missing");
        let symbols = task.symbols;
        if symbols.element.enum_tag_single_payload(result.as_ptr().cast(), 1) == 1 {
            symbols.optional.destroy(result.as_ptr().cast());
            task.control.deliver(None);
            return false;
        }

        let keep_going = task
            .control
            .deliver(Some(result.as_ptr().cast_const().cast()));
        symbols.element.destroy(result.as_ptr().cast());
        keep_going
    }))
    .unwrap_or(false)
}

extern "C" fn cidre_pulled_iter_take_result(task: *mut PulledIterTask) -> *mut () {
    unsafe {
        (*task)
            .result
            .take()
            .expect("Swift task result missing")
            .as_ptr()
            .cast()
    }
}

extern "C" fn cidre_pulled_iter_drop(task: *mut PulledIterTask) {
    unsafe { drop(Box::from_raw(task)) }
}
    };
}

/// Signs the return address, and enables the `pauth` mnemonics for the rest of
/// the fragments in this function.
#[cfg(target_feature = "paca")]
macro_rules! swift_async_pauth_prologue {
    () => {
        concat!(".arch_extension pauth\n", "pacibsp\n")
    };
}

#[cfg(not(target_feature = "paca"))]
macro_rules! swift_async_pauth_prologue {
    () => {
        ""
    };
}

#[cfg(target_feature = "paca")]
macro_rules! swift_async_pauth_epilogue {
    () => {
        "\nautibsp"
    };
}

#[cfg(not(target_feature = "paca"))]
macro_rules! swift_async_pauth_epilogue {
    () => {
        ""
    };
}

/// Opens an async frame: sets the frame pointer's async marker bit, saves the
/// frame record and the incoming async context, and establishes `x29`.
///
/// `frame` is the frame size, `fp` the offset of the saved frame record, and
/// `ctx` the offset of the saved async context.
macro_rules! swift_async_prologue {
    (frame: $frame:literal, fp: $fp:literal, ctx: $ctx:literal) => {
        concat!(
            $crate::swift::concurrency::swift_async_pauth_prologue!(),
            "orr x29, x29, #0x1000000000000000\n",
            "sub sp, sp, #",
            $frame,
            "\n",
            "stp x29, x30, [sp, #",
            $fp,
            "]\n",
            "str x22, [sp, #",
            $ctx,
            "]\n",
            "add x29, sp, #",
            $fp,
        )
    };
}

/// Closes an async frame, leaving the tail call to the caller.
macro_rules! swift_async_epilogue {
    (frame: $frame:literal, fp: $fp:literal) => {
        concat!(
            "ldp x29, x30, [sp, #",
            $fp,
            "]\n",
            "and x29, x29, #0xefffffffffffffff\n",
            "add sp, sp, #",
            $frame,
            $crate::swift::concurrency::swift_async_pauth_epilogue!(),
        )
    };
}

/// Stores this frame's async context (`x22`) as the parent of a freshly
/// allocated callee context (`x9`).
#[cfg(not(target_feature = "paca"))]
macro_rules! swift_async_store_parent {
    () => {
        "str x22, [x9]"
    };
}

#[cfg(target_feature = "paca")]
macro_rules! swift_async_store_parent {
    () => {
        concat!(
            "mov x8, x22\n",
            "mov x16, x9\n",
            "movk x16, #48546, lsl #48\n",
            "pacda x8, x16\n",
            "str x8, [x9]",
        )
    };
}

/// Loads the parent async context out of this frame's context (`x22`) into
/// `x9`.
#[cfg(not(target_feature = "paca"))]
macro_rules! swift_async_load_parent {
    () => {
        "ldr x9, [x22]"
    };
}

#[cfg(target_feature = "paca")]
macro_rules! swift_async_load_parent {
    () => {
        concat!(
            "ldr x16, [x22]\n",
            "mov x17, x22\n",
            "movk x17, #48546, lsl #48\n",
            "autda x16, x17\n",
            "mov x9, x16",
        )
    };
}

/// Stores `resume` as the resume function of a freshly allocated callee context
/// (`x9`).
#[cfg(not(target_feature = "paca"))]
macro_rules! swift_async_store_resume {
    ($resume:literal) => {
        concat!(
            "adrp x8, ",
            $resume,
            "@PAGE\n",
            "add x8, x8, ",
            $resume,
            "@PAGEOFF\n",
            "str x8, [x9, #8]",
        )
    };
}

#[cfg(target_feature = "paca")]
macro_rules! swift_async_store_resume {
    ($resume:literal) => {
        concat!(
            "add x8, x9, #8\n",
            "adrp x16, ",
            $resume,
            "@PAGE\n",
            "add x16, x16, ",
            $resume,
            "@PAGEOFF\n",
            "mov x17, x8\n",
            "movk x17, #55047, lsl #48\n",
            "pacia x16, x17\n",
            "str x16, [x9, #8]",
        )
    };
}

/// Loads the resume function of the parent context (`x9`) into `x16`, ready to
/// be branched to.
#[cfg(not(target_feature = "paca"))]
macro_rules! swift_async_load_resume {
    () => {
        "ldr x16, [x9, #8]"
    };
}

#[cfg(target_feature = "paca")]
macro_rules! swift_async_load_resume {
    () => {
        concat!(
            "ldr x16, [x9, #8]\n",
            "add x17, x9, #8\n",
            "movk x17, #55047, lsl #48\n",
            "autia x16, x17",
        )
    };
}

/// Materializes `target` in `x0` as a function pointer `swift_task_switch` can
/// resume into.
#[cfg(not(target_feature = "paca"))]
macro_rules! swift_async_function_pointer {
    ($target:literal) => {
        concat!(
            "adrp x0, ",
            $target,
            "@PAGE\n",
            "add x0, x0, ",
            $target,
            "@PAGEOFF",
        )
    };
}

#[cfg(target_feature = "paca")]
macro_rules! swift_async_function_pointer {
    ($target:literal) => {
        concat!(
            "adrp x16, ",
            $target,
            "@PAGE\n",
            "add x16, x16, ",
            $target,
            "@PAGEOFF\n",
            "paciza x16\n",
            "mov x0, x16",
        )
    };
}

/// Defines a Swift `AsyncFunctionPointer` for a trampoline entry point, plus
/// the `extern` declaration that lets Rust hand it to `swift_task_create`.
///
/// The record is a relative pointer to the entry symbol followed by the context
/// size, and a 32-bit displacement between two symbols can only be computed by
/// the assembler, so this is the one piece that cannot move into Rust.
macro_rules! swift_async_task_descriptor {
    (
        $(#[$meta:meta])*
        $name:ident, entry: $entry:path, context_size: $size:literal $(,)?
    ) => {
        $(#[$meta])*
        core::arch::global_asm!(
            ".section __TEXT,__const",
            concat!(".globl _", stringify!($name)),
            concat!(".private_extern _", stringify!($name)),
            ".p2align 3",
            concat!("_", stringify!($name), ":"),
            concat!(".long {entry} - _", stringify!($name)),
            concat!(".long ", $size),
            entry = sym $entry,
        );

        $(#[$meta])*
        unsafe extern "C" {
            static $name: u8;
        }
    };
}

/// Emits the mangled type of `some AsyncSequence`'s iterator, so
/// `swift_getTypeByMangledNameInContext2` can resolve it at runtime.
///
/// The record holds a relative reference to the opaque type descriptor's GOT
/// entry, which again only the assembler can compute. The bytes are a mangled
/// name: an opaque type reference, `y_Qo_` to select its first underlying type,
/// then `13AsyncIteratorSciQx` for that type's `AsyncIterator` associated type.
macro_rules! swift_opaque_iterator_typeref {
    (
        $start:ident ..= $end:ident, descriptor: $descriptor:path $(,)?
    ) => {
        core::arch::global_asm!(
            ".section __TEXT,__swift5_typeref",
            concat!(".globl _", stringify!($start)),
            concat!(".private_extern _", stringify!($start)),
            concat!("_", stringify!($start), ":"),
            ".byte 2",
            concat!("L", stringify!($start), ":"),
            concat!(".long {descriptor}@GOT-L", stringify!($start)),
            ".ascii \"y_Qo_13AsyncIteratorSciQx\"",
            concat!(".globl _", stringify!($end)),
            concat!(".private_extern _", stringify!($end)),
            concat!("_", stringify!($end), ":"),
            descriptor = sym $descriptor,
        );

        unsafe extern "C" {
            static $start: u8;
            static $end: u8;
        }
    };
}

/// Binds one concrete Swift `AsyncSequence` whose iterator and element types
/// both have metadata accessors.
///
/// `element` is a [`FromSwift`](crate::swift::FromSwift) type, which names the
/// Swift type it reads from, so there is no separate marker to keep in step
/// with it.
///
/// A struct-typed sequence mints a marker for its own metadata; a class-typed
/// one is written `class C` and uses `arc::R<C>`, which already names the type
/// and carries its ABI shape.
macro_rules! define_async_sequence {
    // A struct-typed sequence, which needs a marker minted for its metadata.
    (
        $(#[$meta:meta])*
        $sequence:ident, $sequence_value:ident, $iterator_value:ident,
        framework = $framework:literal,
        element = $element:ty,
        sequence_metadata = $sequence_metadata:ident => $sequence_metadata_link:literal,
        iterator_metadata = $iterator_metadata:ident => $iterator_metadata_link:literal,
        make_iterator = $make_iterator:ident => $make_iterator_link:literal,
        next = $next:ident => $next_link:literal,
        next_async = $next_async:ident => $next_async_link:literal,
        async_iter = $async_iter:ident $(,)?
    ) => {
        #[link(name = $framework, kind = "framework")]
        unsafe extern "C" {
            #[link_name = $sequence_metadata_link]
            fn $sequence_metadata();
        }

        $crate::define_swift_marker!(pub(crate) $sequence_value = accessor $sequence_metadata);

        $crate::swift::concurrency::define_async_sequence! {
            @build
            $(#[$meta])*
            $sequence, $sequence_value, $iterator_value,
            framework = $framework,
            element = $element,
            iterator_metadata = $iterator_metadata => $iterator_metadata_link,
            make_iterator = $make_iterator => $make_iterator_link,
            next = $next => $next_link,
            next_async = $next_async => $next_async_link,
            async_iter = $async_iter,
        }
    };
    // A class-typed sequence: its value is the class reference, so
    // `arc::R<Class>` already carries the metadata and the ABI shape.
    (
        $(#[$meta:meta])*
        $sequence:ident, class $class:ty, $iterator_value:ident,
        framework = $framework:literal,
        element = $element:ty,
        iterator_metadata = $iterator_metadata:ident => $iterator_metadata_link:literal,
        make_iterator = $make_iterator:ident => $make_iterator_link:literal,
        next = $next:ident => $next_link:literal,
        next_async = $next_async:ident => $next_async_link:literal,
        async_iter = $async_iter:ident $(,)?
    ) => {
        $crate::swift::concurrency::define_async_sequence! {
            @build
            $(#[$meta])*
            $sequence, $crate::arc::R<$class>, $iterator_value,
            framework = $framework,
            element = $element,
            iterator_metadata = $iterator_metadata => $iterator_metadata_link,
            make_iterator = $make_iterator => $make_iterator_link,
            next = $next => $next_link,
            next_async = $next_async => $next_async_link,
            async_iter = $async_iter,
        }
    };
    (
        @build
        $(#[$meta:meta])*
        $sequence:ident, $sequence_value:ty, $iterator_value:ident,
        framework = $framework:literal,
        element = $element:ty,
        iterator_metadata = $iterator_metadata:ident => $iterator_metadata_link:literal,
        make_iterator = $make_iterator:ident => $make_iterator_link:literal,
        next = $next:ident => $next_link:literal,
        next_async = $next_async:ident => $next_async_link:literal,
        async_iter = $async_iter:ident $(,)?
    ) => {
        $(#[$meta])*
        pub struct $sequence($crate::swift::value::AnyValue);

        unsafe impl $crate::swift::SwiftAbi for $sequence {
            const CLASS: $crate::swift::AbiClass = $crate::swift::AbiClass::Indirect;
        }

        /// A sequence is made once and then iterated, so it keeps its value in
        /// storage the runtime sizes rather than declaring a layout of its own.
        impl $crate::swift::value::SwiftOut for $sequence {
            type Buf = $crate::swift::value::DynamicStorage;

            #[inline]
            fn out_buf() -> Self::Buf {
                unsafe {
                    $crate::swift::value::DynamicStorage::new(
                        <$sequence_value as $crate::swift::SwiftMetadata>::metadata(),
                    )
                }
            }

            #[inline]
            fn out_ptr(buf: &mut Self::Buf) -> *mut () {
                buf.as_mut_ptr()
            }

            #[inline]
            unsafe fn out_take(buf: Self::Buf) -> Self {
                Self(unsafe { buf.assume_init() })
            }
        }

        #[link(name = $framework, kind = "framework")]
        unsafe extern "C" {
            #[link_name = $iterator_metadata_link]
            fn $iterator_metadata();

            #[link_name = $make_iterator_link]
            fn $make_iterator();

            #[link_name = $next_link]
            fn $next();

            #[link_name = $next_async_link]
            static $next_async: u8;
        }

        $crate::define_swift_marker!($iterator_value = accessor $iterator_metadata);

        impl $sequence {
            #[allow(dead_code)]
            pub(crate) unsafe fn from_storage(
                storage: $crate::swift::value::DynamicStorage,
            ) -> Self {
                Self(unsafe { storage.assume_init() })
            }

            pub fn for_each_while<F>(self, mut callback: F)
            where
                F: FnMut(Option<$element>) -> bool + Send + 'static,
            {
                $crate::swift::concurrency::iterate_async_sequence(
                    self.0,
                    Self::symbols(),
                    move |value| match value {
                        Some(value) => callback(Some(unsafe { <$element as $crate::swift::FromSwift>::copy_swift(value) })),
                        None => {
                            callback(None);
                            false
                        }
                    },
                );
            }

            pub fn for_each<F>(self, mut callback: F)
            where
                F: FnMut(Option<$element>) + Send + 'static,
            {
                self.for_each_while(move |value| {
                    let has_value = value.is_some();
                    callback(value);
                    has_value
                });
            }

            /// Compatibility spelling for [`Self::for_each`].
            pub fn next<F>(self, mut callback: F)
            where
                F: FnMut(Option<$element>) + Send + 'static,
            {
                self.for_each(move |value| callback(value));
            }

            fn symbols() -> $crate::swift::concurrency::AsyncSequenceSymbols {
                unsafe {
                    $crate::swift::concurrency::AsyncSequenceSymbols::new(
                        <$iterator_value as $crate::swift::SwiftMetadata>::metadata(),
                        <$element as $crate::swift::SwiftMetadata>::metadata(),
                        $make_iterator as *const (),
                        <$sequence_value as $crate::swift::SwiftMetadata>::IS_CLASS_REF,
                        $next as *const (),
                        (&raw const $next_async).cast(),
                    )
                }
            }

            /// Turns the sequence into a Rust async iterator that pulls one
            /// element per `next().await`, mirroring Swift's
            /// `AsyncIteratorProtocol`.
            #[cfg(feature = "async")]
            pub fn async_iter(self) -> $async_iter {
                unsafe fn copy(value: *const ()) -> $element {
                    unsafe { <$element as $crate::swift::FromSwift>::copy_swift(value) }
                }

                $async_iter {
                    pulled: $crate::swift::concurrency::pulled_async_iter(
                        self.0,
                        Self::symbols(),
                        copy,
                    ),
                }
            }
        }

        #[doc = concat!("A Rust `AsyncIteratorProtocol` over `", stringify!($sequence), "`.")]
        ///
        /// Each `next` drives exactly one Swift `next()`, so elements are
        /// pulled on demand rather than produced eagerly and queued. One Swift
        /// task serves the whole iteration, parked on a continuation between
        /// elements, so advancing costs no allocation.
        #[cfg(feature = "async")]
        pub struct $async_iter {
            pulled: $crate::swift::concurrency::PulledIter<$element>,
        }

        #[cfg(feature = "async")]
        impl $async_iter {
            /// Awaits the next element, or `None` once the sequence has ended.
            ///
            /// Cancel-safe: dropping the future without awaiting it to
            /// completion loses no element, so it can be the losing branch of a
            /// `select!` or a timeout. An element the dropped future had
            /// already asked for is handed to the next caller instead.
            pub fn next(
                &mut self,
            ) -> impl ::core::future::Future<Output = Option<$element>> + use<'_> {
                self.pulled.next()
            }
        }
    };
}

pub(crate) use {
    define_async_sequence, swift_async_epilogue, swift_async_function_pointer,
    swift_async_load_parent, swift_async_load_resume, swift_async_pauth_epilogue,
    swift_async_pauth_prologue, swift_async_prologue, swift_async_store_parent,
    swift_async_store_resume, swift_async_task_descriptor, swift_opaque_iterator_typeref,
};

define_async_sequence_runner!();

/// One suspending Swift call, driven on a Swift task.
///
/// Every binding that awaits a Swift function used to hand-write the same three
/// trampolines, differing only in which registers its arguments went in and
/// which symbol was tail-called. Both are data, so they live in this
/// fixed-layout struct instead: the trampolines below read them by offset and
/// serve every such call, which is why there is exactly one copy of the
/// assembly and one task descriptor.
///
/// The layout is the ABI those trampolines are written against, and they reach
/// every field of it through `offset_of!`, so moving one moves the assembly
/// with it rather than silently miscompiling.
#[repr(C, align(16))]
struct AsyncCallTask {
    /// The Swift function to tail-call.
    function: *const (),
    /// Its async function pointer, whose second word is the context size the
    /// callee needs.
    async_fn: *const u8,
    /// Where the resume trampoline puts what the call produced.
    result: *mut (),
    /// The error box Swift threw, or null.
    error: *mut (),
    /// The registers the call goes out in, which the entry trampoline loads
    /// straight out of here.
    args: AsyncCallArgs,
    /// Runs the completion and frees the allocation this header begins.
    ///
    /// A monomorphized function rather than a trait object: the completion and
    /// what the call borrowed live in that same allocation, so there is nothing
    /// to point at, and the one word here replaces both a second allocation and
    /// the vtable half of a fat pointer. It is also what remembers how the
    /// allocation was made, and so how to free it.
    complete: unsafe fn(*mut AsyncCallTask),
}

/// The step from one argument slot to the next, which is what the entry
/// trampoline's load-pairs advance by.
const ARG_STRIDE: usize = core::mem::size_of::<*mut ()>();

/// The same for the vector registers, which are twice as wide.
const VECTOR_STRIDE: usize = core::mem::size_of::<[u64; 2]>();

// A load-pair addresses its operand as a multiple of the access width, so each
// block has to begin on one: eight bytes for a pair of words, sixteen for a
// pair of vectors.
const _: () = assert!(core::mem::offset_of!(AsyncCallTask, args.args) % ARG_STRIDE == 0);
const _: () = assert!(core::mem::offset_of!(AsyncCallTask, args.vectors) % VECTOR_STRIDE == 0);

// The resume trampoline writes what the call produced as one pair, so the two
// have to sit next to each other in that order.
const _: () = assert!(
    core::mem::offset_of!(AsyncCallTask, error)
        == core::mem::offset_of!(AsyncCallTask, result) + ARG_STRIDE
);

impl AsyncCallTask {
    /// The header of a call that has not gone out yet: the arguments are filled
    /// in afterwards, from the allocation this ends up in.
    #[inline]
    fn new(
        function: *const (),
        async_fn: *const u8,
        complete: unsafe fn(*mut AsyncCallTask),
    ) -> Self {
        Self {
            function,
            async_fn,
            result: core::ptr::null_mut(),
            error: core::ptr::null_mut(),
            args: AsyncCallArgs::new(),
            complete,
        }
    }
}

/// A task and everything belonging to it, in one allocation.
///
/// The header is first and `repr(C)`, so the trampolines find the fields they
/// read by the offsets they are written against whatever the tail holds.
///
/// Who frees it is the tail's business: a call with a completion handler is a
/// `Box`, since the task is the only owner, while one that is awaited is an
/// `Arc`, since the future holds the same allocation and either may outlive the
/// other. Both hand the trampolines the same pointer.
#[repr(C)]
struct AsyncCall<O, C> {
    header: AsyncCallTask,
    /// Everything the call borrowed, kept alive until it finishes. Pointers
    /// into it are taken after this is allocated and stay valid for the call.
    ///
    /// Taken out by the completion and by nothing else, so it is not dropped
    /// when the allocation is.
    owned: core::mem::ManuallyDrop<O>,
    completion: C,
}

/// Finishes a call whose completion is a handler the caller supplied.
///
/// # Safety
///
/// `task` must begin a boxed `AsyncCall<O, F>` that nothing has taken yet.
unsafe fn complete_boxed<O, F>(task: *mut AsyncCallTask)
where
    F: FnOnce(O, *mut (), *mut ()),
{
    let call = *unsafe { Box::from_raw(task.cast::<AsyncCall<O, F>>()) };
    let owned = core::mem::ManuallyDrop::into_inner(call.owned);
    (call.completion)(owned, call.header.result, call.header.error);
}

/// What the arguments of one call are, in the registers Swift passes them in.
///
/// Every register is loaded whether or not the callee reads it: a Swift
/// function ignores the argument registers past its own arity, so one
/// trampoline can serve calls of different shapes without branching on which.
///
/// This is the register block of an [`AsyncCallTask`] rather than a description
/// of one that has to be copied into it, so the layout the trampolines read is
/// written down once.
#[derive(Default)]
#[repr(C, align(16))]
pub(crate) struct AsyncCallArgs {
    /// `swiftself`: the instance for a method, the metadata for a static.
    swift_self: *mut (),
    /// `x0`–`x3`.
    args: [*mut (); 4],
    /// Keeps the vector registers at an offset a 128-bit load can reach.
    _pad: usize,
    /// `v0`–`v3`, loaded whole: a `Double` argument is the low half, and a
    /// two-`Double` vector — half of a quaternion, say — is both.
    vectors: [[u64; 2]; 4],
}

impl AsyncCallArgs {
    #[inline]
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// The instance a method is called on, or a static method's metadata.
    #[inline]
    pub(crate) fn swift_self(mut self, value: *mut ()) -> Self {
        self.swift_self = value;
        self
    }

    /// One integer, pointer, or reference argument, in declaration order.
    #[inline]
    pub(crate) fn arg(mut self, index: usize, value: *mut ()) -> Self {
        self.args[index] = value;
        self
    }

    /// One floating-point argument, counted separately, as the ABI does.
    #[inline]
    pub(crate) fn float(mut self, index: usize, value: f64) -> Self {
        self.vectors[index][0] = value.to_bits();
        self
    }

    /// One two-`Double` vector argument, which is how Swift passes the halves
    /// of a quaternion and the other short vector types.
    #[inline]
    pub(crate) fn vector2(mut self, index: usize, value: [f64; 2]) -> Self {
        self.vectors[index] = [value[0].to_bits(), value[1].to_bits()];
        self
    }
}

/// Awaits `function` on a Swift task, then runs `completion` with the register
/// the call returned in and the error box it threw.
///
/// `owned` is whatever the call needs kept alive — retained objects, argument
/// storage, the buffer an indirect result is written into. It lives in the
/// task's own allocation, which is made before `args` runs, so a pointer taken
/// into it stays valid for the whole call.
///
/// # Safety
///
/// `function` must be the entry point of a suspending Swift function and
/// `async_fn` its async function pointer, and the arguments `args` builds must
/// be what that function takes, still owned as its convention expects.
pub(crate) unsafe fn call_async<O, F>(
    function: *const (),
    async_fn: *const u8,
    owned: O,
    args: impl FnOnce(&mut O) -> AsyncCallArgs,
    completion: F,
) where
    O: Send + 'static,
    F: FnOnce(O, *mut (), *mut ()) + Send + 'static,
{
    let mut call = Box::new(AsyncCall {
        header: AsyncCallTask::new(function, async_fn, complete_boxed::<O, F>),
        owned: core::mem::ManuallyDrop::new(owned),
        completion,
    });
    // Built from the allocation the call will point into rather than from
    // anything on this frame, which is gone by the time Swift reads it.
    call.header.args = args(&mut call.owned);

    unsafe {
        spawn_task(
            (&raw const CIDRE_SWIFT_ASYNC_CALL_TASK_DESCRIPTOR).cast(),
            Box::into_raw(call).cast(),
        );
    }
}

/// [`call_async`] for the shape every binding actually wants: a `Result` whose
/// error is the thrown Swift error bridged to an [`ns::Error`](crate::ns::Error).
///
/// `output` turns what the call produced into the success value, and only runs
/// when nothing was thrown.
///
/// # Safety
///
/// As [`call_async`].
#[cfg(feature = "ns")]
pub(crate) unsafe fn call_async_result<O, T, F>(
    function: *const (),
    async_fn: *const u8,
    owned: O,
    args: impl FnOnce(&mut O) -> AsyncCallArgs,
    output: impl FnOnce(O, *mut ()) -> T + Send + 'static,
    callback: F,
) where
    O: Send + 'static,
    T: 'static,
    F: FnOnce(Result<T, crate::arc::R<crate::ns::Error>>) + Send + 'static,
{
    unsafe {
        call_async(
            function,
            async_fn,
            owned,
            args,
            move |owned, result, error| {
                if error.is_null() {
                    callback(Ok(output(owned, result)));
                } else {
                    // Bridging hands back the error box itself, so the task's
                    // reference becomes the `ns::Error`'s and is not released again.
                    callback(Err(crate::arc::R::from_raw(
                        abi::error_as_ns_error(error).cast(),
                    )));
                }
            },
        );
    }
}

/// Runs the Rust completion once the call is over, and frees the task.
extern "C" fn cidre_swift_async_call_complete(task: *mut AsyncCallTask) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        // The header's own type is what remembers what the tail holds.
        ((*task).complete)(task);
    }));
}

swift_async_task_descriptor!(
    CIDRE_SWIFT_ASYNC_CALL_TASK_DESCRIPTOR,
    entry: async_call_entry,
    context_size: "96",
);

/// Loads the call's arguments out of the task, allocates the callee's context,
/// and tail-calls it.
#[unsafe(naked)]
unsafe extern "C" fn async_call_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        // The task is our context; keep it and our own async context where the
        // resume and finish trampolines can find them. The two slots are
        // adjacent, so one store puts both away.
        "stp x22, x20, [x22, #16]",
        // Word 1 of the callee's async function pointer is its context size.
        "ldr x8, [x20, #{async_fn}]",
        "ldr w0, [x8, #4]",
        "bl {task_alloc}",
        "mov x9, x0",
        "str x9, [x22, #40]",
        swift_async_store_parent!(),
        swift_async_store_resume!("{resume}"),
        // Every argument register the call could want, whether or not this
        // callee reads it, so one trampoline serves calls of every shape. Each
        // pair is contiguous in the block and lands in consecutive registers,
        // which is what a load-pair wants, so eight loads are four.
        "ldr x10, [x22, #24]",
        "ldr x16, [x10, #{function}]",
        "ldp x0, x1, [x10, #{args_lo}]",
        "ldp x2, x3, [x10, #{args_hi}]",
        "ldp q0, q1, [x10, #{vectors_lo}]",
        "ldp q2, q3, [x10, #{vectors_hi}]",
        "ldr x20, [x10, #{swift_self}]",
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        function = const core::mem::offset_of!(AsyncCallTask, function),
        async_fn = const core::mem::offset_of!(AsyncCallTask, async_fn),
        swift_self = const core::mem::offset_of!(AsyncCallTask, args.swift_self),
        args_lo = const core::mem::offset_of!(AsyncCallTask, args.args),
        args_hi = const core::mem::offset_of!(AsyncCallTask, args.args) + 2 * ARG_STRIDE,
        vectors_lo = const core::mem::offset_of!(AsyncCallTask, args.vectors),
        vectors_hi = const core::mem::offset_of!(AsyncCallTask, args.vectors) + 2 * VECTOR_STRIDE,
        task_alloc = sym swift_task_alloc,
        resume = sym async_call_resume,
    );
}

/// Resumed with the result in `x0` and any thrown error in `x20`. Records both
/// in the task, then hops to a plain frame before running Rust.
#[unsafe(naked)]
unsafe extern "C" fn async_call_resume() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "48", fp: "32", ctx: "24"),
        swift_async_load_parent!(),
        "str x9, [sp, #16]",
        // Spilled as a pair, error first, which is the order the store below
        // reads them back in.
        "stp x20, x0, [sp]",
        "mov x0, x22",
        "bl {task_dealloc}",
        "ldr x9, [sp, #16]",
        "ldr x10, [x9, #24]",
        // The spill holds the error first and the result second; the task wants
        // them the other way round, which the pair swaps on the way through.
        "ldp x11, x12, [sp]",
        "stp x12, x11, [x10, #{result}]",
        // Nothing since has touched the parent context, so it is still in `x9`
        // rather than only in the frame.
        "mov x22, x9",
        swift_async_function_pointer!("{finish}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "48", fp: "32"),
        "b {task_switch}",
        result = const core::mem::offset_of!(AsyncCallTask, result),
        task_dealloc = sym swift_task_dealloc,
        finish = sym async_call_finish,
        task_switch = sym swift_task_switch,
    );
}

/// Runs the Rust completion, then returns to whoever created the task.
#[unsafe(naked)]
unsafe extern "C" fn async_call_finish() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {complete}",
        "ldr x22, [sp, #8]",
        "ldr x9, [x22, #16]",
        swift_async_load_resume!(),
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        complete = sym cidre_swift_async_call_complete,
    );
}

/// A concrete Swift `AsyncSequence` that runs without any hardware.
///
/// Every sequence the crate binds for real needs a device — DockKit's all throw
/// without a docked accessory — so nothing otherwise exercises an element
/// actually crossing the boundary, only the failure paths.
/// `NotificationCenter.Notifications` is driven entirely by
/// `NotificationCenter.post`, so a test can produce elements on demand.
#[cfg(all(test, feature = "ns"))]
mod notification_sequence {
    use crate::{ns, swift::SwiftMetadata, swift::value::Storage};

    /// One delivered `Foundation.Notification`.
    ///
    /// Nothing is read out of the value: these tests are about the iteration
    /// machinery, so an element only has to arrive and then be destroyed.
    pub struct Notification;

    unsafe impl crate::swift::SwiftMetadata for Notification {
        fn metadata() -> *const crate::swift::abi::TypeMetadata {
            NotificationValue::metadata()
        }
    }

    unsafe impl crate::swift::FromSwift for Notification {
        unsafe fn copy_swift(_value: *const ()) -> Self {
            Self
        }

        unsafe fn take_swift(value: *mut ()) -> Self {
            unsafe { crate::swift::abi::destroy_value(value, Self::metadata()) };
            Self
        }
    }

    #[link(name = "Foundation", kind = "framework")]
    unsafe extern "C" {
        #[link_name = "$s10Foundation12NotificationVMa"]
        fn notification_metadata();

        #[link_name = "$sSo20NSNotificationCenterC10FoundationE13NotificationsCMa"]
        fn notifications_metadata();

        #[link_name = "$sSo20NSNotificationCenterC10FoundationE13notifications5named6objectAbCE13NotificationsCSo0A4Namea_yXlSgtF"]
        fn notifications_named();
    }

    crate::define_swift_marker!(pub(crate) NotificationValue = accessor notification_metadata);

    // `NotificationCenter.Notifications` is a class, so its value is a
    // reference and `arc::R` is what the sequence holds.
    crate::define_swift_class!(pub NotificationsClass = accessor notifications_metadata);

    define_async_sequence! {
        /// `NotificationCenter.Notifications`.
        Notifications, class NotificationsClass, NotificationsIteratorValue,
        framework = "Foundation",
        element = Notification,
        iterator_metadata = notifications_iterator_metadata => "$sSo20NSNotificationCenterC10FoundationE13NotificationsC8IteratorVMa",
        make_iterator = notifications_make_iterator => "$sSo20NSNotificationCenterC10FoundationE13NotificationsC17makeAsyncIteratorAE0G0VyF",
        next = notifications_next => "$sSo20NSNotificationCenterC10FoundationE13NotificationsC8IteratorV4nextAC12NotificationVSgyYaF",
        next_async = NOTIFICATIONS_NEXT_ASYNC => "$sSo20NSNotificationCenterC10FoundationE13NotificationsC8IteratorV4nextAC12NotificationVSgyYaFTu",
        async_iter = NotificationsAsyncIter,
    }

    impl Notifications {
        /// `NotificationCenter.notifications(named:object:)`.
        pub fn named(center: &ns::NotificationCenter, name: &ns::NotificationName) -> Self {
            unsafe {
                let sequence: crate::arc::R<NotificationsClass> = crate::arc::R::from_raw(
                    call_method_objects2_to_object(
                        notifications_named as *const (),
                        (center as *const ns::NotificationCenter).cast(),
                        (name as *const ns::NotificationName).cast(),
                        core::ptr::null(),
                    )
                    .cast(),
                );
                // A class-typed sequence is one reference, written straight
                // into the storage the runtime sized for it.
                let mut storage = crate::swift::value::DynamicStorage::new(<crate::arc::R<
                    NotificationsClass,
                > as SwiftMetadata>::metadata(
                ));
                storage
                    .as_mut_ptr()
                    .cast::<*mut NotificationsClass>()
                    .write(sequence.into_raw());
                Self::from_storage(storage)
            }
        }
    }

    /// Calls a Swift method with `self` in `x20` and two object arguments.
    #[inline]
    unsafe fn call_method_objects2_to_object(
        function: *const (),
        this: *const (),
        first: *const (),
        second: *const (),
    ) -> *mut () {
        unsafe { crate::swift::abi::call::static_values_to_object(function, this, first, second) }
    }
}

#[cfg(test)]
mod tests {
    /// Drives the stream through its feed callback, which is what a Swift
    /// sequence does, so the buffering and wake-up can be checked without one.
    #[cfg(feature = "async")]
    fn block_on<F: Future>(mut fut: F) -> F::Output {
        use std::sync::{Arc, Condvar, Mutex};
        use std::task::{Context, Poll, Wake, Waker};

        struct Signal(Mutex<bool>, Condvar);
        impl Wake for Signal {
            fn wake(self: Arc<Self>) {
                *self.0.lock().unwrap() = true;
                self.1.notify_all();
            }
        }

        let signal = Arc::new(Signal(Mutex::new(false), Condvar::new()));
        let waker = Waker::from(signal.clone());
        let mut cx = Context::from_waker(&waker);
        let mut fut = unsafe { std::pin::Pin::new_unchecked(&mut fut) };
        loop {
            if let Poll::Ready(out) = fut.as_mut().poll(&mut cx) {
                return out;
            }
            let mut ready = signal.0.lock().unwrap();
            while !*ready {
                ready = signal.1.wait(ready).unwrap();
            }
            *ready = false;
        }
    }

    /// Polls `fut` once against a waker nobody listens to, which is what puts a
    /// request in flight without waiting for the element that answers it.
    #[cfg(feature = "async")]
    fn poll_once<F: Future + Unpin>(fut: &mut F) -> std::task::Poll<F::Output> {
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        std::pin::Pin::new(fut).poll(&mut cx)
    }

    /// A `next()` asks the Swift task for an element only once it is polled, so
    /// dropping one must not consume an element that no later `next()` sees.
    ///
    /// The task is stood in for here: driving a real one would need a sequence
    /// that yields on demand, and what is under test is the state machine, not
    /// the trampolines.
    #[test]
    #[cfg(feature = "async")]
    fn a_dropped_next_future_consumes_no_element() {
        use super::{PullControl, PullShared, PullState, PulledIter};
        use std::task::Poll;

        unsafe fn copy(value: *const ()) -> u32 {
            unsafe { value.cast::<u32>().read() }
        }

        let shared = std::sync::Arc::new(PullShared {
            state: parking_lot::Mutex::new(PullState {
                continuation: None,
                outstanding: false,
                delivered: None,
                waker: None,
                ended: false,
                stop: false,
            }),
            copy,
        });
        let mut iter = PulledIter {
            shared: shared.clone(),
        };

        // The task fetches one element per request in flight, and never runs
        // otherwise. It is parked throughout, so no continuation is involved.
        let mut produced = 0u32;
        let run_task = |produced: &mut u32| {
            while shared.state.lock().outstanding {
                *produced += 1;
                let value = *produced;
                shared.deliver(Some((&raw const value).cast()));
            }
        };

        drop(iter.next());
        run_task(&mut produced);
        assert_eq!(0, produced, "an unpolled `next()` asks for nothing");

        // Polled and then dropped: the one request it made is answered, and
        // that element has to be waiting for the next caller.
        let mut abandoned = iter.next();
        assert!(poll_once(&mut abandoned).is_pending());
        drop(abandoned);
        run_task(&mut produced);
        assert_eq!(1, produced);

        let mut next = iter.next();
        assert_eq!(
            Poll::Ready(Some(1)),
            poll_once(&mut next),
            "the abandoned future's element must not be lost"
        );
        drop(next);
        run_task(&mut produced);
        assert_eq!(1, produced, "taking a delivered element refetches nothing");

        // Repeated polls of one future are a single request, not one each.
        let mut next = iter.next();
        assert!(poll_once(&mut next).is_pending());
        assert!(poll_once(&mut next).is_pending());
        run_task(&mut produced);
        assert_eq!(2, produced);
        assert_eq!(Poll::Ready(Some(2)), poll_once(&mut next));
    }

    /// Keeps posting `name` until `arrived` reports the element landed.
    ///
    /// The sequence only starts observing once the Swift task has built its
    /// iterator, so a single post can be dropped on the floor; retrying makes
    /// the test independent of when that happens.
    #[cfg(feature = "ns")]
    fn post_until(name: &crate::ns::NotificationName, mut arrived: impl FnMut() -> bool) -> bool {
        let center = crate::ns::NotificationCenter::default();
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while std::time::Instant::now() < deadline {
            center.post_with_name_obj(name, None);
            if arrived() {
                return true;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        false
    }

    /// The callback path has to see a real element cross from Swift, not just
    /// the end-of-sequence and error cases the device-backed sequences reach.
    #[cfg(feature = "ns")]
    #[test]
    fn for_each_while_delivers_an_element_from_swift() {
        use super::notification_sequence::Notifications;
        use crate::ns;

        // The default centre is process-wide, so this test owns its own name.
        let name = ns::NotificationName::with_raw(ns::str!(c"cidre.swift.async.for_each"));
        let center = ns::NotificationCenter::default();

        let (tx, rx) = std::sync::mpsc::channel();
        Notifications::named(&center, name).for_each_while(move |value| {
            let _ = tx.send(value.is_some());
            // Stop at the first element so the Swift task finishes and frees
            // itself rather than outliving the test.
            false
        });

        assert!(
            post_until(name, || rx.try_recv() == Ok(true)),
            "a posted notification must reach the Rust callback"
        );
    }

    /// The pulled iterator must advance: two `next().await`s, two elements.
    #[cfg(feature = "ns")]
    #[test]
    #[cfg(feature = "async")]
    fn async_iter_advances_across_awaits() {
        use super::notification_sequence::Notifications;
        use crate::ns;
        use std::sync::atomic::{AtomicBool, Ordering};

        let name = ns::NotificationName::with_raw(ns::str!(c"cidre.swift.async.async_iter"));
        let center = ns::NotificationCenter::default();
        let mut iter = Notifications::named(&center, name).async_iter();

        // `next()` parks this thread, so the posts have to come from another.
        let stop = std::sync::Arc::new(AtomicBool::new(false));
        let poster = std::thread::spawn({
            let stop = stop.clone();
            move || {
                let name =
                    ns::NotificationName::with_raw(ns::str!(c"cidre.swift.async.async_iter"));
                let center = ns::NotificationCenter::default();
                while !stop.load(Ordering::Relaxed) {
                    center.post_with_name_obj(name, None);
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        });

        // Enough turns of the loop that a stale continuation, a leaked callee
        // context, or a lost wake-up would show up.
        for round in 0..25 {
            assert!(
                block_on(iter.next()).is_some(),
                "element {round} must arrive"
            );
        }

        stop.store(true, Ordering::Relaxed);
        poster.join().unwrap();
    }

    /// The cancel-safety the pulled iterator advertises, against a real Swift
    /// task rather than a stand-in: abandoning a polled `next()` must leave the
    /// iterator able to deliver, with the abandoned request's element going to
    /// the next caller instead of being dropped on the floor.
    #[cfg(feature = "ns")]
    #[test]
    #[cfg(feature = "async")]
    fn a_dropped_next_leaves_the_iterator_usable() {
        use super::notification_sequence::Notifications;
        use crate::ns;
        use std::sync::atomic::{AtomicBool, Ordering};

        let name = ns::NotificationName::with_raw(ns::str!(c"cidre.swift.async.cancel"));
        let center = ns::NotificationCenter::default();
        let mut iter = Notifications::named(&center, name).async_iter();

        let stop = std::sync::Arc::new(AtomicBool::new(false));
        let poster = std::thread::spawn({
            let stop = stop.clone();
            move || {
                let name = ns::NotificationName::with_raw(ns::str!(c"cidre.swift.async.cancel"));
                let center = ns::NotificationCenter::default();
                while !stop.load(Ordering::Relaxed) {
                    center.post_with_name_obj(name, None);
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        });

        for round in 0..25 {
            // A request really goes out here, and is then abandoned while the
            // Swift task is still inside `next()`.
            let mut abandoned = iter.next();
            assert!(poll_once(&mut abandoned).is_pending());
            drop(abandoned);

            // Whatever that request fetched has to still be reachable, and the
            // iterator has to keep advancing afterwards.
            assert!(
                block_on(iter.next()).is_some(),
                "element {round} must survive the abandoned future"
            );
        }

        stop.store(true, Ordering::Relaxed);
        poster.join().unwrap();
    }

    /// Dropping the iterator while its Swift task is parked has to wind the
    /// task up, not strand it waiting for a request that will never come.
    #[cfg(feature = "ns")]
    #[test]
    #[cfg(feature = "async")]
    fn dropping_a_parked_iterator_releases_its_task() {
        use super::notification_sequence::Notifications;
        use crate::ns;

        let name = ns::NotificationName::with_raw(ns::str!(c"cidre.swift.async.drop"));
        let center = ns::NotificationCenter::default();

        for _ in 0..200 {
            let mut iter = Notifications::named(&center, name).async_iter();

            // Polling is what asks for an element, so the future has to be
            // polled and not merely built to leave the request outstanding and
            // put the task somewhere between parked and fetching when the drop
            // lands.
            let mut pending = iter.next();
            assert!(poll_once(&mut pending).is_pending());
            drop(pending);
            drop(iter);
        }

        // Nothing to assert beyond surviving: a stranded or double-freed task
        // shows up as a hang or a crash here.
        center.post_with_name_obj(name, None);
    }
}
