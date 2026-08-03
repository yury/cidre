use core::{arch::asm, ffi::c_char};

use super::RawString;

#[repr(C)]
pub struct TypeMetadata {
    _priv: [u8; 0],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValueLayout {
    pub size: usize,
    pub stride: usize,
    pub align: usize,
}

/// Medium-priority Swift task that is enqueued immediately, returns no value,
/// and consumes its opaque context without applying Swift ARC to it.
pub const ENQUEUED_DISCARDING_TASK_FLAGS: usize = 0x15 | (1 << 12) | (1 << 14) | (1 << 15);

#[link(name = "swiftCore")]
unsafe extern "C" {
    /// Declared `C_CC` in the runtime, so it needs no assembly shim.
    fn swift_retain(object: *const ()) -> *const ();

    /// Declared `C_CC` in the runtime, so it needs no assembly shim.
    fn swift_release(object: *const ());

    /// Declared `C_CC` in the runtime, so it needs no assembly shim.
    fn swift_errorRetain(error: *const ()) -> *const ();

    /// Declared `C_CC` in the runtime, so it needs no assembly shim.
    fn swift_errorRelease(error: *const ());
    fn swift_getTypeByMangledNameInContext2(
        name: *const u8,
        name_len: usize,
        context: *const (),
        args: *const *const (),
    ) -> *const TypeMetadata;
    fn swift_getOpaqueTypeMetadata(
        request: usize,
        args: *const *const (),
        descriptor: *const (),
        index: usize,
    );
    fn swift_getOpaqueTypeConformance2(args: *const *const (), descriptor: *const (), index: usize);
    fn swift_getAssociatedConformanceWitness(
        witness: *const (),
        conforming_type: *const TypeMetadata,
        associated_type: *const TypeMetadata,
        requirements: *const (),
        conformance: *const (),
    );

    fn swift_bridgeObjectRetain(object: *const ()) -> *const ();
    fn swift_bridgeObjectRelease(object: *const ());

    fn _swift_stdlib_bridgeErrorToNSError(error: *mut ()) -> *mut ();

    /// Declared `C_CC` in the runtime, so it needs no assembly shim.
    fn swift_arrayInitWithCopy(
        dst: *mut (),
        src: *const (),
        count: usize,
        metadata: *const TypeMetadata,
    );

    #[link_name = "$sSS7cStringSSSPys4Int8VG_tcfC"]
    fn swift_string_from_c_string();

    #[link_name = "$sSS5countSivg"]
    fn swift_string_count();

    #[link_name = "$sSS18_uncheckedFromUTF8ySSSRys5UInt8VGFZ"]
    fn swift_string_from_utf8();

    #[link_name = "$sSS11utf8CStrings15ContiguousArrayVys4Int8VGvg"]
    fn swift_string_utf8_c_string();

    #[link_name = "$sSS2eeoiySbSS_SStFZ"]
    fn swift_string_equal();

    #[link_name = "$ss27_allocateUninitializedArrayySayxG_BptBwlF"]
    fn swift_allocate_uninitialized_array();

    #[link_name = "$sSa5countSivg"]
    fn swift_array_count();

    #[link_name = "$sSayxSicig"]
    fn swift_array_get();

    #[link_name = "$sSaMa"]
    fn swift_array_metadata();

    #[link_name = "$ss15ContiguousArrayV5countSivg"]
    fn swift_contiguous_array_count();

    #[link_name = "$ss15ContiguousArrayVyxSicig"]
    fn swift_contiguous_array_get();

    #[link_name = "$sSbN"]
    static SWIFT_BOOL_METADATA: u8;
    #[link_name = "$sSiN"]
    static SWIFT_INT_METADATA: u8;
    #[link_name = "$sSuN"]
    static SWIFT_UINT_METADATA: u8;
    #[link_name = "$ss4Int8VN"]
    static SWIFT_INT8_METADATA: u8;
    #[link_name = "$ss5UInt8VN"]
    static SWIFT_UINT8_METADATA: u8;
    #[link_name = "$ss5Int16VN"]
    static SWIFT_INT16_METADATA: u8;
    #[link_name = "$ss6UInt16VN"]
    static SWIFT_UINT16_METADATA: u8;
    #[link_name = "$ss5Int32VN"]
    static SWIFT_INT32_METADATA: u8;
    #[link_name = "$ss6UInt32VN"]
    static SWIFT_UINT32_METADATA: u8;
    #[link_name = "$ss5Int64VN"]
    static SWIFT_INT64_METADATA: u8;
    #[link_name = "$ss6UInt64VN"]
    static SWIFT_UINT64_METADATA: u8;
    #[link_name = "$sSfN"]
    static SWIFT_FLOAT_METADATA: u8;
    #[link_name = "$sSdN"]
    static SWIFT_DOUBLE_METADATA: u8;
    #[link_name = "$sSSN"]
    static SWIFT_STRING_METADATA: u8;
}

#[link(name = "swift_Concurrency")]
unsafe extern "C" {
    fn swift_task_create();
    fn swift_task_create_common();
    fn swift_task_enqueueGlobal(task: *mut ());
}

/// Retains a native Swift object.
///
/// Returns the same pointer, as the runtime declares `FirstParamReturned`.
///
/// # Safety
///
/// `object` must be a live native Swift object.
#[inline]
pub unsafe fn object_retain(object: *const ()) -> *const () {
    unsafe { swift_retain(object) }
}

/// Releases a native Swift object.
///
/// # Safety
///
/// The caller must own a reference to `object`.
#[inline]
pub unsafe fn object_release(object: *const ()) {
    unsafe { swift_release(object) }
}

#[inline]
pub unsafe fn task_create_common(
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
pub unsafe fn task_create(
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
pub unsafe fn task_enqueue_global(task: *mut ()) {
    unsafe { swift_task_enqueueGlobal(task) }
}

/// Fills in a thrown Swift error's `NSError` representation and returns it.
///
/// On Darwin an error box is an Objective-C object already laid out as an
/// `NSError`, so the runtime populates its domain, code, and user info and hands
/// back the same pointer without retaining it. Ownership of `error` therefore
/// carries straight over to the returned `NSError`: release it exactly once, and
/// either [`error_release`] or Objective-C release will do, because
/// `swift_errorRelease` is `objc_release`.
///
/// # Safety
///
/// `error` must be a live Swift error box that the caller owns a reference to.
#[inline]
pub unsafe fn error_as_ns_error(error: *mut ()) -> *mut () {
    unsafe { _swift_stdlib_bridgeErrorToNSError(error) }
}

/// Retains a Swift error box.
///
/// # Safety
///
/// `error` must be a live Swift error box.
#[inline]
pub unsafe fn error_retain(error: *const ()) -> *const () {
    unsafe { swift_errorRetain(error) }
}

/// Releases a Swift error box.
///
/// # Safety
///
/// The caller must own a reference to `error`.
#[inline]
pub unsafe fn error_release(error: *const ()) {
    unsafe { swift_errorRelease(error) }
}

macro_rules! metadata {
    ($fn:ident, $symbol:ident) => {
        #[inline]
        pub fn $fn() -> *const TypeMetadata {
            (&raw const $symbol).cast()
        }
    };
}

metadata!(bool_metadata, SWIFT_BOOL_METADATA);
metadata!(int_metadata, SWIFT_INT_METADATA);
metadata!(uint_metadata, SWIFT_UINT_METADATA);
metadata!(int8_metadata, SWIFT_INT8_METADATA);
metadata!(uint8_metadata, SWIFT_UINT8_METADATA);
metadata!(int16_metadata, SWIFT_INT16_METADATA);
metadata!(uint16_metadata, SWIFT_UINT16_METADATA);
metadata!(int32_metadata, SWIFT_INT32_METADATA);
metadata!(uint32_metadata, SWIFT_UINT32_METADATA);
metadata!(int64_metadata, SWIFT_INT64_METADATA);
metadata!(uint64_metadata, SWIFT_UINT64_METADATA);
metadata!(float_metadata, SWIFT_FLOAT_METADATA);
metadata!(double_metadata, SWIFT_DOUBLE_METADATA);
metadata!(string_metadata, SWIFT_STRING_METADATA);

#[inline]
pub unsafe fn array_metadata(element: *const TypeMetadata) -> *const TypeMetadata {
    let metadata: *const TypeMetadata;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_array_metadata,
            inlateout("x0") 0usize => metadata,
            in("x1") element,
            clobber_abi("C"),
        );
    }
    metadata
}

#[inline]
pub unsafe fn value_witness_table(metadata: *const TypeMetadata) -> *const usize {
    unsafe { *metadata.cast::<*const usize>().sub(1) }
}

#[inline]
pub unsafe fn value_layout(metadata: *const TypeMetadata) -> ValueLayout {
    let vwt = unsafe { value_witness_table(metadata) };
    let size = unsafe { *vwt.add(8) };
    let stride = unsafe { *vwt.add(9) };
    let flags = unsafe { *vwt.add(10) };
    let align_mask = flags & 0xff;
    ValueLayout {
        size,
        stride,
        align: (align_mask + 1).max(16),
    }
}

#[inline]
pub unsafe fn destroy_value(value: *mut (), metadata: *const TypeMetadata) {
    let vwt = unsafe { value_witness_table(metadata) };
    let destroy: unsafe extern "C" fn(*mut (), *const TypeMetadata) =
        unsafe { std::mem::transmute(*vwt.add(1)) };
    unsafe { destroy(value, metadata) };
}

#[inline]
pub unsafe fn initialize_with_take(
    dst: *mut (),
    src: *mut (),
    metadata: *const TypeMetadata,
) -> *mut () {
    let vwt = unsafe { value_witness_table(metadata) };
    let init: unsafe extern "C" fn(*mut (), *mut (), *const TypeMetadata) -> *mut () =
        unsafe { std::mem::transmute(*vwt.add(4)) };
    unsafe { init(dst, src, metadata) }
}

#[inline]
pub unsafe fn initialize_with_copy(
    dst: *mut (),
    src: *const (),
    metadata: *const TypeMetadata,
) -> *mut () {
    let vwt = unsafe { value_witness_table(metadata) };
    let init: unsafe extern "C" fn(*mut (), *const (), *const TypeMetadata) -> *mut () =
        unsafe { std::mem::transmute(*vwt.add(2)) };
    unsafe { init(dst, src, metadata) }
}

/// Copies `count` contiguous values through their `initializeWithCopy` witness.
///
/// Both buffers must be laid out at the type's Swift stride.
///
/// # Safety
///
/// `dst` must be uninitialized storage for `count` values described by
/// `metadata`, and `src` must hold that many initialized ones.
#[inline]
pub unsafe fn array_initialize_with_copy(
    dst: *mut (),
    src: *const (),
    count: usize,
    metadata: *const TypeMetadata,
) {
    unsafe { swift_arrayInitWithCopy(dst, src, count, metadata) };
}

#[inline]
pub unsafe fn get_enum_tag_single_payload(
    value: *const (),
    empty_cases: u32,
    metadata: *const TypeMetadata,
) -> u32 {
    let vwt = unsafe { value_witness_table(metadata) };
    let get_tag: unsafe extern "C" fn(*const (), u32, *const TypeMetadata) -> u32 =
        unsafe { std::mem::transmute(*vwt.add(6)) };
    unsafe { get_tag(value, empty_cases, metadata) }
}

#[inline]
pub unsafe fn store_enum_tag_single_payload(
    value: *mut (),
    tag: u32,
    empty_cases: u32,
    metadata: *const TypeMetadata,
) {
    let vwt = unsafe { value_witness_table(metadata) };
    let store_tag: unsafe extern "C" fn(*mut (), u32, u32, *const TypeMetadata) =
        unsafe { std::mem::transmute(*vwt.add(7)) };
    unsafe { store_tag(value, tag, empty_cases, metadata) };
}

/// `ValueWitnessFlags::HasEnumWitnesses`.
const HAS_ENUM_WITNESSES: usize = 0x0020_0000;

/// Writes a case tag into uninitialized enum storage through the runtime's
/// `destructiveInjectEnumTag` witness.
///
/// For a no-payload case this fully initializes the value whatever layout the
/// runtime picked, so it stays correct for resilient enums whose size or spare
/// bits change between OS releases.
#[inline]
pub unsafe fn destructive_inject_enum_tag(value: *mut (), tag: u32, metadata: *const TypeMetadata) {
    let vwt = unsafe { value_witness_table(metadata) };
    let flags = unsafe { *vwt.add(10) };
    assert!(
        flags & HAS_ENUM_WITNESSES != 0,
        "Swift type must carry enum value witnesses"
    );
    let inject: unsafe extern "C" fn(*mut (), u32, *const TypeMetadata) =
        unsafe { std::mem::transmute(*vwt.add(13)) };
    unsafe { inject(value, tag, metadata) };
}

#[inline]
pub unsafe fn type_by_mangled_name(name: &str) -> *const TypeMetadata {
    unsafe {
        swift_getTypeByMangledNameInContext2(
            name.as_ptr(),
            name.len(),
            core::ptr::null(),
            core::ptr::null(),
        )
    }
}

#[inline]
pub unsafe fn type_by_mangled_name_bytes(name: &[u8]) -> *const TypeMetadata {
    unsafe {
        swift_getTypeByMangledNameInContext2(
            name.as_ptr(),
            name.len(),
            core::ptr::null(),
            core::ptr::null(),
        )
    }
}

#[inline]
pub unsafe fn opaque_type_metadata(descriptor: *const (), index: usize) -> *const TypeMetadata {
    let metadata: *const TypeMetadata;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_getOpaqueTypeMetadata,
            in("x0") 0usize,
            in("x1") core::ptr::null::<*const ()>(),
            in("x2") descriptor,
            in("x3") index,
            lateout("x0") metadata,
            clobber_abi("C"),
        );
    }
    metadata
}

