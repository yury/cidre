//! Calling Swift functions from Rust.
//!
//! Swift's calling convention is the C one plus three registers Rust cannot
//! name in a function type: `x20` carries `self`, `x21` carries a thrown error,
//! and `x8` points at storage for a value returned indirectly. So every call
//! goes through the one assembly block below, told which registers to fill.
//!
//! It used to be one hand-written `asm!` per argument shape — forty of them,
//! named things like `static_array_value_to_object` — which is why the
//! shapes below are just data now.

use core::arch::asm;

use super::{RawString, TypeMetadata};

/// The registers a Swift call takes its arguments in.
///
/// Every register is passed whether or not the callee reads it: a function
/// ignores the argument registers past its own arity, which is what lets one
/// assembly block serve every call.
#[derive(Clone, Copy, Default)]
pub struct Call {
    /// `x0`–`x6`.
    args: [usize; 7],
    /// `d0`–`d3`.
    doubles: [f64; 4],
    /// `x8`: where a value returned indirectly is written.
    indirect: *mut (),
    /// `x20`: the instance for a method, the metadata for a static.
    swift_self: *const (),
}

impl Call {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// The instance a method is called on, or a static method's metadata.
    ///
    /// Swift also passes a struct's own storage address as the first argument
    /// to some entry points, so a value method sets both.
    #[inline]
    pub fn swift_self(mut self, value: *const ()) -> Self {
        self.swift_self = value;
        self
    }

    /// The `self` of a value type, which reaches the callee both ways.
    #[inline]
    pub fn value_self(self, value: *const ()) -> Self {
        self.swift_self(value).ptr(0, value)
    }

    #[inline]
    pub fn word(mut self, index: usize, value: usize) -> Self {
        self.args[index] = value;
        self
    }

    #[inline]
    pub fn int(self, index: usize, value: isize) -> Self {
        self.word(index, value as usize)
    }

    #[inline]
    pub fn bool(self, index: usize, value: bool) -> Self {
        self.word(index, value as usize)
    }

    #[inline]
    pub fn ptr(self, index: usize, value: *const ()) -> Self {
        self.word(index, value as usize)
    }

    /// A Swift `String`, which is two words.
    #[inline]
    pub fn string(self, index: usize, value: RawString) -> Self {
        self.word(index, value.word0).word(index + 1, value.word1)
    }

    #[inline]
    pub fn double(mut self, index: usize, value: f64) -> Self {
        self.doubles[index] = value;
        self
    }

    /// A `CGRect` and the other four-`Double` aggregates, which the ABI passes
    /// as their scalars.
    #[inline]
    pub fn doubles(mut self, values: &[f64]) -> Self {
        self.doubles[..values.len()].copy_from_slice(values);
        self
    }

    /// Storage for a value the callee returns indirectly.
    #[inline]
    pub fn indirect(mut self, out: *mut ()) -> Self {
        self.indirect = out;
        self
    }
}

/// What a Swift call left in the registers it returns through.
#[derive(Clone, Copy)]
pub struct CallResult {
    words: [usize; 4],
    doubles: [f64; 4],
    error: *mut (),
}

impl CallResult {
    #[inline]
    pub fn word(&self, index: usize) -> usize {
        self.words[index]
    }

    #[inline]
    pub fn int(&self) -> isize {
        self.words[0] as isize
    }

    #[inline]
    pub fn bool(&self) -> bool {
        self.words[0] & 1 != 0
    }

    #[inline]
    pub fn ptr(&self) -> *mut () {
        self.words[0] as *mut ()
    }

    #[inline]
    pub fn string(&self) -> RawString {
        RawString {
            word0: self.words[0],
            word1: self.words[1],
        }
    }

    #[inline]
    pub fn double(&self, index: usize) -> f64 {
        self.doubles[index]
    }

    /// The error box the call threw, or null.
    ///
    /// A function that cannot throw leaves `x21` as it found it, so this is
    /// null for those without the caller having to say which is which.
    #[inline]
    pub fn error(&self) -> *mut () {
        self.error
    }
}

