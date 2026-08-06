//! Calling Swift functions from Rust.
//!
//! Swift's calling convention is the C one plus three registers Rust cannot
//! name in a function type: `x20` carries `self`, `x21` carries a thrown error,
//! and `x8` points at storage for a value returned indirectly. So every call
//! goes through one of the two forms below, each naming only the registers the
//! callee actually reads.
//!
//! Filling every register instead — one block taking the whole set as data —
//! costs a `mov` of zero per unused register at each call site, which is a
//! dozen instructions Swift itself never emits. So the register list is per
//! shape, and the hand-off stays here.
//!
//! Most shapes go through [`swift_thunk!`], a naked function the caller reaches
//! by an ordinary C call. That matters because an `asm!` block at the call site
//! has to declare the C clobbers, which cover all of `v8`-`v15` where the ABI
//! preserves only their low halves: Rust's assembly syntax cannot describe a
//! half-preserved register, so every function inlining such a call spills
//! `d8`-`d15` and gives up all eight for its whole body, where Swift spills
//! none. Behind a C call the register mask applies instead and the caller pays
//! nothing.
//!
//! What still uses [`swift_call!`] is what a thunk cannot carry: a throwing
//! call, whose error arrives in `x21` with no C return to put it in; a
//! three-word result, which Swift returns in `x0`-`x2` where C returns it
//! indirectly; and the one initializer whose operands leave no argument
//! register free for the callee's address.

use super::{RawString, TypeMetadata};

/// Two words, which C returns in `x0` and `x1` just as Swift does.
#[repr(C)]
struct Words2(u64, u64);

/// A homogeneous float aggregate, which is what puts these in `d0` up.
#[repr(C)]
struct Doubles2(f64, f64);

#[repr(C)]
struct Doubles4(f64, f64, f64, f64);

/// Declares a naked thunk that fills the registers C cannot name and hands off
/// to a Swift entry point held in one of its own arguments.
///
/// Its parameters are ordinary C ones, so the compiler builds the argument list
/// and takes the result, and the AAPCS register mask applies at the call site —
/// which is what keeps `d8`-`d15` off the caller's stack. Integer parameters
/// fill `x0` up and floating-point ones `d0` up, each in declaration order and
/// independently of the other, so the instructions below can name them.
///
/// A thunk that writes `x20` has to put it back: it is callee-saved under C,
/// and the Swift callee restores whatever the thunk left there rather than what
/// the caller had. One that does not touch it branches straight through.
macro_rules! swift_thunk {
    (
        $(#[$meta:meta])*
        fn $name:ident($($param:ident: $ty:ty),* $(,)?) $(-> $ret:ty)?,
        $($insn:literal),+ $(,)?
    ) => {
        $(#[$meta])*
        #[unsafe(naked)]
        unsafe extern "C" fn $name($($param: $ty),*) $(-> $ret)? {
            core::arch::naked_asm!($($insn),+)
        }
    };
}

/// Calls a Swift entry point with `operands` naming the registers it reads and
/// writes.
///
/// Prefer [`swift_thunk!`], which costs its caller nothing. This form spills
/// `d8`–`d15` in whatever inlines it, for the reason the module note gives, and
/// is kept only for the shapes a thunk cannot carry.
///
/// # Safety
///
/// `function` must be a Swift entry point whose parameters are what `operands`
/// fills in, still owned as its convention expects, and any value it returns
/// indirectly must have storage to be written into.
macro_rules! swift_call {
    ($function:expr, $($operands:tt)*) => {
        core::arch::asm!(
            "blr {__fn}",
            __fn = in(reg) $function,
            $($operands)*
            clobber_abi("C"),
        )
    };
}

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
    "stp x20, x30, [sp, #-16]!",
    "mov x8, x1",
    "mov x20, x2",
    "blr x3",
    "ldp x20, x30, [sp], #16",
    "ret",
);

/// Calls a protocol requirement through its witness: the conforming value as
/// `self`, its metadata and the witness table as the generic context, and the
/// result written into the caller's buffer.
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
    "stp x20, x30, [sp, #-16]!",
    "mov x8, x2",
    "mov x20, x3",
    "blr x4",
    "ldp x20, x30, [sp], #16",
    "ret",
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
    let (w0, w1, w2): (u64, u64, u64);
    unsafe {
        swift_call!(function,
            in("x20") value,
            inlateout("x0") metadata => w0, lateout("x1") w1, lateout("x2") w2,
        );
    }
    (w0, w1, w2)
}

#[inline]
pub unsafe fn double_to_words2(function: *const (), value: f64) -> (u64, u64) {
    let words = unsafe { double_to_words2_thunk(function, value) };
    (words.0, words.1)
}

swift_thunk!(
    fn double_to_words2_thunk(_function: *const (), _value: f64) -> Words2,
    "br x0",
);