#[inline]
pub unsafe fn opaque_type_conformance(descriptor: *const (), index: usize) -> *const () {
    let witness: *const ();
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_getOpaqueTypeConformance2,
            in("x0") core::ptr::null::<*const ()>(),
            in("x1") descriptor,
            in("x2") index,
            lateout("x0") witness,
            clobber_abi("C"),
        );
    }
    witness
}

#[inline]
pub unsafe fn associated_conformance_witness(
    witness: *const (),
    conforming_type: *const TypeMetadata,
    associated_type: *const TypeMetadata,
    requirements: *const (),
    conformance: *const (),
) -> *const () {
    let result: *const ();
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_getAssociatedConformanceWitness,
            inlateout("x0") witness => result,
            in("x1") conforming_type,
            in("x2") associated_type,
            in("x3") requirements,
            in("x4") conformance,
            clobber_abi("C"),
        );
    }
    result
}

#[inline]
pub unsafe fn bridge_object_retain(object: usize) -> usize {
    let retained: usize;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_bridgeObjectRetain,
            inlateout("x0") object => retained,
            clobber_abi("C"),
        );
    }
    retained
}

#[inline]
pub unsafe fn bridge_object_release(object: usize) {
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_bridgeObjectRelease,
            in("x0") object,
            clobber_abi("C"),
        );
    }
}

