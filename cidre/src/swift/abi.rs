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
    fn swift_getObjCClassMetadata(class: *const ()) -> *const TypeMetadata;
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

    fn swift_getWitnessTable(
        conformance: *const (),
        type_metadata: *const TypeMetadata,
        instantiation_args: *const *const (),
    ) -> *const ();

    #[link_name = "$sSh12arrayLiteralShyxGxd_tcfC"]
    fn swift_set_from_array_literal();

    #[link_name = "$sSDyq_Sgxcig"]
    fn swift_dictionary_subscript();

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

    #[link_name = "$sSqMa"]
    fn swift_optional_metadata();

    #[link_name = "$sShMa"]
    fn swift_set_metadata();

    #[link_name = "$sSDMa"]
    fn swift_dictionary_metadata();

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

    /// The immortal storage every empty `Dictionary` shares, which is what
    /// Swift's own `[:]` literal is.
    static _swiftEmptyDictionarySingleton: u8;

    /// The same, for `Set`.
    static _swiftEmptySetSingleton: u8;
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
pub unsafe fn objc_class_metadata(class: *const ()) -> *const TypeMetadata {
    unsafe { swift_getObjCClassMetadata(class) }
}

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

/// The storage word of an empty `Dictionary`.
#[inline]
pub fn empty_dictionary_storage() -> *mut () {
    (&raw const _swiftEmptyDictionarySingleton)
        .cast_mut()
        .cast()
}

/// The storage word of an empty `Set`.
#[inline]
pub fn empty_set_storage() -> *mut () {
    (&raw const _swiftEmptySetSingleton).cast_mut().cast()
}

/// Returns `Set<Element>`'s metadata from the standard library's generic
/// metadata accessor.
///
/// # Safety
///
/// `element` must be metadata for a type that conforms to `Hashable`, which is
/// what `Set` constrains its parameter to.
#[inline]
pub unsafe fn set_metadata(element: *const TypeMetadata) -> *const TypeMetadata {
    let metadata: *const TypeMetadata;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_set_metadata,
            inlateout("x0") 0usize => metadata,
            in("x1") element,
            clobber_abi("C"),
        );
    }
    metadata
}

/// Returns `Dictionary<Key, Value>`'s metadata from the standard library's
/// generic metadata accessor.
///
/// # Safety
///
/// `key` must be metadata for a type that conforms to `Hashable`, which is what
/// `Dictionary` constrains its key parameter to.
#[inline]
pub unsafe fn dictionary_metadata(
    key: *const TypeMetadata,
    value: *const TypeMetadata,
) -> *const TypeMetadata {
    let metadata: *const TypeMetadata;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_dictionary_metadata,
            inlateout("x0") 0usize => metadata,
            in("x1") key,
            in("x2") value,
            clobber_abi("C"),
        );
    }
    metadata
}

/// Returns `Optional<Wrapped>`'s metadata from the standard library's generic
/// metadata accessor.
///
/// This replaces spelling every optional out as a mangled `…Sg` name.
/// Instantiates a protocol witness table from its conformance descriptor.
///
/// Bindings need this because a framework exports the descriptor but not the
/// table itself; the table is built lazily by whoever uses the conformance.
///
/// # Safety
///
/// `conformance` must describe a conformance of the type `type_metadata`
/// describes, and the conformance must need no instantiation arguments.
#[inline]
pub unsafe fn witness_table(
    conformance: *const (),
    type_metadata: *const TypeMetadata,
) -> *const () {
    unsafe { swift_getWitnessTable(conformance, type_metadata, core::ptr::null()) }
}

/// Builds a `Set<Element>` from an array, consuming it.
///
/// This is `Set.init(arrayLiteral:)`, whose variadic parameter is an array.
///
/// # Safety
///
/// `array` must be an owned `Array<Element>`, and `metadata` and `witness` must
/// describe that element type and its `Hashable` conformance.
#[inline]
pub unsafe fn set_from_array(
    array: *mut (),
    metadata: *const TypeMetadata,
    witness: *const (),
) -> *mut () {
    let set: *mut ();
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_set_from_array_literal,
            inlateout("x0") array => set,
            in("x1") metadata,
            in("x2") witness,
            clobber_abi("C"),
        );
    }
    set
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
    let metadata: *const TypeMetadata;
    unsafe {
        asm!(
            "blr {f}",
            f = in(reg) accessor,
            inlateout("x0") 0usize => metadata,
            in("x1") arg,
            clobber_abi("C"),
        );
    }
    metadata
}

