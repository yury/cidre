use core::{arch::asm, ffi::c_char};

use super::{Int, RawString};

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

#[link(name = "swiftCore")]
unsafe extern "C" {
    fn swift_retain(object: *const ()) -> *const ();
    fn swift_release(object: *const ());
    fn swift_errorRelease(error: *const ());
    fn swift_getTypeByMangledNameInContext2(
        name: *const u8,
        name_len: usize,
        context: *const (),
        args: *const *const (),
    ) -> *const TypeMetadata;

    fn swift_bridgeObjectRetain(object: *const ()) -> *const ();
    fn swift_bridgeObjectRelease(object: *const ());

    #[link_name = "$sSS7cStringSSSPys4Int8VG_tcfC"]
    fn swift_string_from_c_string();

    #[link_name = "$sSS5countSivg"]
    fn swift_string_count();

    #[link_name = "$ss27_allocateUninitializedArrayySayxG_BptBwlF"]
    fn swift_allocate_uninitialized_array();

    #[link_name = "$sSa5countSivg"]
    fn swift_array_count();

    #[link_name = "$sSiN"]
    static SWIFT_INT_METADATA: u8;
}

#[link(name = "swift_Concurrency")]
unsafe extern "C" {
    fn swift_task_create();
    fn swift_task_create_common();
    fn swift_task_enqueueGlobal(task: *mut ());
}

#[inline]
pub unsafe fn object_retain(object: *const ()) -> *const () {
    let retained: *const ();
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_retain,
            inlateout("x0") object => retained,
            clobber_abi("C"),
        );
    }
    retained
}

#[inline]
pub unsafe fn object_release(object: *const ()) {
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_release,
            in("x0") object,
            clobber_abi("C"),
        );
    }
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

#[inline]
pub unsafe fn error_release(error: *const ()) {
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_errorRelease,
            in("x0") error,
            clobber_abi("C"),
        );
    }
}

#[inline]
pub fn int_metadata() -> *const TypeMetadata {
    (&raw const SWIFT_INT_METADATA).cast()
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
pub unsafe fn string_count(string: RawString) -> Int {
    let count: Int;
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
pub unsafe fn array_count(array: *const (), element: *const TypeMetadata) -> Int {
    let count: Int;
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
pub unsafe fn call_int_to_int(function: *const (), arg: Int) -> Int {
    let result: Int;
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
pub unsafe fn call_string_to_int(function: *const (), arg: RawString) -> Int {
    let result: Int;
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
pub unsafe fn call_array_to_int(function: *const (), array: *const ()) -> Int {
    let result: Int;
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
pub unsafe fn call_value_to_int(function: *const (), value: *const ()) -> Int {
    let result: Int;
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