#[inline]
pub unsafe fn string_from_c_str(ptr: *const c_char) -> RawString {
    let word0: usize;
    let word1: usize;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_string_from_c_string,
            inlateout("x0") ptr as usize => word0,
            lateout("x1") word1,
            clobber_abi("C"),
        );
    }
    RawString { word0, word1 }
}

#[inline]
pub unsafe fn string_count(string: RawString) -> isize {
    let count: isize;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_string_count,
            inlateout("x0") string.word0 => count,
            in("x1") string.word1,
            clobber_abi("C"),
        );
    }
    count
}

#[inline]
pub unsafe fn string_from_utf8(bytes: &[u8]) -> RawString {
    let word0: usize;
    let word1: usize;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_string_from_utf8,
            inlateout("x0") bytes.as_ptr() => word0,
            inlateout("x1") bytes.len() => word1,
            clobber_abi("C"),
        );
    }
    RawString { word0, word1 }
}

#[inline]
pub unsafe fn string_utf8_c_string(string: RawString) -> *mut () {
    let storage: *mut ();
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_string_utf8_c_string,
            inlateout("x0") string.word0 => storage,
            in("x1") string.word1,
            clobber_abi("C"),
        );
    }
    storage
}

#[inline]
pub unsafe fn string_equal(lhs: RawString, rhs: RawString) -> bool {
    let equal: usize;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_string_equal,
            inlateout("x0") lhs.word0 => equal,
            in("x1") lhs.word1,
            in("x2") rhs.word0,
            in("x3") rhs.word1,
            clobber_abi("C"),
        );
    }
    equal & 1 == 1
}

