pub mod call;

use core::{arch::naked_asm, ffi::c_char};

use super::RawString;

#[repr(C)]
pub struct TypeMetadata {
    _priv: [u8; 0],
}

/// `MetadataRequest(0)`, which asks for complete metadata — the only state a
/// binding can use.
const COMPLETE_METADATA: usize = 0;

/// The two pointers `_allocateUninitializedArray` hands back: the array's
/// storage word, and the address of its first element.
///
/// Two words, so both come back in registers rather than through memory.
#[repr(C)]
struct ArrayAllocation {
    storage: *mut (),
    elements: *mut (),
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
    ) -> *const TypeMetadata;
    fn swift_getOpaqueTypeConformance2(
        args: *const *const (),
        descriptor: *const (),
        index: usize,
    ) -> *const ();
    fn swift_getAssociatedConformanceWitness(
        witness: *const (),
        conforming_type: *const TypeMetadata,
        associated_type: *const TypeMetadata,
        requirements: *const (),
        conformance: *const (),
    ) -> *const ();

    fn swift_bridgeObjectRetain(object: usize) -> usize;
    fn swift_bridgeObjectRelease(object: usize);

    fn _swift_stdlib_bridgeErrorToNSError(error: *mut ()) -> *mut ();

    fn swift_getWitnessTable(
        conformance: *const (),
        type_metadata: *const TypeMetadata,
        instantiation_args: *const *const (),
    ) -> *const ();

    /// `Set.init(arrayLiteral:)`, whose variadic parameter is an array.
    #[link_name = "$sSh12arrayLiteralShyxGxd_tcfC"]
    fn swift_set_from_array_literal(
        array: *mut (),
        element: *const TypeMetadata,
        hashable: *const (),
    ) -> *mut ();

    /// `Dictionary.subscript`, which writes `Value?` through the indirect
    /// result register and so is reached through a thunk rather than directly.
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
    fn swift_string_from_c_string(ptr: *const c_char) -> RawString;

    #[link_name = "$sSS5countSivg"]
    fn swift_string_count(word0: usize, word1: usize) -> isize;

    /// Returns `Bool`, which occupies one register whose other bits are the
    /// callee's business, so it is taken as a word and masked.
    #[link_name = "$sSS7isEmptySbvg"]
    fn swift_string_is_empty(word0: usize, word1: usize) -> usize;

    #[link_name = "$sSS9hashValueSivg"]
    fn swift_string_hash_value(word0: usize, word1: usize) -> isize;

    #[link_name = "$sSS18_uncheckedFromUTF8ySSSRys5UInt8VGFZ"]
    fn swift_string_from_utf8(ptr: *const u8, len: usize) -> RawString;

    #[link_name = "$sSS11utf8CStrings15ContiguousArrayVys4Int8VGvg"]
    fn swift_string_utf8_c_string(word0: usize, word1: usize) -> *mut ();

    /// The same masking as `isEmpty`.
    #[link_name = "$sSS2eeoiySbSS_SStFZ"]
    fn swift_string_equal(l0: usize, l1: usize, r0: usize, r1: usize) -> usize;

    #[link_name = "$ss27_allocateUninitializedArrayySayxG_BptBwlF"]
    fn swift_allocate_uninitialized_array(
        len: usize,
        element: *const TypeMetadata,
    ) -> ArrayAllocation;

    #[link_name = "$sSa5countSivg"]
    fn swift_array_count(array: *const (), element: *const TypeMetadata) -> isize;

    /// `Array.subscript`, which returns its element indirectly, so it is
    /// reached through a thunk.
    #[link_name = "$sSayxSicig"]
    fn swift_array_get();

    #[link_name = "$sSaMa"]
    fn swift_array_metadata(request: usize, element: *const TypeMetadata) -> MetadataResponse;

    #[link_name = "$sSqMa"]
    fn swift_optional_metadata(request: usize, wrapped: *const TypeMetadata) -> MetadataResponse;

    #[link_name = "$sShMa"]
    fn swift_set_metadata(request: usize, element: *const TypeMetadata) -> MetadataResponse;

    #[link_name = "$sSDMa"]
    fn swift_dictionary_metadata(
        request: usize,
        key: *const TypeMetadata,
        value: *const TypeMetadata,
    ) -> MetadataResponse;

    #[link_name = "$ss15ContiguousArrayV5countSivg"]
    fn swift_contiguous_array_count(array: *const (), element: *const TypeMetadata) -> isize;

    /// The same indirect return as `Array.subscript`.
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
    unsafe { swift_array_metadata(COMPLETE_METADATA, element).metadata }
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
    unsafe { swift_set_metadata(COMPLETE_METADATA, element).metadata }
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
    unsafe { swift_dictionary_metadata(COMPLETE_METADATA, key, value).metadata }
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
    unsafe { swift_set_from_array_literal(array, metadata, witness) }
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
        dictionary_subscript_thunk(key, dictionary, key_metadata, value_metadata, witness, out)
    }
}