/// The deprecated synchronous `setOrientation(_:duration:relative:)`, taking a
/// vector.
///
/// # Safety
///
/// `object` must be the accessory the method is called on.
#[inline]
pub unsafe fn vector_duration_bool_object(
    function: *const (),
    vector: (f64, f64, f64),
    duration: (u64, u64),
    relative: bool,
    object: *const (),
) -> (*mut (), *mut ()) {
    let (result, error): (usize, *mut ());
    unsafe {
        swift_call!(function,
            in("d0") vector.0, in("d1") vector.1, in("d2") vector.2,
            inlateout("x0") duration.0 as usize => result,
            in("x1") duration.1 as usize,
            in("x2") relative as usize,
            in("x20") object,
            inlateout("x21") 0usize => error,
        );
    }
    (result as *mut (), error)
}

/// The same, taking a rotation.
///
/// Note that this passes the quaternion's four `Double`s in `d0`–`d3`, where
/// the asynchronous entry point passes the same value as two vector registers.
///
/// # Safety
///
/// As [`vector_duration_bool_object`].
#[inline]
pub unsafe fn rotation_duration_bool_object(
    function: *const (),
    rotation: (f64, f64, f64, f64),
    duration: (u64, u64),
    relative: bool,
    object: *const (),
) -> (*mut (), *mut ()) {
    let (result, error): (usize, *mut ());
    unsafe {
        swift_call!(function,
            in("d0") rotation.0, in("d1") rotation.1,
            in("d2") rotation.2, in("d3") rotation.3,
            inlateout("x0") duration.0 as usize => result,
            in("x1") duration.1 as usize,
            in("x2") relative as usize,
            in("x20") object,
            inlateout("x21") 0usize => error,
        );
    }
    (result as *mut (), error)
}

/// # Safety
///
/// `out` must be uninitialized storage for what the initializer returns.
#[inline]
pub unsafe fn doubles3_to_throwing_value(
    function: *const (),
    values: (f64, f64, f64),
    out: *mut (),
) -> *mut () {
    let error: *mut ();
    unsafe {
        swift_call!(function,
            in("d0") values.0, in("d1") values.1, in("d2") values.2,
            in("x8") out,
            inlateout("x21") 0usize => error,
        );
    }
    error
}

/// # Safety
///
/// The three values must be what the callee takes, and `out` uninitialized
/// storage for what it returns.
#[inline]
pub unsafe fn values3_to_value(
    function: *const (),
    first: *const (),
    second: *const (),
    third: *const (),
    out: *mut (),
) {
    unsafe { values3_to_value_thunk(first, second, third, out, function) }
}

swift_thunk!(
    fn values3_to_value_thunk(
        _first: *const (),
        _second: *const (),
        _third: *const (),
        _out: *mut (),
        _function: *const (),
    ),
    "mov x8, x3",
    "br x4",
);

/// # Safety
///
/// `value` must be what the static method takes, and `type_metadata` the
/// metadata of the type it belongs to.
#[inline]
pub unsafe fn static_value_bool_to_object(
    function: *const (),
    type_metadata: *const (),
    value: *const (),
    flag: bool,
) -> *mut () {
    unsafe {
        static_pair_to_object_thunk(value, flag as usize as *const (), type_metadata, function)
    }
}

swift_thunk!(
    /// Two operands and the type's metadata as `self`, which is the shape every
    /// static member below shares.
    fn static_pair_to_object_thunk(
        _first: *const (),
        _second: *const (),
        _self: *const (),
        _function: *const (),
    ) -> *mut (),
    "stp x20, x30, [sp, #-16]!",
    "mov x20, x2",
    "blr x3",
    "ldp x20, x30, [sp], #16",
    "ret",
);

/// # Safety
///
/// As [`static_value_bool_to_object`], for a method taking two values.
#[inline]
pub unsafe fn static_values_to_object(
    function: *const (),
    type_metadata: *const (),
    first: *const (),
    second: *const (),
) -> *mut () {
    unsafe { static_pair_to_object_thunk(first, second, type_metadata, function) }
}

/// # Safety
///
/// `array` must be an owned array the method consumes, and `value` what it
/// takes alongside it.
#[inline]
pub unsafe fn static_array_value_to_object(
    function: *const (),
    type_metadata: *const (),
    array: *mut (),
    value: *const (),
) -> *mut () {
    unsafe { static_pair_to_object_thunk(array.cast_const(), value, type_metadata, function) }
}

/// `DockAccessory.Observation.init(identifier:type:rect:faceYawAngle:)`.
///
/// # Safety
///
/// The arguments must be what that initializer takes, and `out` uninitialized
/// storage for an observation.
#[inline]
pub unsafe fn int_value_rect_value_to_value(
    function: *const (),
    integer: isize,
    value: *const (),
    rect: (f64, f64, f64, f64),
    trailing_value: *const (),
    out: *mut (),
) {
    unsafe {
        int_value_rect_value_to_value_thunk(
            integer,
            value,
            trailing_value,
            out,
            function,
            rect.0,
            rect.1,
            rect.2,
            rect.3,
        )
    }
}