#[inline]
pub unsafe fn allocate_uninitialized_array(
    len: usize,
    element: *const TypeMetadata,
) -> (*mut (), *mut ()) {
    let storage: *mut ();
    let elements: *mut ();
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_allocate_uninitialized_array,
            inlateout("x0") len => storage,
            in("x1") element,
            lateout("x1") elements,
            clobber_abi("C"),
        );
    }
    (storage, elements)
}

#[inline]
pub unsafe fn array_count(array: *const (), element: *const TypeMetadata) -> isize {
    let count: isize;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_array_count,
            inlateout("x0") array => count,
            in("x1") element,
            clobber_abi("C"),
        );
    }
    count
}

#[inline]
pub unsafe fn array_get(
    array: *const (),
    index: isize,
    out: *mut (),
    element: *const TypeMetadata,
) {
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_array_get,
            in("x8") out,
            in("x0") index,
            in("x1") array,
            in("x2") element,
            clobber_abi("C"),
        );
    }
}

#[inline]
pub unsafe fn contiguous_array_count(array: *const (), element: *const TypeMetadata) -> isize {
    let count: isize;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_contiguous_array_count,
            inlateout("x0") array => count,
            in("x1") element,
            clobber_abi("C"),
        );
    }
    count
}

#[inline]
pub unsafe fn contiguous_array_get(
    array: *const (),
    index: isize,
    out: *mut (),
    element: *const TypeMetadata,
) {
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_contiguous_array_get,
            in("x8") out,
            in("x0") index,
            in("x1") array,
            in("x2") element,
            clobber_abi("C"),
        );
    }
}

