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
    value::{AnyValue, DynamicStorage, Optional, Value},
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

    fn swift_task_create();
    fn swift_task_create_common();
    fn swift_task_enqueueGlobal(task: *mut ());

    /// Arms a continuation embedded in `context`, returning the handle that
    /// resumes it. Declared `SWIFT_CC(swift)`, which for these arguments is the
    /// C convention.
    fn swift_continuation_init(context: *mut (), flags: usize) -> *mut ();

    /// Resumes a continuation from outside the task. Also effectively C.
    fn swift_continuation_resume(continuation: *mut ());

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

/// An owned reference to a Swift `AsyncTask`.
///
/// `swift_task_create` returns the task at +1, and the enqueued job holds a
/// reference of its own: Swift's own codegen for a task whose handle is
/// discarded releases the returned reference immediately. Dropping this does
/// the same, so a task nobody keeps is freed once it finishes instead of
/// leaking its allocation.
///
/// Keeping one alive is what a future cancellation API would hold on to.
#[repr(transparent)]
pub(crate) struct Task(NonNull<()>);

// The Swift runtime reference-counts tasks atomically, so a reference may be
// dropped from whichever thread ends up owning it.
unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Task {
    /// Takes ownership of a task reference the runtime just handed back.
    ///
    /// # Safety
    ///
    /// The caller must own a reference to `task`.
    #[inline]
    unsafe fn from_raw(task: *mut ()) -> Self {
        Self(NonNull::new(task).expect("Swift task creation failed"))
    }

    #[inline]
    pub(crate) fn as_ptr(&self) -> *mut () {
        self.0.as_ptr()
    }
}

impl Drop for Task {
    #[inline]
    fn drop(&mut self) {
        unsafe { abi::object_release(self.0.as_ptr().cast_const()) }
    }
}

#[inline]
pub(crate) unsafe fn task_create_common(
    flags: usize,
    future_result_type: *const TypeMetadata,
    function: *const (),
    context: *mut (),
    initial_context_size: usize,
) -> (Task, *mut ()) {
    let task: *mut ();
    let initial_context: *mut ();
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_task_create_common,
            inlateout("x0") flags => task,
            in("x1") core::ptr::null_mut::<()>(),
            in("x2") future_result_type,
            in("x3") function,
            in("x4") context,
            in("x5") initial_context_size,
            lateout("x1") initial_context,
            clobber_abi("C"),
        );
    }
    (unsafe { Task::from_raw(task) }, initial_context)
}

#[inline]
pub(crate) unsafe fn task_create(
    flags: usize,
    future_result_type: *const TypeMetadata,
    function: *const (),
    context: *mut (),
) -> (Task, *mut ()) {
    let task: *mut ();
    let initial_context: *mut ();
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_task_create,
            inlateout("x0") flags => task,
            in("x1") core::ptr::null_mut::<()>(),
            in("x2") future_result_type,
            in("x3") function,
            in("x4") context,
            lateout("x1") initial_context,
            clobber_abi("C"),
        );
    }
    (unsafe { Task::from_raw(task) }, initial_context)
}

#[inline]
pub(crate) unsafe fn task_enqueue_global(task: *mut ()) {
    unsafe { swift_task_enqueueGlobal(task) }
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
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") sequence,
            in("x0") sequence_metadata,
            in("x1") witness,
            in("x8") iterator,
            clobber_abi("C"),
        );
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
            abi::call_value_to_value(self.make_iterator, this, out);
        }
    }
}

struct AsyncSequenceTask {
    // Declaration order is drop order, and the iterator borrows the sequence.
    iterator: Option<AnyValue>,
    sequence: Option<AnyValue>,
    result: Option<NonNull<u8>>,
    symbols: AsyncSequenceSymbols,
    callback: Box<dyn FnMut(Option<*const ()>) -> bool + Send>,
}