/// Calls a member of a generic type that returns its value indirectly.
///
/// Unlike [`call_value_to_value`], the callee also needs the generic context,
/// which is the enclosing type's metadata.
///
/// # Safety
///
/// `function` must be a member of the type `metadata` describes, `value` a
/// valid instance of it, and `out` uninitialized storage for the result.
#[inline]
pub unsafe fn call_generic_value_to_value(
    function: *const (),
    value: *const (),
    metadata: *const TypeMetadata,
    out: *mut (),
) {
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
            in("x0") metadata,
            in("x8") out,
            clobber_abi("C"),
        );
    }
}

/// Calls a member of a generic type that returns three words directly, such as
/// a `CMTime`.
///
/// # Safety
///
/// As [`call_generic_value_to_value`], and the member must return exactly three
/// words in registers.
#[inline]
pub unsafe fn call_generic_value_to_words3(
    function: *const (),
    value: *const (),
    metadata: *const TypeMetadata,
) -> [usize; 3] {
    let (w0, w1, w2);
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
            inlateout("x0") metadata => w0,
            lateout("x1") w1,
            lateout("x2") w2,
            clobber_abi("C"),
        );
    }
    [w0, w1, w2]
}

/// Looks a key up in a `Dictionary<Key, Value>`, writing `Value?` to `out`.
///
/// # Safety
///
/// `key` must be a valid `Key`, `dictionary` a valid `Dictionary<Key, Value>`,
/// the two metadata arguments must describe `Key` and `Value`, `witness` must
/// be `Key`'s `Hashable` conformance, and `out` must be uninitialized storage
/// for `Value?`.
#[inline]
#[allow(clippy::too_many_arguments)]
pub unsafe fn dictionary_get(
    key: *const (),
    dictionary: *const (),
    key_metadata: *const TypeMetadata,
    value_metadata: *const TypeMetadata,
    witness: *const (),
    out: *mut (),
) {
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_dictionary_subscript,
            in("x0") key,
            in("x1") dictionary,
            in("x2") key_metadata,
            in("x3") value_metadata,
            in("x4") witness,
            in("x8") out,
            clobber_abi("C"),
        );
    }
}

#[inline]
pub unsafe fn optional_metadata(wrapped: *const TypeMetadata) -> *const TypeMetadata {
    let metadata: *const TypeMetadata;
    unsafe {
        asm!(
            "bl {f}",
            f = sym swift_optional_metadata,
            inlateout("x0") 0usize => metadata,
            in("x1") wrapped,
            clobber_abi("C"),
        );
    }
    metadata
}

#[inline]
pub unsafe fn value_witness_table(metadata: *const TypeMetadata) -> *const usize {
    unsafe { *metadata.cast::<*const usize>().sub(1) }
}

/// A process-lifetime cache for one pointer the Swift runtime resolves.
///
/// Swift emits a cache word beside every mangled type reference and only calls
/// the runtime while that word is still empty; resolving a type otherwise means
/// re-demangling its name through the runtime's locked type cache on every use.
/// A conformance is the same story: the runtime builds the witness table on
/// demand and every call that carries one would otherwise rebuild it.
pub struct RuntimeCache<T>(core::sync::atomic::AtomicPtr<T>);

/// The cache for one Swift type's metadata.
pub type MetadataCache = RuntimeCache<TypeMetadata>;

/// The cache for one protocol conformance's witness table.
pub type WitnessCache = RuntimeCache<()>;

impl<T> RuntimeCache<T> {
    #[allow(clippy::new_without_default)]
    pub const fn new() -> Self {
        Self(core::sync::atomic::AtomicPtr::new(core::ptr::null_mut()))
    }

    /// Returns the pointer, calling `resolve` only on the first use.
    ///
    /// Relaxed is enough, as it is for the equivalent Swift stub: `resolve`
    /// returns the same pointer every time, so a race just resolves twice, and
    /// reading through the pointer is an address-dependent load from storage the
    /// runtime published before it ever handed it out.
    ///
    /// A failed lookup stays uncached, so callers keep seeing null and assert
    /// rather than the cache making the failure permanent.
    #[inline]
    pub fn get(&self, resolve: impl FnOnce() -> *const T) -> *const T {
        use core::sync::atomic::Ordering::Relaxed;

        let cached = self.0.load(Relaxed);
        if !cached.is_null() {
            return cached;
        }

        let resolved = resolve();
        if !resolved.is_null() {
            self.0.store(resolved.cast_mut(), Relaxed);
        }
        resolved
    }
}