/// Moves the caller's buffer into the indirect-result register and hands off.
///
/// `x8` is not an argument register, so C has no way to name it; a naked thunk
/// is how it gets set without an `asm!` block at the call site marking
/// `v8`-`v15` clobbered and costing every caller eight registers.
#[unsafe(naked)]
unsafe extern "C" fn dictionary_subscript_thunk(
    _key: *const (),
    _dictionary: *const (),
    _key_metadata: *const TypeMetadata,
    _value_metadata: *const TypeMetadata,
    _witness: *const (),
    _out: *mut (),
) {
    naked_asm!("mov x8, x5", "b {f}", f = sym swift_dictionary_subscript)
}

#[inline]
pub unsafe fn optional_metadata(wrapped: *const TypeMetadata) -> *const TypeMetadata {
    unsafe { swift_optional_metadata(COMPLETE_METADATA, wrapped).metadata }
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
            align: (flags & 0xff) + 1,
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
        align: align_mask + 1,
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
const IS_NON_POD: usize = 0x0001_0000;

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

/// What a type's metadata accessor hands back: the metadata, and how far the
/// runtime got building it.
#[repr(C)]
pub struct MetadataResponse {
    pub metadata: *const TypeMetadata,
    pub state: usize,
}

/// Calls a type's metadata accessor.
///
/// An accessor takes a `MetadataRequest` and returns two words. It reads no
/// context register, throws nothing, and returns nothing indirectly, so Swift's
/// convention and C's coincide here — which is why this is a plain call rather
/// than an assembly shim, and why resolving metadata no longer makes its caller
/// spill `d8`-`d15`.
///
/// # Safety
///
/// `accessor` must be a metadata accessor symbol for some type.
#[inline]
pub unsafe fn call_metadata_accessor(accessor: *const ()) -> *const TypeMetadata {
    let accessor: extern "C" fn(usize) -> MetadataResponse =
        unsafe { core::mem::transmute(accessor) };
    // `MetadataRequest(0)` asks for complete metadata, which is the only state
    // a binding can use.
    accessor(0).metadata
}

/// Whether an initialized value may be relocated with a plain byte copy.
///
/// Most Swift values may; the exceptions keep pointers into themselves, so
/// their address is part of their state.
#[inline]
pub unsafe fn is_pod(metadata: *const TypeMetadata) -> bool {
    let vwt = unsafe { value_witness_table(metadata) };
    unsafe { *vwt.add(10) & IS_NON_POD == 0 }
}

/// # Safety
///
/// `metadata` must be valid.
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
    unsafe { swift_getOpaqueTypeMetadata(COMPLETE_METADATA, core::ptr::null(), descriptor, index) }
}

#[inline]
pub unsafe fn opaque_type_conformance(descriptor: *const (), index: usize) -> *const () {
    unsafe { swift_getOpaqueTypeConformance2(core::ptr::null(), descriptor, index) }
}