/// Iterates a concrete Swift `AsyncSequence` on a Swift task.
///
/// The callback returns whether iteration should continue after an element.
/// It is called once with `None` when the sequence finishes naturally.
pub(crate) fn iterate_async_sequence<S, F>(
    sequence: Value<S>,
    symbols: AsyncSequenceSymbols,
    callback: F,
) where
    S: SwiftMetadata,
    F: FnMut(Option<*const ()>) -> bool + Send + 'static,
{
    unsafe {
        let task = Box::new(AsyncSequenceTask {
            sequence: Some(sequence.erase()),
            iterator: None,
            result: None,
            symbols,
            callback: Box::new(callback),
        });
        let context = Box::into_raw(task).cast();
        let _ = task_create(
            ENQUEUED_DISCARDING_TASK_FLAGS,
            core::ptr::null(),
            (&raw const CIDRE_SWIFT_ASYNC_SEQUENCE_TASK_DESCRIPTOR).cast(),
            context,
        );
    }
}

/// The Swift task's side of a [`PulledIter`].
///
/// Kept alive by the task itself: the trampolines hand this pointer to every
/// Rust callback, and the last one drops it.
#[cfg(feature = "async")]
struct PulledIterTask {
    // Declaration order is drop order, and the iterator borrows the sequence.
    iterator: AnyValue,
    _sequence: AnyValue,
    symbols: AsyncSequenceSymbols,
    result: Option<NonNull<u8>>,
    control: std::sync::Arc<dyn PullControl>,
}

#[cfg(feature = "async")]
unsafe impl Send for PulledIterTask {}

/// The half of a pulled iterator the Swift task talks to, with the element type
/// erased so one set of trampolines serves every sequence.
#[cfg(feature = "async")]
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
pub(crate) fn pulled_async_iter<S, T>(
    sequence: Value<S>,
    symbols: AsyncSequenceSymbols,
    copy: unsafe fn(*const ()) -> T,
) -> PulledIter<T>
where
    S: SwiftMetadata,
    T: Send + 'static,
{
    unsafe {
        let sequence = sequence.erase();
        let mut storage = DynamicStorage::new(symbols.iterator_metadata);
        symbols.call_make_iterator(sequence.as_ptr(), storage.as_mut_ptr());

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

        let task = Box::new(PulledIterTask {
            iterator: storage.assume_init(),
            _sequence: sequence,
            symbols,
            result: None,
            control: shared.clone(),
        });
        let context = Box::into_raw(task).cast();
        let _ = task_create(
            ENQUEUED_DISCARDING_TASK_FLAGS,
            core::ptr::null(),
            (&raw const CIDRE_SWIFT_PULLED_ITER_TASK_DESCRIPTOR).cast(),
            context,
        );

        PulledIter { shared }
    }
}

swift_async_task_descriptor!(
    CIDRE_SWIFT_ASYNC_SEQUENCE_TASK_DESCRIPTOR,
    entry: async_sequence_task_entry,
    context_size: "64",
);

macro_rules! async_sequence_next_call {
    () => {
        concat!(
            "ldr x0, [x22, #24]\n",
            "bl {next_async_fn}\n",
            "ldr w0, [x0, #4]\n",
            "bl {task_alloc}\n",
            "mov x9, x0\n",
            "str x9, [x22, #48]\n",
            $crate::swift::concurrency::swift_async_store_parent!(), "\n",
            $crate::swift::concurrency::swift_async_store_resume!("{resume}"), "\n",
            "ldr x0, [x22, #24]\n",
            "bl {next_fn}\n",
            "mov x16, x0\n",
            "ldr x0, [x22, #40]\n",
            "ldr x20, [x22, #32]\n",
            "mov x22, x9\n",
            "mov x21, #0\n",
            $crate::swift::concurrency::swift_async_epilogue!(frame: "32", fp: "16"), "\n",
            "br x16",
        )
    };
}

#[unsafe(naked)]
unsafe extern "C" fn async_sequence_task_entry() {
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
        swift_async_function_pointer!("{run}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {task_switch}",
        result_size = sym cidre_swift_async_sequence_result_size,
        set_result = sym cidre_swift_async_sequence_set_result,
        task_alloc = sym swift_task_alloc,
        task_switch = sym swift_task_switch,
        run = sym async_sequence_task_run,
    );
}