/// A Swift type's metadata paired with its value witness table.
///
/// The free functions below re-derive the witness table from the metadata on
/// every call. Swift instead loads it once and then indexes it directly for the
/// rest of a loop — its `for await` bodies keep both pointers in the async
/// context and never look at the metadata header again. This is that pair, for
/// the paths that touch the same type once per element.
#[derive(Clone, Copy, Debug)]
pub struct ValueWitnesses {
    metadata: *const TypeMetadata,
    vwt: *const usize,
}

// The tables are immutable process-lifetime runtime data.
unsafe impl Send for ValueWitnesses {}
unsafe impl Sync for ValueWitnesses {}

impl ValueWitnesses {
    /// # Safety
    ///
    /// `metadata` must be metadata the Swift runtime published, and it must
    /// stay valid for as long as this value is used.
    #[inline]
    pub unsafe fn new(metadata: *const TypeMetadata) -> Self {
        assert!(!metadata.is_null(), "Swift type metadata must exist");
        Self {
            metadata,
            vwt: unsafe { value_witness_table(metadata) },
        }
    }

    #[inline]
    pub fn metadata(&self) -> *const TypeMetadata {
        self.metadata
    }

    #[inline]
    pub fn layout(&self) -> ValueLayout {
        let size = unsafe { *self.vwt.add(8) };
        let stride = unsafe { *self.vwt.add(9) };
        let flags = unsafe { *self.vwt.add(10) };
        ValueLayout {
            size,
            stride,
            align: ((flags & 0xff) + 1).max(16),
        }
    }

    #[inline]
    pub fn is_bitwise_takable(&self) -> bool {
        unsafe { *self.vwt.add(10) & IS_NON_BITWISE_TAKABLE == 0 }
    }

    /// # Safety
    ///
    /// `value` must be an initialized value of this type, and the caller must
    /// own it.
    #[inline]
    pub unsafe fn destroy(&self, value: *mut ()) {
        let destroy: unsafe extern "C" fn(*mut (), *const TypeMetadata) =
            unsafe { std::mem::transmute(*self.vwt.add(1)) };
        unsafe { destroy(value, self.metadata) };
    }

    /// # Safety
    ///
    /// `value` must be an initialized single-payload enum whose payload is this
    /// type, with `empty_cases` empty cases.
    #[inline]
    pub unsafe fn enum_tag_single_payload(&self, value: *const (), empty_cases: u32) -> u32 {
        let get_tag: unsafe extern "C" fn(*const (), u32, *const TypeMetadata) -> u32 =
            unsafe { std::mem::transmute(*self.vwt.add(6)) };
        unsafe { get_tag(value, empty_cases, self.metadata) }
    }
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

/// Reads the case tag of a multi-payload Swift enum.
#[inline]
pub unsafe fn get_enum_tag(value: *const (), metadata: *const TypeMetadata) -> u32 {
    let vwt = unsafe { value_witness_table(metadata) };
    let get_tag: unsafe extern "C" fn(*const (), *const TypeMetadata) -> u32 =
        unsafe { std::mem::transmute(*vwt.add(11)) };
    unsafe { get_tag(value, metadata) }
}

/// Replaces an enum value with its selected case's payload in place.
#[inline]
pub unsafe fn destructive_project_enum_data(value: *mut (), metadata: *const TypeMetadata) {
    let vwt = unsafe { value_witness_table(metadata) };
    let project: unsafe extern "C" fn(*mut (), *const TypeMetadata) =
        unsafe { std::mem::transmute(*vwt.add(12)) };
    unsafe { project(value, metadata) };
}

/// `ValueWitnessFlags::HasEnumWitnesses`.
const HAS_ENUM_WITNESSES: usize = 0x0020_0000;

/// `ValueWitnessFlags::IsNonBitwiseTakable`.
const IS_NON_BITWISE_TAKABLE: usize = 0x0010_0000;

/// Offset of the element count inside native Swift array storage.
pub(crate) const ARRAY_COUNT_OFFSET: usize = 16;

/// Offset of the first element inside native array storage, past the heap
/// object header and `_ArrayBody`. Valid for elements aligned to at most 16.
///
/// These two are an internal standard-library layout, but a stable one: an
/// optimized `a[i]` on `[Double]` inlines to `ldr x8, [x0, #16]` for the count
/// and `ldr d0, [x0, x1, lsl #3, #32]` for the element. Every caller still
/// checks the stored count against the one the array reports, so a layout
/// change shows up as a fallback rather than a wild read.
pub(crate) const ARRAY_ELEMENTS_OFFSET: usize = 32;

/// Bits the standard library sets on an array's storage word when the buffer is
/// an `NSArray` rather than native Swift storage.
///
/// Swift's own subscript tests exactly these before touching the buffer and
/// branches to `_ArrayBuffer._getElementSlowPath` when any is set.
pub(crate) const ARRAY_BRIDGED_TAG: usize = 0xc000_0000_0000_0001;

/// Clears the spare bits Swift keeps in the storage word before dereferencing.
pub(crate) const ARRAY_STORAGE_MASK: usize = !0x7;

/// Whether an initialized value may be relocated with a plain byte copy.
///
/// Most Swift values may; the exceptions keep pointers into themselves, so
/// their address is part of their state.
#[inline]
pub unsafe fn is_bitwise_takable(metadata: *const TypeMetadata) -> bool {
    let vwt = unsafe { value_witness_table(metadata) };
    unsafe { *vwt.add(10) & IS_NON_BITWISE_TAKABLE == 0 }
}

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
pub unsafe fn call_double_to_words2(function: *const (), value: f64) -> (u64, u64) {
    let word0: u64;
    let word1: u64;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("d0") value,
            lateout("x0") word0,
            lateout("x1") word1,
            clobber_abi("C"),
        );
    }
    (word0, word1)
}