swift_thunk!(
    fn int_value_rect_value_to_value_thunk(
        _integer: isize,
        _value: *const (),
        _trailing: *const (),
        _out: *mut (),
        _function: *const (),
        _r0: f64,
        _r1: f64,
        _r2: f64,
        _r3: f64,
    ),
    "mov x8, x3",
    "br x4",
);

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
        swift_call!(function,
            in("x0") device_type, in("x1") position,
            in("x2") orientation, in("x3") intrinsics,
            in("x4") reference_dimensions.0 as usize,
            in("x5") reference_dimensions.1 as usize,
            in("x6") reference_dimensions.2 as usize,
            in("x8") out,
        );
    }
}

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

/// Declares a getter that takes its `self` in `x20` and again in `x0`, which is
/// every reader below; only the register the result lands in differs.
macro_rules! value_getter {
    ($(#[$meta:meta])* $vis:vis fn $name:ident -> $ret:ty, $thunk:ident) => {
        swift_thunk!(
            fn $thunk(_value: *const (), _self: *const (), _function: *const ()) -> $ret,
            "stp x20, x30, [sp, #-16]!",
            "mov x20, x1",
            "blr x2",
            "ldp x20, x30, [sp], #16",
            "ret",
        );

        $(#[$meta])*
        #[inline]
        $vis unsafe fn $name(function: *const (), value: *const ()) -> $ret {
            unsafe { $thunk(value, value, function) }
        }
    };
}

value_getter!(pub fn value_to_int -> isize, value_to_int_thunk);
value_getter!(fn value_to_bool_word -> usize, value_to_bool_thunk);
value_getter!(fn value_to_doubles2_pair -> Doubles2, value_to_doubles2_thunk);
value_getter!(fn value_to_rect_quad -> Doubles4, value_to_rect_thunk);
value_getter!(pub fn value_to_object -> *mut (), value_to_object_thunk);
value_getter!(pub fn value_to_string -> RawString, value_to_string_thunk);

/// Swift's `Bool` occupies one register whose other bits are the callee's
/// business, so it is taken as a word and masked.
#[inline]
pub unsafe fn value_to_bool(function: *const (), value: *const ()) -> bool {
    unsafe { value_to_bool_word(function, value) & 1 != 0 }
}

#[inline]
pub unsafe fn value_to_words3(function: *const (), value: *const ()) -> (u64, u64, u64) {
    let (w0, w1, w2): (u64, u64, u64);
    unsafe {
        swift_call!(function,
            in("x20") value,
            inlateout("x0") value => w0, lateout("x1") w1, lateout("x2") w2,
        );
    }
    (w0, w1, w2)
}

#[inline]
pub unsafe fn value_to_doubles2(function: *const (), value: *const ()) -> (f64, f64) {
    let pair = unsafe { value_to_doubles2_pair(function, value) };
    (pair.0, pair.1)
}

#[inline]
pub unsafe fn value_to_rect(function: *const (), value: *const ()) -> (f64, f64, f64, f64) {
    let quad = unsafe { value_to_rect_quad(function, value) };
    (quad.0, quad.1, quad.2, quad.3)
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
    "stp x20, x30, [sp, #-16]!",
    "mov x8, x1",
    "mov x20, x2",
    "blr x3",
    "ldp x20, x30, [sp], #16",
    "ret",
);

/// # Safety
///
/// As [`object_to_value`].
#[inline]
pub unsafe fn value_to_value(function: *const (), value: *const (), out: *mut ()) {
    unsafe { self_to_value_thunk(value, out, value, function) }
}

/// Returns the error the getter threw, or null.
///
/// # Safety
///
/// `out` must be uninitialized storage for what the getter returns, and is only
/// initialized when this returns null.
#[inline]
pub unsafe fn object_to_throwing_value(
    function: *const (),
    object: *const (),
    out: *mut (),
) -> *mut () {
    let error: *mut ();
    unsafe {
        swift_call!(function,
            in("x20") object, in("x0") object, in("x8") out,
            inlateout("x21") 0usize => error,
        );
    }
    error
}

/// Returns the error the method threw, or null.
///
/// # Safety
///
/// `value` must be what the method takes and `object` the instance it is called
/// on.
#[inline]
pub unsafe fn value_object_to_throwing_void(
    function: *const (),
    value: *const (),
    object: *const (),
) -> *mut () {
    let error: *mut ();
    unsafe {
        swift_call!(function,
            in("x0") value, in("x20") object,
            inlateout("x21") 0usize => error,
        );
    }
    error
}