/// Calls a Swift function with the given registers filled in.
///
/// # Safety
///
/// `function` must be a Swift entry point whose parameters are what `args`
/// fills in, still owned as its convention expects, and any value it returns
/// indirectly must have storage to be written into.
#[inline]
pub unsafe fn call(function: *const (), args: Call) -> CallResult {
    let (w0, w1, w2, w3): (usize, usize, usize, usize);
    let (d0, d1, d2, d3): (f64, f64, f64, f64);
    let error: *mut ();
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            inlateout("x0") args.args[0] => w0,
            inlateout("x1") args.args[1] => w1,
            inlateout("x2") args.args[2] => w2,
            inlateout("x3") args.args[3] => w3,
            in("x4") args.args[4],
            in("x5") args.args[5],
            in("x6") args.args[6],
            in("x8") args.indirect,
            in("x20") args.swift_self,
            inlateout("x21") 0usize => error,
            inlateout("d0") args.doubles[0] => d0,
            inlateout("d1") args.doubles[1] => d1,
            inlateout("d2") args.doubles[2] => d2,
            inlateout("d3") args.doubles[3] => d3,
            clobber_abi("C"),
        );
    }
    CallResult {
        words: [w0, w1, w2, w3],
        doubles: [d0, d1, d2, d3],
        error,
    }
}

/// Calls a generic type's metadata accessor that takes one type argument.
///
/// # Safety
///
/// `accessor` must be the metadata accessor of a type with exactly one generic
/// parameter, and `arg` its argument's metadata.
#[inline]
pub unsafe fn generic_metadata1(
    accessor: *const (),
    arg: *const TypeMetadata,
) -> *const TypeMetadata {
    unsafe {
        call(accessor, Call::new().word(0, 0).ptr(1, arg.cast())).word(0) as *const TypeMetadata
    }
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
    unsafe {
        call(
            function,
            Call::new()
                .swift_self(value)
                .ptr(0, metadata.cast())
                .indirect(out),
        );
    }
}

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
    unsafe {
        let result = call(
            function,
            Call::new().swift_self(value).ptr(0, metadata.cast()),
        );
        (
            result.word(0) as u64,
            result.word(1) as u64,
            result.word(2) as u64,
        )
    }
}

#[inline]
pub unsafe fn int_to_int(function: *const (), arg: isize) -> isize {
    unsafe { call(function, Call::new().int(0, arg)).int() }
}

#[inline]
pub unsafe fn double_to_words2(function: *const (), value: f64) -> (u64, u64) {
    unsafe {
        let result = call(function, Call::new().double(0, value));
        (result.word(0) as u64, result.word(1) as u64)
    }
}

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
    unsafe {
        let result = call(
            function,
            Call::new()
                .doubles(&[vector.0, vector.1, vector.2])
                .word(0, duration.0 as usize)
                .word(1, duration.1 as usize)
                .bool(2, relative)
                .swift_self(object),
        );
        (result.ptr(), result.error())
    }
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
    unsafe {
        let result = call(
            function,
            Call::new()
                .doubles(&[rotation.0, rotation.1, rotation.2, rotation.3])
                .word(0, duration.0 as usize)
                .word(1, duration.1 as usize)
                .bool(2, relative)
                .swift_self(object),
        );
        (result.ptr(), result.error())
    }
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
    unsafe {
        call(
            function,
            Call::new()
                .doubles(&[values.0, values.1, values.2])
                .indirect(out),
        )
        .error()
    }
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
    unsafe {
        call(
            function,
            Call::new()
                .ptr(0, first)
                .ptr(1, second)
                .ptr(2, third)
                .indirect(out),
        );
    }
}

#[inline]
pub unsafe fn static0_object(function: *const (), type_metadata: *const ()) -> *mut () {
    unsafe { call(function, Call::new().swift_self(type_metadata)).ptr() }
}

#[inline]
pub unsafe fn static0_bool(function: *const (), type_metadata: *const ()) -> bool {
    unsafe { call(function, Call::new().swift_self(type_metadata)).bool() }
}

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
        call(
            function,
            Call::new()
                .swift_self(type_metadata)
                .ptr(0, value)
                .bool(1, flag),
        )
        .ptr()
    }
}

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
    unsafe {
        call(
            function,
            Call::new()
                .swift_self(type_metadata)
                .ptr(0, first)
                .ptr(1, second),
        )
        .ptr()
    }
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
    unsafe {
        call(
            function,
            Call::new()
                .swift_self(type_metadata)
                .ptr(0, array)
                .ptr(1, value),
        )
        .ptr()
    }
}

/// # Safety
///
/// `string` must be owned as the callee expects, and `out` uninitialized
/// storage for what it returns.
#[inline]
pub unsafe fn string_to_value(function: *const (), string: RawString, out: *mut ()) {
    unsafe {
        call(function, Call::new().string(0, string).indirect(out));
    }
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
        call(
            function,
            Call::new()
                .int(0, integer)
                .ptr(1, value)
                .doubles(&[rect.0, rect.1, rect.2, rect.3])
                .ptr(2, trailing_value)
                .indirect(out),
        );
    }
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
        call(
            function,
            Call::new()
                .ptr(0, device_type)
                .int(1, position)
                .ptr(2, orientation)
                .ptr(3, intrinsics)
                .word(4, reference_dimensions.0 as usize)
                .word(5, reference_dimensions.1 as usize)
                .word(6, reference_dimensions.2 as usize)
                .indirect(out),
        );
    }
}

