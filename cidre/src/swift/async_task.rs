//! Shared assembly for the hand-written Swift async task trampolines.
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

//#[link(name = "swift_Concurrency")]
unsafe extern "C" {
    /// Only ever referenced as a `sym` operand, so the signature is unused.
    pub(crate) fn swift_task_alloc();
    pub(crate) fn swift_task_dealloc();
    pub(crate) fn swift_task_switch();
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
            $crate::swift::async_task::swift_async_pauth_prologue!(),
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
            $crate::swift::async_task::swift_async_pauth_epilogue!(),
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