#[inline]
pub unsafe fn call_int_to_int(function: *const (), arg: isize) -> isize {
    let result: isize;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            inlateout("x0") arg => result,
            clobber_abi("C"),
        );
    }
    result
}

#[inline]
pub unsafe fn call_string_to_int(function: *const (), arg: RawString) -> isize {
    let result: isize;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            inlateout("x0") arg.word0 => result,
            in("x1") arg.word1,
            clobber_abi("C"),
        );
    }
    result
}

#[inline]
pub unsafe fn call_array_to_int(function: *const (), array: *const ()) -> isize {
    let result: isize;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            inlateout("x0") array => result,
            clobber_abi("C"),
        );
    }
    result
}

#[inline]
pub unsafe fn call0_string(function: *const ()) -> RawString {
    let word0: usize;
    let word1: usize;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            lateout("x0") word0,
            lateout("x1") word1,
            clobber_abi("C"),
        );
    }
    RawString { word0, word1 }
}

#[inline]
pub unsafe fn call0_array(function: *const ()) -> *mut () {
    let array: *mut ();
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            lateout("x0") array,
            clobber_abi("C"),
        );
    }
    array
}

#[inline]
pub unsafe fn call0_object(function: *const ()) -> *mut () {
    let object: *mut ();
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            lateout("x0") object,
            clobber_abi("C"),
        );
    }
    object
}

#[inline]
pub unsafe fn call_static0_object(function: *const (), type_metadata: *const ()) -> *mut () {
    let object: *mut ();
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") type_metadata,
            lateout("x0") object,
            clobber_abi("C"),
        );
    }
    object
}

#[inline]
pub unsafe fn call_static0_bool(function: *const (), type_metadata: *const ()) -> bool {
    let result: usize;
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") type_metadata,
            lateout("x0") result,
            clobber_abi("C"),
        );
    }
    result & 1 == 1
}

#[inline]
pub unsafe fn call_static_value_bool_to_object(
    function: *const (),
    type_metadata: *const (),
    value: *mut (),
    boolean: bool,
) -> *mut () {
    let object: *mut ();
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") type_metadata,
            inlateout("x0") value => object,
            in("x1") boolean as usize,
            clobber_abi("C"),
        );
    }
    object
}

#[inline]
pub unsafe fn call_static_values_to_object(
    function: *const (),
    type_metadata: *const (),
    value0: *mut (),
    value1: *mut (),
) -> *mut () {
    let object: *mut ();
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") type_metadata,
            inlateout("x0") value0 => object,
            in("x1") value1,
            clobber_abi("C"),
        );
    }
    object
}

#[inline]
pub unsafe fn call_static_array_value_to_object(
    function: *const (),
    type_metadata: *const (),
    array: *mut (),
    value: *mut (),
) -> *mut () {
    let object: *mut ();
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") type_metadata,
            inlateout("x0") array => object,
            in("x1") value,
            clobber_abi("C"),
        );
    }
    object
}