#[inline]
pub unsafe fn call_vector_duration_bool_object(
    function: *const (),
    vector: (f64, f64, f64),
    duration: (u64, u64),
    relative: bool,
    object: *const (),
) -> (*mut (), *mut ()) {
    let result: *mut ();
    let error: *mut ();
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("d0") vector.0,
            in("d1") vector.1,
            in("d2") vector.2,
            inlateout("x0") duration.0 => result,
            in("x1") duration.1,
            in("x2") relative as usize,
            in("x20") object,
            inlateout("x21") 0usize => error,
            clobber_abi("C"),
        );
    }
    (result, error)
}

#[inline]
pub unsafe fn call_rotation_duration_bool_object(
    function: *const (),
    rotation: (f64, f64, f64, f64),
    duration: (u64, u64),
    relative: bool,
    object: *const (),
) -> (*mut (), *mut ()) {
    let result: *mut ();
    let error: *mut ();
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("d0") rotation.0,
            in("d1") rotation.1,
            in("d2") rotation.2,
            in("d3") rotation.3,
            inlateout("x0") duration.0 => result,
            in("x1") duration.1,
            in("x2") relative as usize,
            in("x20") object,
            inlateout("x21") 0usize => error,
            clobber_abi("C"),
        );
    }
    (result, error)
}

#[inline]
pub unsafe fn call_doubles3_to_throwing_value(
    function: *const (),
    values: (f64, f64, f64),
    out: *mut (),
) -> *mut () {
    let error: *mut ();
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("d0") values.0,
            in("d1") values.1,
            in("d2") values.2,
            in("x8") out,
            inlateout("x21") 0usize => error,
            clobber_abi("C"),
        );
    }
    error
}

#[inline]
pub unsafe fn call_values3_to_value(
    function: *const (),
    value0: *mut (),
    value1: *mut (),
    value2: *mut (),
    out: *mut (),
) {
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x0") value0,
            in("x1") value1,
            in("x2") value2,
            in("x8") out,
            clobber_abi("C"),
        );
    }
}

#[inline]
pub unsafe fn call_static0_object(function: *const (), type_metadata: *const ()) -> *mut () {
    let object: *mut ();
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") type_metadata,
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
            "blr {function}",
            function = in(reg) function,
            in("x20") type_metadata,
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
            "blr {function}",
            function = in(reg) function,
            in("x20") type_metadata,
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
            "blr {function}",
            function = in(reg) function,
            in("x20") type_metadata,
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
            "blr {function}",
            function = in(reg) function,
            in("x20") type_metadata,
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
pub unsafe fn call_int_value_rect_value_to_value(
    function: *const (),
    integer: isize,
    value: *const (),
    rect: (f64, f64, f64, f64),
    trailing_value: *const (),
    out: *mut (),
) {
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x0") integer,
            in("x1") value,
            in("d0") rect.0,
            in("d1") rect.1,
            in("d2") rect.2,
            in("d3") rect.3,
            in("x2") trailing_value,
            in("x8") out,
            clobber_abi("C"),
        );
    }
}

