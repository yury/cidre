//! Calls into Swift that `#[swift::call]` does not generate.
//!
//! Swift's calling convention is the C one plus three registers Rust cannot
//! name in a function type: `x20` carries `self`, `x21` carries a thrown error,
//! and `x8` points at storage for a value returned indirectly. What is left
//! here are the shapes a generated call cannot express: members reached through
//! a raw function pointer the caller looked up at runtime (a witness, an opaque
//! getter, a static a macro picked), members of generic types, which also take
//! the generic context, and the one initializer whose optional size travels in
//! three argument registers.
//!
//! Every one goes through a [`swift_thunk!`], reached by an ordinary C call,
//! so the AAPCS register mask applies and the caller keeps `d8`-`d15`. An
//! `asm!` block would have to declare the C clobbers, which cover all of
//! `v8`-`v15`, costing every caller eight registers.

use super::{RawString, TypeMetadata};

/// Declares a naked thunk that fills the registers C cannot name and hands off
/// to a Swift entry point held in one of its own arguments.
///
/// Integer parameters fill `x0` up and floating-point ones `d0` up, each in
/// declaration order, so the instructions can name them. A thunk that writes
/// `x20` must restore it — it is callee-saved under C, and the Swift callee
/// gives back what the thunk left, not what the caller had — which is what
/// [`thunk_enter!`] and [`thunk_leave!`] are for.
macro_rules! swift_thunk {
    (
        $(#[$meta:meta])*
        fn $name:ident($($param:ident: $ty:ty),* $(,)?) $(-> $ret:ty)?,
        $($insn:expr),+ $(,)?
    ) => {
        $(#[$meta])*
        #[unsafe(naked)]
        unsafe extern "C" fn $name($($param: $ty),*) $(-> $ret)? {
            core::arch::naked_asm!($($insn),+)
        }
    };
}

/// Opens a thunk that calls rather than tail-calls: a frame record, so a
/// backtrace through the Swift callee still reaches the Rust caller, and `x20`
/// saved beside it. The frame's last word is free for the thunk's own use.
macro_rules! thunk_enter {
    () => {
        "stp x29, x30, [sp, #-32]!\nmov x29, sp\nstr x20, [sp, #16]"
    };
}

/// Closes what [`thunk_enter!`] opened and returns.
macro_rules! thunk_leave {
    () => {
        "ldr x20, [sp, #16]\nldp x29, x30, [sp], #32\nret"
    };
}

/// Three words, which C returns through a buffer at `x8` and Swift in
/// `x0`-`x2`, so a thunk moves them from one to the other.
#[repr(C)]
struct Words3(u64, u64, u64);

/// Calls a member of a generic type that returns its value indirectly.
///
/// Unlike [`value_to_value`], the callee also needs the generic context,
/// which is the enclosing type's metadata.
///
/// # Safety
///
/// `function` must be a member of the type `metadata` describes, `value` a
/// valid instance of it, and `out` uninitialized storage for the result.
#[inline]
pub unsafe fn generic_value_to_value(
    function: *const (),
    value: *const (),
    metadata: *const TypeMetadata,
    out: *mut (),
) {
    unsafe { generic_value_to_value_thunk(metadata, out, value, function) }
}

swift_thunk!(
    fn generic_value_to_value_thunk(
        _metadata: *const TypeMetadata,
        _out: *mut (),
        _self: *const (),
        _function: *const (),
    ),
    thunk_enter!(),
    "mov x8, x1",
    "mov x20, x2",
    "blr x3",
    thunk_leave!(),
);

/// Calls a protocol requirement through its witness.
///
/// # Safety
///
/// `function` must be the witness entry for `value`'s conformance, `witness`
/// that conformance's table, and `out` uninitialized storage for the result.
#[inline]
pub unsafe fn witness_value_to_value(
    function: *const (),
    value: *mut (),
    metadata: *const TypeMetadata,
    witness: *const (),
    out: *mut (),
) {
    unsafe { witness_value_to_value_thunk(metadata, witness, out, value, function) }
}

swift_thunk!(
    fn witness_value_to_value_thunk(
        _metadata: *const TypeMetadata,
        _witness: *const (),
        _out: *mut (),
        _self: *mut (),
        _function: *const (),
    ),
    thunk_enter!(),
    "mov x8, x2",
    "mov x20, x3",
    "blr x4",
    thunk_leave!(),
);

/// Calls a member of a generic type that returns three words directly, such as
/// a `CMTime`.
///
/// # Safety
///
/// As [`generic_value_to_value`], and the member must return exactly three
/// words in registers.
#[inline]
pub unsafe fn generic_value_to_words3(
    function: *const (),
    value: *const (),
    metadata: *const TypeMetadata,
) -> (u64, u64, u64) {
    let words = unsafe { words3_thunk(metadata.cast(), value, function) };
    (words.0, words.1, words.2)
}

swift_thunk!(
    /// One operand in `x0`, `self` in `x20`, and three words back.
    fn words3_thunk(_first: *const (), _self: *const (), _function: *const ()) -> Words3,
    thunk_enter!(),
    "str x8, [sp, #24]",
    "mov x20, x1",
    "blr x2",
    "ldr x9, [sp, #24]",
    "stp x0, x1, [x9]",
    "str x2, [x9, #16]",
    thunk_leave!(),
);

swift_thunk!(
    /// Two operands and the type's metadata as `self`.
    fn static_pair_to_object_thunk(
        _first: *const (),
        _second: *const (),
        _self: *const (),
        _function: *const (),
    ) -> *mut (),
    thunk_enter!(),
    "mov x20, x2",
    "blr x3",
    thunk_leave!(),
);

/// # Safety
///
/// The two values must be what the static method takes, and `type_metadata`
/// the metadata of the type it belongs to.
#[inline]
pub unsafe fn static_values_to_object(
    function: *const (),
    type_metadata: *const (),
    first: *const (),
    second: *const (),
) -> *mut () {
    unsafe { static_pair_to_object_thunk(first, second, type_metadata, function) }
}

/// `DockAccessory.CameraInformation.init(...)`, whose seven arguments are more
/// than any other call these bindings make.
///
/// # Safety
///
/// The arguments must be what that initializer takes, and `out` uninitialized
/// storage for the camera information.
#[cfg(feature = "av")]
#[inline]
pub unsafe fn camera_information_init(
    function: *const (),
    device_type: *const (),
    position: isize,
    orientation: *const (),
    intrinsics: *const (),
    reference_dimensions: (u64, u64, u64),
    out: *mut (),
) {
    unsafe {
        camera_information_init_thunk(
            device_type,
            position,
            orientation,
            intrinsics,
            reference_dimensions.0,
            reference_dimensions.1,
            reference_dimensions.2,
            out,
            function,
        )
    }
}

#[cfg(feature = "av")]
swift_thunk!(
    /// Seven argument words and the buffer fill `x0`-`x7`, so the callee's
    /// address arrives on the stack; the thunk tail-calls it.
    #[allow(clippy::too_many_arguments)]
    fn camera_information_init_thunk(
        _device_type: *const (),
        _position: isize,
        _orientation: *const (),
        _intrinsics: *const (),
        _w0: u64,
        _w1: u64,
        _w2: u64,
        _out: *mut (),
        _function: *const (),
    ),
    "ldr x16, [sp]",
    "mov x8, x7",
    "br x16",
);

/// # Safety
///
/// `out` must be uninitialized storage for what the getter returns.
#[inline]
pub unsafe fn to_value(function: *const (), out: *mut ()) {
    unsafe { to_value_thunk(out, function) }
}

swift_thunk!(
    fn to_value_thunk(_out: *mut (), _function: *const ()),
    "mov x8, x0",
    "br x1",
);

/// A getter taking `self` in `x20` and again in `x0`; only the result differs.
macro_rules! value_getter {
    ($(#[$meta:meta])* $vis:vis fn $name:ident -> $ret:ty, $thunk:ident) => {
        swift_thunk!(
            fn $thunk(_value: *const (), _self: *const (), _function: *const ()) -> $ret,
            thunk_enter!(),
            "mov x20, x1",
            "blr x2",
            thunk_leave!(),
        );

        $(#[$meta])*
        #[inline]
        $vis unsafe fn $name(function: *const (), value: *const ()) -> $ret {
            unsafe { $thunk(value, value, function) }
        }
    };
}

value_getter!(
    /// A getter returning a `String`, in `x0` and `x1`.
    ///
    /// # Safety
    ///
    /// `function` must be a getter of the type `value` is an instance of that
    /// returns a `String`.
    pub fn value_to_string -> RawString,
    value_to_string_thunk
);

/// A getter returning three words in `x0`-`x2`.
///
/// # Safety
///
/// `function` must be a getter of the type `value` is an instance of that
/// returns exactly three words in registers.
#[inline]
pub unsafe fn value_to_words3(function: *const (), value: *const ()) -> (u64, u64, u64) {
    let words = unsafe { words3_thunk(value, value, function) };
    (words.0, words.1, words.2)
}

/// # Safety
///
/// `out` must be uninitialized storage for what the getter returns.
#[inline]
pub unsafe fn object_to_value(function: *const (), object: *const (), out: *mut ()) {
    unsafe { self_to_value_thunk(object, out, object, function) }
}

swift_thunk!(
    /// `self` in `x0` and `x20`, and the caller's buffer in `x8`.
    fn self_to_value_thunk(
        _value: *const (),
        _out: *mut (),
        _self: *const (),
        _function: *const (),
    ),
    thunk_enter!(),
    "mov x8, x1",
    "mov x20, x2",
    "blr x3",
    thunk_leave!(),
);

/// # Safety
///
/// As [`object_to_value`].
#[inline]
pub unsafe fn value_to_value(function: *const (), value: *const (), out: *mut ()) {
    unsafe { self_to_value_thunk(value, out, value, function) }
}