#[inline]
pub unsafe fn call_string_to_value(function: *const (), string: RawString, out: *mut ()) {
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x0") string.word0,
            in("x1") string.word1,
            in("x8") out,
            clobber_abi("C"),
        );
    }
}

#[inline]
pub unsafe fn call0_value(function: *const (), out: *mut ()) {
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x8") out,
            clobber_abi("C"),
        );
    }
}

#[inline]
pub unsafe fn call_make_async_iterator(
    function: *const (),
    sequence: *mut (),
    sequence_metadata: *const TypeMetadata,
    witness: *const (),
    iterator: *mut (),
) {
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") sequence,
            in("x0") sequence_metadata,
            in("x1") witness,
            in("x8") iterator,
            clobber_abi("C"),
        );
    }
}

#[inline]
pub unsafe fn call_object_to_bool(function: *const (), object: *const ()) -> bool {
    let result: usize;
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") object,
            inlateout("x0") object as usize => result,
            clobber_abi("C"),
        );
    }
    result & 1 == 1
}

#[inline]
pub unsafe fn call_object_to_string(function: *const (), object: *const ()) -> RawString {
    let word0: usize;
    let word1: usize;
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") object,
            in("x0") object,
            lateout("x0") word0,
            lateout("x1") word1,
            clobber_abi("C"),
        );
    }
    RawString { word0, word1 }
}

#[inline]
pub unsafe fn call_object_to_rect(function: *const (), object: *const ()) -> (f64, f64, f64, f64) {
    let x: f64;
    let y: f64;
    let width: f64;
    let height: f64;
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") object,
            in("x0") object,
            lateout("d0") x,
            lateout("d1") y,
            lateout("d2") width,
            lateout("d3") height,
            clobber_abi("C"),
        );
    }
    (x, y, width, height)
}

#[inline]
pub unsafe fn call_value_to_int(function: *const (), value: *const ()) -> isize {
    let result: isize;
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") value,
            inlateout("x0") value as isize => result,
            clobber_abi("C"),
        );
    }
    result
}

#[inline]
pub unsafe fn call_value_to_bool(function: *const (), value: *const ()) -> bool {
    let result: usize;
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") value,
            inlateout("x0") value as usize => result,
            clobber_abi("C"),
        );
    }
    result & 1 == 1
}

#[inline]
pub unsafe fn call_value_to_object(function: *const (), value: *const ()) -> *mut () {
    let result: usize;
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") value,
            inlateout("x0") value as usize => result,
            clobber_abi("C"),
        );
    }
    result as *mut ()
}

#[inline]
pub unsafe fn call_value_to_string(function: *const (), value: *const ()) -> RawString {
    let word0: usize;
    let word1: usize;
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") value,
            in("x0") value,
            lateout("x0") word0,
            lateout("x1") word1,
            clobber_abi("C"),
        );
    }
    RawString { word0, word1 }
}

#[inline]
pub unsafe fn call_object_to_value(function: *const (), object: *const (), out: *mut ()) {
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") object,
            in("x0") object,
            in("x8") out,
            clobber_abi("C"),
        );
    }
}

#[inline]
pub unsafe fn call_value_to_value(function: *const (), value: *const (), out: *mut ()) {
    unsafe {
        asm!(
            "stp x20, x19, [sp, #-16]!",
            "mov x20, x10",
            "blr x9",
            "ldp x20, x19, [sp], #16",
            in("x9") function,
            in("x10") value,
            in("x0") value,
            in("x8") out,
            clobber_abi("C"),
        );
    }
}

#[inline]
pub unsafe fn call_object_to_throwing_value(
    function: *const (),
    object: *const (),
    out: *mut (),
) -> *mut () {
    let error: *mut ();
    unsafe {
        asm!(
            "stp x20, x21, [sp, #-16]!",
            "mov x20, x10",
            "mov x21, #0",
            "blr x9",
            "mov x11, x21",
            "ldp x20, x21, [sp], #16",
            in("x9") function,
            in("x10") object,
            in("x0") object,
            in("x8") out,
            lateout("x11") error,
            clobber_abi("C"),
        );
    }
    error
}