#[unsafe(naked)]
unsafe extern "C" fn async_sequence_task_run() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {prepare}",
        "str x0, [x22, #32]",
        "str x22, [x22, #16]",
        async_sequence_next_call!(),
        prepare = sym cidre_swift_async_sequence_prepare,
        next_async_fn = sym cidre_swift_async_sequence_next_async_fn,
        next_fn = sym cidre_swift_async_sequence_next_fn,
        task_alloc = sym swift_task_alloc,
        resume = sym async_sequence_task_resume,
    );
}

#[unsafe(naked)]
unsafe extern "C" fn async_sequence_task_resume() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        swift_async_load_parent!(),
        "str x9, [sp]",
        "str x9, [x9, #16]",
        "ldr x0, [x9, #48]",
        "bl {task_dealloc}",
        "ldr x22, [sp]",
        swift_async_function_pointer!("{complete}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {task_switch}",
        task_dealloc = sym swift_task_dealloc,
        task_switch = sym swift_task_switch,
        complete = sym async_sequence_task_complete,
    );
}

#[unsafe(naked)]
unsafe extern "C" fn async_sequence_task_complete() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {process}",
        "ldr x22, [sp, #8]",
        "cbnz x0, 1f",
        "ldr x0, [x22, #24]",
        "bl {take_result}",
        "bl {task_dealloc}",
        "ldr x0, [x22, #24]",
        "bl {drop_task}",
        "ldr x9, [x22, #16]",
        swift_async_load_resume!(),
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        "1:",
        "str x22, [x22, #16]",
        async_sequence_next_call!(),
        process = sym cidre_swift_async_sequence_process,
        take_result = sym cidre_swift_async_sequence_take_result,
        drop_task = sym cidre_swift_async_sequence_drop,
        task_dealloc = sym swift_task_dealloc,
        next_async_fn = sym cidre_swift_async_sequence_next_async_fn,
        next_fn = sym cidre_swift_async_sequence_next_fn,
        task_alloc = sym swift_task_alloc,
        resume = sym async_sequence_task_resume,
    );
}

extern "C" fn cidre_swift_async_sequence_result_size(task: *mut AsyncSequenceTask) -> usize {
    unsafe { (*task).symbols.result_stride }
}

extern "C" fn cidre_swift_async_sequence_set_result(
    task: *mut AsyncSequenceTask,
    result: *mut u8,
) {
    unsafe {
        let result = NonNull::new(result).expect("Swift task result allocation failed");
        assert!((*task).result.replace(result).is_none());
    }
}

extern "C" fn cidre_swift_async_sequence_prepare(task: *mut AsyncSequenceTask) -> *mut () {
    unsafe {
        let task = &mut *task;
        if task.iterator.is_none() {
            let sequence = task.sequence.as_ref().expect("Swift sequence missing");
            let mut storage = DynamicStorage::new(task.symbols.iterator_metadata);
            task.symbols
                .call_make_iterator(sequence.as_ptr(), storage.as_mut_ptr());
            task.iterator = Some(storage.assume_init());
        }
        task.iterator.as_mut().unwrap().as_mut_ptr()
    }
}

extern "C" fn cidre_swift_async_sequence_next_async_fn(
    task: *const AsyncSequenceTask,
) -> *const u8 {
    unsafe { (*task).symbols.next_async_fn }
}

extern "C" fn cidre_swift_async_sequence_next_fn(task: *const AsyncSequenceTask) -> *const () {
    unsafe { (*task).symbols.next }
}

extern "C" fn cidre_swift_async_sequence_process(task: *mut AsyncSequenceTask) -> bool {
    catch_unwind(AssertUnwindSafe(|| unsafe {
        let task = &mut *task;
        let result = task.result.expect("Swift task result missing");
        let symbols = task.symbols;
        if symbols.element.enum_tag_single_payload(result.as_ptr().cast(), 1) == 1 {
            symbols.optional.destroy(result.as_ptr().cast());
            (task.callback)(None);
            return false;
        }

        let keep_going = (task.callback)(Some(result.as_ptr().cast_const().cast()));
        symbols.element.destroy(result.as_ptr().cast());
        keep_going
    }))
    .unwrap_or(false)
}

extern "C" fn cidre_swift_async_sequence_take_result(
    task: *mut AsyncSequenceTask,
) -> *mut () {
    unsafe {
        (*task)
            .result
            .take()
            .expect("Swift task result missing")
            .as_ptr()
            .cast()
    }
}

