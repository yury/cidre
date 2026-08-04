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

#[inline]
pub(crate) unsafe fn task_create_common(
    flags: usize,
    future_result_type: *const TypeMetadata,
    function: *const (),
    context: *mut (),
    initial_context_size: usize,
) -> (*mut (), *mut ()) {
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
    (task, initial_context)
}

#[inline]
pub(crate) unsafe fn task_create(
    flags: usize,
    future_result_type: *const TypeMetadata,
    function: *const (),
    context: *mut (),
) -> (*mut (), *mut ()) {
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
    (task, initial_context)
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
#[derive(Clone, Copy)]
pub(crate) struct AsyncSequenceSymbols {
    pub(crate) iterator_metadata: *const TypeMetadata,
    pub(crate) element_metadata: *const TypeMetadata,
    pub(crate) make_iterator: *const (),
    pub(crate) next: *const (),
    pub(crate) next_async_fn: *const u8,
}

unsafe impl Send for AsyncSequenceSymbols {}

struct AsyncSequenceTask {
    sequence: Option<AnyValue>,
    iterator: Option<AnyValue>,
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
    unsafe {
        let element = (*task).symbols.element_metadata;
        abi::value_layout(abi::optional_metadata(element)).stride.max(1)
    }
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
            abi::call_value_to_value(
                task.symbols.make_iterator,
                sequence.as_ptr(),
                storage.as_mut_ptr(),
            );
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
        let element = task.symbols.element_metadata;
        let optional = abi::optional_metadata(element);
        if abi::get_enum_tag_single_payload(result.as_ptr().cast(), 1, element) == 1 {
            abi::destroy_value(result.as_ptr().cast(), optional);
            (task.callback)(None);
            return false;
        }

        let keep_going = (task.callback)(Some(result.as_ptr().cast_const().cast()));
        abi::destroy_value(result.as_ptr().cast(), element);
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

pub(crate) use {
    swift_async_epilogue, swift_async_function_pointer, swift_async_load_parent,
    swift_async_load_resume, swift_async_pauth_epilogue, swift_async_pauth_prologue,
    swift_async_prologue, swift_async_store_parent, swift_async_store_resume,
    swift_async_task_descriptor, swift_opaque_iterator_typeref,
};

define_async_sequence_runner!();