#[inline]
pub unsafe fn associated_conformance_witness(
    witness: *const (),
    conforming_type: *const TypeMetadata,
    associated_type: *const TypeMetadata,
    requirements: *const (),
    conformance: *const (),
) -> *const () {
    unsafe {
        swift_getAssociatedConformanceWitness(
            witness,
            conforming_type,
            associated_type,
            requirements,
            conformance,
        )
    }
}

#[inline]
pub unsafe fn bridge_object_retain(object: usize) -> usize {
    unsafe { swift_bridgeObjectRetain(object) }
}

#[inline]
pub unsafe fn bridge_object_release(object: usize) {
    unsafe { swift_bridgeObjectRelease(object) }
}

#[inline]
pub unsafe fn string_from_c_str(ptr: *const c_char) -> RawString {
    unsafe { swift_string_from_c_string(ptr) }
}

#[inline]
pub unsafe fn string_count(string: RawString) -> isize {
    unsafe { swift_string_count(string.word0, string.word1) }
}

/// Whether the string is empty, which Swift answers without walking it.
///
/// # Safety
///
/// `string` must be a valid Swift `String` value.
#[inline]
pub unsafe fn string_is_empty(string: RawString) -> bool {
    unsafe { swift_string_is_empty(string.word0, string.word1) & 1 != 0 }
}

/// The string's own hash, so equal strings hash equally however they are
/// composed.
///
/// # Safety
///
/// `string` must be a valid Swift `String` value.
#[inline]
pub unsafe fn string_hash_value(string: RawString) -> isize {
    unsafe { swift_string_hash_value(string.word0, string.word1) }
}

#[inline]
pub unsafe fn string_from_utf8(bytes: &[u8]) -> RawString {
    unsafe { swift_string_from_utf8(bytes.as_ptr(), bytes.len()) }
}

#[inline]
pub unsafe fn string_utf8_c_string(string: RawString) -> *mut () {
    unsafe { swift_string_utf8_c_string(string.word0, string.word1) }
}

#[inline]
pub unsafe fn string_equal(lhs: RawString, rhs: RawString) -> bool {
    unsafe { swift_string_equal(lhs.word0, lhs.word1, rhs.word0, rhs.word1) & 1 == 1 }
}

#[inline]
pub unsafe fn allocate_uninitialized_array(
    len: usize,
    element: *const TypeMetadata,
) -> (*mut (), *mut ()) {
    let allocation = unsafe { swift_allocate_uninitialized_array(len, element) };
    (allocation.storage, allocation.elements)
}

#[inline]
pub unsafe fn array_count(array: *const (), element: *const TypeMetadata) -> isize {
    unsafe { swift_array_count(array, element) }
}

#[inline]
pub unsafe fn array_get(
    array: *const (),
    index: isize,
    out: *mut (),
    element: *const TypeMetadata,
) {
    unsafe { array_get_thunk(index, array, element, out) }
}

/// The same indirect-result hand-off as the dictionary subscript.
#[unsafe(naked)]
unsafe extern "C" fn array_get_thunk(
    _index: isize,
    _array: *const (),
    _element: *const TypeMetadata,
    _out: *mut (),
) {
    naked_asm!("mov x8, x3", "b {f}", f = sym swift_array_get)
}

#[inline]
pub unsafe fn contiguous_array_count(array: *const (), element: *const TypeMetadata) -> isize {
    unsafe { swift_contiguous_array_count(array, element) }
}

#[inline]
pub unsafe fn contiguous_array_get(
    array: *const (),
    index: isize,
    out: *mut (),
    element: *const TypeMetadata,
) {
    unsafe { contiguous_array_get_thunk(index, array, element, out) }
}

/// The same again, for `ContiguousArray`.
#[unsafe(naked)]
unsafe extern "C" fn contiguous_array_get_thunk(
    _index: isize,
    _array: *const (),
    _element: *const TypeMetadata,
    _out: *mut (),
) {
    naked_asm!("mov x8, x3", "b {f}", f = sym swift_contiguous_array_get)
}