extern "C" fn cidre_swift_async_sequence_drop(task: *mut AsyncSequenceTask) {
    unsafe { drop(Box::from_raw(task)) }
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
#[cfg(feature = "async")]
swift_async_task_descriptor!(
    CIDRE_SWIFT_PULLED_ITER_TASK_DESCRIPTOR,
    entry: pulled_iter_task_entry,
    context_size: "192",
);

/// Allocates the element buffer once for the whole iteration, then waits.
#[cfg(feature = "async")]
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
#[cfg(feature = "async")]
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
#[cfg(feature = "async")]
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
#[cfg(feature = "async")]
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
#[cfg(feature = "async")]
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
#[cfg(feature = "async")]
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
#[cfg(feature = "async")]
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

#[cfg(feature = "async")]
extern "C" fn cidre_pulled_iter_result_size(task: *mut PulledIterTask) -> usize {
    unsafe { (*task).symbols.result_stride }
}

#[cfg(feature = "async")]
extern "C" fn cidre_pulled_iter_set_result(task: *mut PulledIterTask, result: *mut u8) {
    unsafe {
        let result = NonNull::new(result).expect("Swift task result allocation failed");
        assert!((*task).result.replace(result).is_none());
    }
}

#[cfg(feature = "async")]
extern "C" fn cidre_pulled_iter_iterator(task: *mut PulledIterTask) -> *mut () {
    unsafe { (*task).iterator.as_mut_ptr() }
}

#[cfg(feature = "async")]
extern "C" fn cidre_pulled_iter_publish(task: *mut PulledIterTask, continuation: *mut ()) -> bool {
    // Parking on a panic would strand the task forever, so fall through to a
    // fetch instead and let `process` wind the iteration up.
    catch_unwind(AssertUnwindSafe(|| unsafe {
        (*task).control.publish(continuation)
    }))
    .unwrap_or(true)
}

#[cfg(feature = "async")]
extern "C" fn cidre_pulled_iter_should_stop(task: *mut PulledIterTask) -> bool {
    catch_unwind(AssertUnwindSafe(|| unsafe { (*task).control.should_stop() })).unwrap_or(true)
}

#[cfg(feature = "async")]
extern "C" fn cidre_pulled_iter_next_async_fn(task: *const PulledIterTask) -> *const u8 {
    unsafe { (*task).symbols.next_async_fn }
}

#[cfg(feature = "async")]
extern "C" fn cidre_pulled_iter_next_fn(task: *const PulledIterTask) -> *const () {
    unsafe { (*task).symbols.next }
}

#[cfg(feature = "async")]
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

#[cfg(feature = "async")]
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

#[cfg(feature = "async")]
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
        pub struct $sequence($crate::swift::value::Value<$sequence_value>);

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
                storage: $crate::swift::value::Storage<$sequence_value>,
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

/// A Rust view of a Swift `AsyncSequence`.
///
/// Swift drives the sequence on its own cooperative pool and hands each element
/// to a callback; this buffers those elements so a Rust task can `await` them.
/// Dropping the stream stops the Swift-side iteration at its next element.
///
/// There is no backpressure: Swift keeps pulling as fast as the sequence
/// yields, so a consumer that falls behind grows the buffer.
#[cfg(feature = "async")]
pub struct Stream<T> {
    shared: std::sync::Arc<parking_lot::Mutex<StreamShared<T>>>,
}

#[cfg(feature = "async")]
struct StreamShared<T> {
    ready: std::collections::VecDeque<T>,
    ended: bool,
    cancelled: bool,
    waker: Option<std::task::Waker>,
}

#[cfg(feature = "async")]
impl<T: Send + 'static> Stream<T> {
    /// Returns the stream and the callback that feeds it, for handing to one of
    /// the `for_each_while` entry points.
    pub(crate) fn new() -> (Self, impl FnMut(Option<T>) -> bool + Send + 'static) {
        let shared = std::sync::Arc::new(parking_lot::Mutex::new(StreamShared {
            ready: std::collections::VecDeque::new(),
            ended: false,
            cancelled: false,
            waker: None,
        }));

        let sink = shared.clone();
        let feed = move |value: Option<T>| {
            let mut lock = sink.lock();
            if lock.cancelled {
                return false;
            }

            match value {
                Some(value) => lock.ready.push_back(value),
                None => lock.ended = true,
            }

            if let Some(waker) = lock.waker.take() {
                waker.wake();
            }
            !lock.ended
        };

        (Self { shared }, feed)
    }

    /// Waits for the next element, or `None` once the sequence has ended.
    pub fn next(&mut self) -> Next<'_, T> {
        Next { stream: self }
    }
}