#[cfg(feature = "av")]
#[inline]
pub unsafe fn call_camera_information_init(
    function: *const (),
    device_type: *const (),
    position: isize,
    orientation: *const (),
    intrinsics: *const (),
    reference_dimensions: (u64, u64, u64),
    out: *mut (),
) {
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x0") device_type,
            in("x1") position,
            in("x2") orientation,
            in("x3") intrinsics,
            in("x4") reference_dimensions.0,
            in("x5") reference_dimensions.1,
            in("x6") reference_dimensions.2,
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
pub unsafe fn call_object_to_bool(function: *const (), object: *const ()) -> bool {
    let result: usize;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") object,
            inlateout("x0") object as usize => result,
            clobber_abi("C"),
        );
    }
    result & 1 == 1
}

#[inline]
pub unsafe fn call_object_to_int(function: *const (), object: *const ()) -> isize {
    let result: isize;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") object,
            inlateout("x0") object as isize => result,
            clobber_abi("C"),
        );
    }
    result
}

#[inline]
pub unsafe fn call_objects_to_bool(function: *const (), lhs: *const (), rhs: *const ()) -> bool {
    let result: usize;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            inlateout("x0") lhs as usize => result,
            in("x1") rhs,
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
            "blr {function}",
            function = in(reg) function,
            in("x20") object,
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
            "blr {function}",
            function = in(reg) function,
            in("x20") object,
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
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
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
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
            inlateout("x0") value as usize => result,
            clobber_abi("C"),
        );
    }
    result & 1 == 1
}

#[inline]
pub unsafe fn call_value_to_double(function: *const (), value: *const ()) -> f64 {
    let result: f64;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
            in("x0") value,
            lateout("d0") result,
            clobber_abi("C"),
        );
    }
    result
}

#[inline]
pub unsafe fn call_value_to_words3(function: *const (), value: *const ()) -> (u64, u64, u64) {
    let word0: u64;
    let word1: u64;
    let word2: u64;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
            in("x0") value,
            lateout("x0") word0,
            lateout("x1") word1,
            lateout("x2") word2,
            clobber_abi("C"),
        );
    }
    (word0, word1, word2)
}

#[inline]
pub unsafe fn call_value_to_doubles2(function: *const (), value: *const ()) -> (f64, f64) {
    let value0: f64;
    let value1: f64;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
            in("x0") value,
            lateout("d0") value0,
            lateout("d1") value1,
            clobber_abi("C"),
        );
    }
    (value0, value1)
}

#[inline]
pub unsafe fn call_value_to_rect(function: *const (), value: *const ()) -> (f64, f64, f64, f64) {
    let x: f64;
    let y: f64;
    let width: f64;
    let height: f64;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
            in("x0") value,
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
pub unsafe fn call_value_to_object(function: *const (), value: *const ()) -> *mut () {
    let result: usize;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
            inlateout("x0") value as usize => result,
            clobber_abi("C"),
        );
    }
    result as *mut ()
}

/// Calls a member that returns three doubles in `d0`-`d2`, which is how a
/// three-lane vector such as `SPVector3D` comes back.
///
/// # Safety
///
/// `function` must be a member of `value`'s type returning such a vector.
#[inline]
pub unsafe fn call_value_to_doubles3(function: *const (), value: *const ()) -> [f64; 3] {
    let (x, y, z);
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
            in("x0") value,
            lateout("d0") x,
            lateout("d1") y,
            lateout("d2") z,
            clobber_abi("C"),
        );
    }
    [x, y, z]
}

#[inline]
pub unsafe fn call_value_to_string(function: *const (), value: *const ()) -> RawString {
    let word0: usize;
    let word1: usize;
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
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
            "blr {function}",
            function = in(reg) function,
            in("x20") object,
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
            "blr {function}",
            function = in(reg) function,
            in("x20") value,
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
            "blr {function}",
            function = in(reg) function,
            in("x20") object,
            inlateout("x21") 0usize => error,
            in("x0") object,
            in("x8") out,
            clobber_abi("C"),
        );
    }
    error
}

#[inline]
pub unsafe fn call_value_object_to_throwing_void(
    function: *const (),
    value: *const (),
    object: *const (),
) -> *mut () {
    let error: *mut ();
    unsafe {
        asm!(
            "blr {function}",
            function = in(reg) function,
            in("x0") value,
            in("x20") object,
            inlateout("x21") 0usize => error,
            clobber_abi("C"),
        );
    }
    error
}