/// # Safety
///
/// `out` must be uninitialized storage for what the getter returns.
#[inline]
pub unsafe fn to_value(function: *const (), out: *mut ()) {
    unsafe {
        call(function, Call::new().indirect(out));
    }
}

#[inline]
pub unsafe fn object_to_bool(function: *const (), object: *const ()) -> bool {
    unsafe { call(function, Call::new().swift_self(object)).bool() }
}

#[inline]
pub unsafe fn object_to_int(function: *const (), object: *const ()) -> isize {
    unsafe { call(function, Call::new().swift_self(object)).int() }
}

/// A static `==`, which takes both operands as arguments.
///
/// # Safety
///
/// Both must be values of the type the operator belongs to.
#[inline]
pub unsafe fn objects_to_bool(function: *const (), lhs: *const (), rhs: *const ()) -> bool {
    unsafe { call(function, Call::new().ptr(0, lhs).ptr(1, rhs)).bool() }
}

#[inline]
pub unsafe fn object_to_string(function: *const (), object: *const ()) -> RawString {
    unsafe { call(function, Call::new().swift_self(object)).string() }
}

#[inline]
pub unsafe fn object_to_rect(function: *const (), object: *const ()) -> (f64, f64, f64, f64) {
    unsafe {
        let result = call(function, Call::new().swift_self(object));
        (
            result.double(0),
            result.double(1),
            result.double(2),
            result.double(3),
        )
    }
}

#[inline]
pub unsafe fn value_to_int(function: *const (), value: *const ()) -> isize {
    unsafe { call(function, Call::new().value_self(value)).int() }
}

#[inline]
pub unsafe fn value_to_bool(function: *const (), value: *const ()) -> bool {
    unsafe { call(function, Call::new().value_self(value)).bool() }
}

#[inline]
pub unsafe fn value_to_double(function: *const (), value: *const ()) -> f64 {
    unsafe { call(function, Call::new().value_self(value)).double(0) }
}

#[inline]
pub unsafe fn value_to_words3(function: *const (), value: *const ()) -> (u64, u64, u64) {
    unsafe {
        let result = call(function, Call::new().value_self(value));
        (
            result.word(0) as u64,
            result.word(1) as u64,
            result.word(2) as u64,
        )
    }
}

#[inline]
pub unsafe fn value_to_doubles2(function: *const (), value: *const ()) -> (f64, f64) {
    unsafe {
        let result = call(function, Call::new().value_self(value));
        (result.double(0), result.double(1))
    }
}

#[inline]
pub unsafe fn value_to_rect(function: *const (), value: *const ()) -> (f64, f64, f64, f64) {
    unsafe {
        let result = call(function, Call::new().value_self(value));
        (
            result.double(0),
            result.double(1),
            result.double(2),
            result.double(3),
        )
    }
}

#[inline]
pub unsafe fn value_to_object(function: *const (), value: *const ()) -> *mut () {
    unsafe { call(function, Call::new().value_self(value)).ptr() }
}

#[inline]
pub unsafe fn value_to_doubles3(function: *const (), value: *const ()) -> [f64; 3] {
    unsafe {
        let result = call(function, Call::new().value_self(value));
        [result.double(0), result.double(1), result.double(2)]
    }
}

#[inline]
pub unsafe fn value_to_string(function: *const (), value: *const ()) -> RawString {
    unsafe { call(function, Call::new().value_self(value)).string() }
}

/// # Safety
///
/// `out` must be uninitialized storage for what the getter returns.
#[inline]
pub unsafe fn object_to_value(function: *const (), object: *const (), out: *mut ()) {
    unsafe {
        call(function, Call::new().value_self(object).indirect(out));
    }
}

/// # Safety
///
/// As [`object_to_value`].
#[inline]
pub unsafe fn value_to_value(function: *const (), value: *const (), out: *mut ()) {
    unsafe {
        call(function, Call::new().value_self(value).indirect(out));
    }
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
    unsafe { call(function, Call::new().value_self(object).indirect(out)).error() }
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
    unsafe { call(function, Call::new().ptr(0, value).swift_self(object)).error() }
}