#[cfg(feature = "async")]
impl<T> Drop for Stream<T> {
    fn drop(&mut self) {
        // Tells the Swift task to stop at its next element rather than
        // buffering into a queue nobody reads.
        self.shared.lock().cancelled = true;
    }
}

/// The future returned by [`Stream::next`].
#[cfg(feature = "async")]
pub struct Next<'a, T> {
    stream: &'a mut Stream<T>,
}

#[cfg(feature = "async")]
impl<T> std::future::Future for Next<'_, T> {
    type Output = Option<T>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut lock = self.stream.shared.lock();

        if let Some(value) = lock.ready.pop_front() {
            std::task::Poll::Ready(Some(value))
        } else if lock.ended {
            std::task::Poll::Ready(None)
        } else {
            lock.waker = Some(cx.waker().clone());
            std::task::Poll::Pending
        }
    }
}

/// A concrete Swift `AsyncSequence` that runs without any hardware.
///
/// Every sequence the crate binds for real needs a device — DockKit's all throw
/// without a docked accessory — so nothing otherwise exercises an element
/// actually crossing the boundary, only the failure paths.
/// `NotificationCenter.Notifications` is driven entirely by
/// `NotificationCenter.post`, so a test can produce elements on demand.
#[cfg(all(test, feature = "async", feature = "ns"))]
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
                Self::from_storage(Storage::from_class_ref(sequence))
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
        let result: *mut ();
        unsafe {
            core::arch::asm!(
                "blr {function}",
                function = in(reg) function,
                in("x20") this,
                inlateout("x0") first => result,
                in("x1") second,
                clobber_abi("C"),
            );
        }
        result
    }
}

#[cfg(all(test, feature = "async"))]
mod tests {
    use super::Stream;

    /// Drives the stream through its feed callback, which is what a Swift
    /// sequence does, so the buffering and wake-up can be checked without one.
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
    fn poll_once<F: Future + Unpin>(fut: &mut F) -> std::task::Poll<F::Output> {
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        std::pin::Pin::new(fut).poll(&mut cx)
    }

    #[test]
    fn buffers_elements_until_awaited() {
        let (mut stream, mut feed) = Stream::<u32>::new();

        // Elements produced before anyone awaits must not be lost.
        assert!(feed(Some(1)));
        assert!(feed(Some(2)));
        assert!(!feed(None), "the feed stops once the sequence ends");

        assert_eq!(Some(1), block_on(stream.next()));
        assert_eq!(Some(2), block_on(stream.next()));
        assert_eq!(None, block_on(stream.next()));
        assert_eq!(None, block_on(stream.next()), "end is sticky");
    }

    #[test]
    fn wakes_a_pending_consumer() {
        let (mut stream, mut feed) = Stream::<u32>::new();

        let producer = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(50));
            feed(Some(7));
            feed(None);
        });

        // Nothing is buffered yet, so this parks until the producer wakes it.
        assert_eq!(Some(7), block_on(stream.next()));
        assert_eq!(None, block_on(stream.next()));
        producer.join().unwrap();
    }

    #[test]
    fn dropping_the_stream_stops_the_sequence() {
        let (stream, mut feed) = Stream::<u32>::new();
        assert!(feed(Some(1)));

        drop(stream);
        assert!(
            !feed(Some(2)),
            "the Swift side must be told to stop iterating"
        );
    }

    /// A `next()` asks the Swift task for an element only once it is polled, so
    /// dropping one must not consume an element that no later `next()` sees.
    ///
    /// The task is stood in for here: driving a real one would need a sequence
    /// that yields on demand, and what is under test is the state machine, not
    /// the trampolines.
    #[test]
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
