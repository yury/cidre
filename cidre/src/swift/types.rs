use super::abi::{self, TypeMetadata};

/// A Swift type whose runtime metadata is determined by the Rust type.
///
/// Unlike [`SwiftType`], an implementor may be a zero-sized marker for a Swift
/// value whose layout is only available through its runtime metadata.
///
/// # Safety
///
/// [`metadata`](Self::metadata) must always return metadata for the same Swift
/// type. The metadata must remain valid for the duration of the process.
pub unsafe trait SwiftMetadata {
    fn metadata() -> *const TypeMetadata;

    /// Whether a value of this type is a single reference held directly.
    ///
    /// Swift passes such a value as the reference itself where it passes every
    /// other value by address, so callers that build a `self` operand have to
    /// know which they have. Deriving it from the type keeps a binding from
    /// stating the wrong one, which is a silent ABI mismatch rather than a
    /// compile error.
    const IS_CLASS_REF: bool = false;
}

/// A Rust type with the same value representation and ownership semantics as a
/// Swift type.
///
/// # Safety
///
/// Implementors must have the exact size, alignment, valid bit patterns, and
/// ownership semantics described by the metadata returned from [`metadata`].
/// A value may be copied or destroyed through that metadata's value witnesses.
///
/// [`metadata`]: SwiftMetadata::metadata
pub unsafe trait SwiftType: SwiftMetadata + Sized {}

/// A Rust value that can stand as the `self` operand of a Swift call.
///
/// Swift passes `self` in a register of its own, holding the reference itself
/// for a class and the address of the value for everything else.
/// [`#[swift::call]`](crate::swift::call) builds that operand through this
/// trait rather than deciding per binding which of the two it has, which is the
/// one thing about a call site that cannot be read off the Rust signature.
///
/// # Safety
///
/// [`swift_self_ptr`](Self::swift_self_ptr) must return what Swift's convention
/// expects in the context register: a class reference, or the address of an
/// initialized value of the type the callee belongs to.
pub unsafe trait SwiftSelf {
    fn swift_self_ptr(&self) -> *const ();
}

/// Which registers a value of this type occupies when Swift returns it.
///
/// [`#[swift::call]`](crate::swift::call) picks the registers by reading the
/// Rust type, and treats a name it does not recognise as a value returned
/// indirectly, because that is what every framework value type is. A type that
/// is really something else would otherwise be called with the wrong registers
/// and no diagnostic at all, so the expansion asserts its guess against this.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum AbiClass {
    /// Written into caller-provided storage through the indirect-result
    /// register.
    Indirect,
    /// One integer register.
    Word,
    /// Two integer registers, which is a `Swift.String`.
    Words2,
    /// Three integer registers, which is a `CMTime`.
    Words3,
    /// One floating-point register, as a `double`.
    Double,
    /// One floating-point register, as a `float`.
    Float,
    /// A run of floating-point registers.
    Doubles(u8),
}

impl AbiClass {
    /// A discriminant the expansion can compare in a `const` assertion, since
    /// `PartialEq` is not available there.
    pub const fn tag(self) -> u16 {
        match self {
            Self::Indirect => 0,
            Self::Word => 1,
            Self::Words2 => 2,
            Self::Words3 => 3,
            Self::Double => 4,
            Self::Float => 5,
            Self::Doubles(count) => 0x100 | count as u16,
        }
    }
}

/// The registers Swift returns this type in.
///
/// # Safety
///
/// [`CLASS`](Self::CLASS) must be the convention Swift really uses for the type,
/// since a binding calls through it without further checking.
pub unsafe trait SwiftAbi {
    const CLASS: AbiClass;
}

macro_rules! impl_swift_abi {
    ($($ty:ty => $class:expr),+ $(,)?) => {
        $(unsafe impl SwiftAbi for $ty {
            const CLASS: AbiClass = $class;
        })+
    };
}

impl_swift_abi!(
    bool => AbiClass::Word,
    isize => AbiClass::Word,
    usize => AbiClass::Word,
    i8 => AbiClass::Word,
    u8 => AbiClass::Word,
    i16 => AbiClass::Word,
    u16 => AbiClass::Word,
    i32 => AbiClass::Word,
    u32 => AbiClass::Word,
    i64 => AbiClass::Word,
    u64 => AbiClass::Word,
    f32 => AbiClass::Float,
    f64 => AbiClass::Double,
    super::String => AbiClass::Words2,
);

/// A class-typed value is one retained reference.
unsafe impl<T: SwiftClass> SwiftAbi for crate::arc::R<T> {
    const CLASS: AbiClass = AbiClass::Word;
}

/// A container is one word whatever it holds.
unsafe impl<T> SwiftAbi for super::Array<T> {
    const CLASS: AbiClass = AbiClass::Word;
}

unsafe impl<T> SwiftAbi for super::Set<T> {
    const CLASS: AbiClass = AbiClass::Word;
}

unsafe impl<K, V> SwiftAbi for super::Dictionary<K, V> {
    const CLASS: AbiClass = AbiClass::Word;
}

#[cfg(feature = "cm")]
unsafe impl SwiftAbi for crate::cm::Time {
    const CLASS: AbiClass = AbiClass::Words3;
}

#[cfg(feature = "cg")]
unsafe impl SwiftAbi for crate::cg::Rect {
    const CLASS: AbiClass = AbiClass::Doubles(4);
}

#[cfg(feature = "spatial")]
unsafe impl SwiftAbi for crate::spatial::Vector3D {
    const CLASS: AbiClass = AbiClass::Doubles(3);
}

/// A geometric value Swift returns in floating-point registers.
///
/// How many registers that is belongs to the Swift type, not the Rust one — a
/// [`spatial::Vector3D`](crate::spatial::Vector3D) is four doubles wide but
/// comes back in three — so the value is built through this rather than by
/// reinterpreting a register block as the Rust struct.
///
/// # Safety
///
/// [`from_doubles`](Self::from_doubles) must accept exactly the registers
/// Swift's convention returns for the type.
pub unsafe trait FromSwiftDoubles: Sized {
    fn from_doubles(values: &[f64]) -> Self;
}

#[cfg(feature = "cg")]
unsafe impl FromSwiftDoubles for crate::cg::Rect {
    #[inline]
    fn from_doubles(values: &[f64]) -> Self {
        let [x, y, width, height] = values.try_into().expect("a rect is four doubles");
        Self {
            origin: crate::cg::Point { x, y },
            size: crate::cg::Size { width, height },
        }
    }
}

#[cfg(feature = "spatial")]
unsafe impl FromSwiftDoubles for crate::spatial::Vector3D {
    #[inline]
    fn from_doubles(values: &[f64]) -> Self {
        let [x, y, z] = values.try_into().expect("a vector is three doubles");
        Self::new(x, y, z)
    }
}

/// A Swift type whose values may cross threads, which is Swift's `Sendable`.
///
/// [`Storage`](super::value::Storage) holds a raw pointer, so a wrapper around
/// one is neither `Send` nor `Sync` by inference and every binding used to
/// declare both by hand — inconsistently, since nothing said what the answer
/// depended on. It depends on the Swift type, so that is where it is stated:
/// the storage and every wrapper over it then follow.
///
/// # Safety
///
/// The Swift type must be safe to move between threads and to read from
/// several at once: value semantics over atomically reference-counted
/// contents, which is what Swift's own `Sendable` conformance promises.
pub unsafe trait SwiftSendable: SwiftMetadata {}

/// States that a Swift type is `Sendable`, so its values are `Send` and `Sync`.
#[macro_export]
macro_rules! impl_swift_sendable {
    ($($ty:ty),+ $(,)?) => {
        $(unsafe impl $crate::swift::SwiftSendable for $ty {})+
    };
}

/// A Rust value that can be read out of a Swift value of the type it names.
///
/// This is what lets the generic containers — [`Array`](super::Array),
/// [`Set`](super::Set), [`Dictionary`](super::Dictionary) — hold both kinds of
/// Rust type: one whose layout *is* the Swift value's ([`SwiftType`]), and one
/// that owns runtime-laid-out storage or decodes into a native Rust value. The
/// container only ever needs the two operations below plus the element's
/// metadata, so it does not care which kind it has.
///
/// # Safety
///
/// [`copy_swift`](Self::copy_swift) must read a value of [`Self::metadata`] at
/// the pointer without taking ownership of it: the caller still destroys the
/// Swift value afterwards. [`take_swift`](Self::take_swift) must leave the
/// storage uninitialized, so the caller must not destroy it again.
pub unsafe trait FromSwift: SwiftMetadata + Sized {
    /// Copies a borrowed Swift value, leaving the original to its owner.
    ///
    /// # Safety
    ///
    /// `value` must point to an initialized Swift value of this type that
    /// outlives the call.
    unsafe fn copy_swift(value: *const ()) -> Self;

    /// Takes ownership of the Swift value at `value`, leaving its storage
    /// uninitialized.
    ///
    /// This is what a `+1` result — an array subscript, a getter's indirect
    /// return — hands back, so the default copy-then-destroy exists only for
    /// types that decode into something Rust owns outright.
    ///
    /// # Safety
    ///
    /// `value` must point to an initialized Swift value of this type, and the
    /// caller must own it and not destroy it afterwards.
    unsafe fn take_swift(value: *mut ()) -> Self {
        unsafe {
            let taken = Self::copy_swift(value.cast_const());
            abi::destroy_value(value, Self::metadata());
            taken
        }
    }
}

/// A Rust value that can be written into storage for the Swift type it names.
///
/// The counterpart of [`FromSwift`]: together they are what a container needs
/// to move elements in either direction without knowing how the Rust type
/// stores them.
///
/// # Safety
///
/// [`copy_to_swift`](Self::copy_to_swift) must leave `dst` holding an
/// initialized, independently owned value of [`Self::metadata`].
/// [`IS_CONTIGUOUS`](Self::IS_CONTIGUOUS) may only be set when a Rust slice of
/// `Self` is already a run of Swift values at that type's stride.
pub unsafe trait ToSwift: SwiftMetadata {
    /// Whether `&[Self]` is already a run of Swift values, so a whole slice can
    /// be copied with one call instead of one per element.
    const IS_CONTIGUOUS: bool = false;

    /// Copies this value into uninitialized storage for its Swift type.
    ///
    /// # Safety
    ///
    /// `dst` must be uninitialized storage laid out for this type.
    unsafe fn copy_to_swift(&self, dst: *mut ());
}

/// Implements the value traits for a type whose Rust layout is the Swift one,
/// so reading is a move out of the buffer, writing goes through the type's own
/// copy witness, and a Rust slice is already a run of Swift values.
///
/// The second form takes the generic parameters and their bounds, for the
/// container and reference types that are one word whatever they hold.
#[macro_export]
macro_rules! impl_swift_memcpy_value {
    ($ty:ty) => {
        $crate::impl_swift_memcpy_value!(@impls $ty, <>);
    };
    ($ty:ty, <$($param:ident : $bound:path),+ $(,)?>) => {
        $crate::impl_swift_memcpy_value!(@impls $ty, <$($param: $bound),+>);
    };
    (@impls $ty:ty, <$($param:ident : $bound:path),*>) => {
        unsafe impl<$($param: $bound),*> $crate::swift::FromSwift for $ty {
            #[inline]
            unsafe fn copy_swift(value: *const ()) -> Self {
                unsafe {
                    let mut copy = ::core::mem::MaybeUninit::<Self>::uninit();
                    $crate::swift::abi::initialize_with_copy(
                        copy.as_mut_ptr().cast(),
                        value,
                        <Self as $crate::swift::SwiftMetadata>::metadata(),
                    );
                    copy.assume_init()
                }
            }

            #[inline]
            unsafe fn take_swift(value: *mut ()) -> Self {
                unsafe { value.cast::<Self>().read() }
            }
        }

        unsafe impl<$($param: $bound),*> $crate::swift::ToSwift for $ty {
            const IS_CONTIGUOUS: bool = true;

            #[inline]
            unsafe fn copy_to_swift(&self, dst: *mut ()) {
                unsafe {
                    $crate::swift::abi::initialize_with_copy(
                        dst,
                        ::core::ptr::from_ref(self).cast(),
                        <Self as $crate::swift::SwiftMetadata>::metadata(),
                    )
                };
            }
        }
    };
}

/// Declares a newtype for a retained Objective-C object used where Swift
/// expects a value of that class.
///
/// Swift imports an Objective-C class as a class type whose metadata the
/// runtime derives from the class object, and whose value is one retained
/// reference — the same shape as a native Swift class, but reached differently.
/// The newtype is what carries that metadata, so an imported class composes
/// with the generic containers instead of needing a hand-written array type.
#[macro_export]
macro_rules! define_swift_objc_ref {
    ($(#[$meta:meta])* $vis:vis $ty:ident($obj:ty) = class $class:literal) => {
        $(#[$meta])*
        #[repr(transparent)]
        $vis struct $ty(pub $crate::arc::R<$obj>);

        unsafe impl $crate::swift::SwiftMetadata for $ty {
            const IS_CLASS_REF: bool = true;

            fn metadata() -> *const $crate::swift::abi::TypeMetadata {
                static CACHE: $crate::swift::abi::MetadataCache =
                    $crate::swift::abi::MetadataCache::new();
                CACHE.get(|| unsafe {
                    let class =
                        $crate::objc::objc_getClass(concat!($class, "\0").as_ptr().cast())
                            .expect(concat!($class, " class must exist"));
                    $crate::swift::abi::objc_class_metadata(
                        (class as *const $crate::objc::Class<$crate::objc::Id>).cast(),
                    )
                })
            }
        }

        /// One retained reference is the whole value, and the class's own value
        /// witnesses are what copy and destroy it.
        unsafe impl $crate::swift::SwiftType for $ty {}

        $crate::impl_swift_memcpy_value!($ty);
    };
}

/// A Swift type whose `Hashable` conformance a binding can hand to the runtime.
///
/// `Set` and `Dictionary` are generic over `Hashable`, so every call into them
/// carries that conformance's witness table alongside the element metadata. A
/// framework exports only the conformance descriptor, so the table is
/// instantiated on first use and cached from then on.
///
/// # Safety
///
/// [`hashable_witness`](Self::hashable_witness) must return `Self`'s own
/// `Hashable` witness table, which must not be null.
pub unsafe trait SwiftHashable: SwiftMetadata {
    fn hashable_witness() -> *const ();
}

/// Implements [`SwiftHashable`] from the conformance descriptor a framework
/// exports for it, caching the table the runtime builds.
#[macro_export]
macro_rules! impl_swift_hashable {
    ($ty:ty = descriptor $descriptor:expr) => {
        unsafe impl $crate::swift::SwiftHashable for $ty {
            fn hashable_witness() -> *const () {
                static CACHE: $crate::swift::abi::WitnessCache =
                    $crate::swift::abi::WitnessCache::new();
                CACHE.get(|| unsafe {
                    let witness = $crate::swift::abi::witness_table(
                        $descriptor,
                        <$ty as $crate::swift::SwiftMetadata>::metadata(),
                    );
                    assert!(!witness.is_null(), "Hashable witness table must exist");
                    witness
                })
            }
        }
    };
}

/// A native Swift class.
///
/// A class type names a Swift type but has none of its value's layout, exactly
/// like a marker: the *value* of a class-typed variable is one reference, which
/// is what [`arc::R<Self>`](crate::arc::R) already is. The impls below give that
/// pointer the [`SwiftType`] role, so a class composes with everything built on
/// value witnesses — arrays, optionals, dictionaries, sequence elements —
/// instead of needing hand-written glue at each use.
///
/// # Safety
///
/// `Self` must be a native Swift class whose references are retained and
/// released by `swift_retain`/`swift_release`. Objective-C classes are not
/// these: they have their own metadata and their own ARC entry points.
pub unsafe trait SwiftClass: SwiftMetadata + crate::arc::Retain + 'static {
    /// Per-class cache for `Self?`'s metadata.
    ///
    /// A `static` inside a generic function is shared by every instantiation,
    /// so the cache has to be handed in from a place that is monomorphic in the
    /// class. [`define_swift_class!`](crate::define_swift_class) generates it.
    #[doc(hidden)]
    fn optional_metadata_cache() -> &'static abi::MetadataCache;
}

/// A retained reference is the ABI value of a class-typed variable: one word,
/// destroyed by release and copied by retain, which is what the runtime's value
/// witnesses for a class do.
unsafe impl<T: SwiftClass> SwiftMetadata for crate::arc::R<T> {
    const IS_CLASS_REF: bool = true;

    #[inline]
    fn metadata() -> *const TypeMetadata {
        <T as SwiftMetadata>::metadata()
    }
}

unsafe impl<T: SwiftClass> SwiftType for crate::arc::R<T> {}

impl_swift_memcpy_value!(crate::arc::R<T>, <T: SwiftClass>);

/// Swift represents `C?` for a class `C` by using the null reference as `.none`,
/// which is the same niche Rust picks for `Option<arc::R<C>>`, so the two are
/// bit-for-bit the same value.
unsafe impl<T: SwiftClass> SwiftMetadata for Option<crate::arc::R<T>> {
    #[inline]
    fn metadata() -> *const TypeMetadata {
        T::optional_metadata_cache()
            .get(|| unsafe { abi::optional_metadata(<T as SwiftMetadata>::metadata()) })
    }
}

unsafe impl<T: SwiftClass> SwiftType for Option<crate::arc::R<T>> {}

impl_swift_memcpy_value!(Option<crate::arc::R<T>>, <T: SwiftClass>);

/// Declares a zero-sized marker naming a Swift type, and implements
/// [`SwiftMetadata`] for it.
///
/// Deliberately not [`SwiftType`]: a marker has none of the Swift value's
/// layout, so it may name a type but never stand in for one.
///
/// The three forms match how Swift publishes a type's metadata — a generated
/// accessor symbol, a mangled name resolved at runtime, or an opaque return
/// type's descriptor.
///
/// Each marker caches the metadata it resolves, the way Swift emits a per-type
/// cache word and only calls the runtime while that word is still empty. The
/// mangled form matters most: without a cache every use re-demangles the name
/// through the runtime's locked type cache, and markers are resolved once per
/// [`Storage`](crate::swift::value::Storage) construction and once more per
/// drop.
#[macro_export]
macro_rules! define_swift_marker {
    ($(#[$meta:meta])* $vis:vis $ty:ident = accessor $accessor:expr) => {
        $crate::define_swift_marker!(@marker $(#[$meta])* $vis $ty, unsafe {
            $crate::swift::abi::call::int_to_int($accessor as *const (), 0)
                as *const $crate::swift::abi::TypeMetadata
        });
    };
    ($(#[$meta:meta])* $vis:vis $ty:ident = mangled $name:literal) => {
        $crate::define_swift_marker!(@marker $(#[$meta])* $vis $ty, unsafe {
            $crate::swift::abi::type_by_mangled_name($name)
        });
    };
    ($(#[$meta:meta])* $vis:vis $ty:ident = opaque $descriptor:expr, $index:literal) => {
        $crate::define_swift_marker!(@marker $(#[$meta])* $vis $ty, unsafe {
            $crate::swift::abi::opaque_type_metadata($descriptor, $index)
        });
    };
    (@marker $(#[$meta:meta])* $vis:vis $ty:ident, $resolve:expr) => {
        $(#[$meta])*
        $vis struct $ty;

        unsafe impl $crate::swift::SwiftMetadata for $ty {
            #[inline]
            fn metadata() -> *const $crate::swift::abi::TypeMetadata {
                static CACHE: $crate::swift::abi::MetadataCache =
                    $crate::swift::abi::MetadataCache::new();
                CACHE.get(|| $resolve)
            }
        }
    };
}

macro_rules! impl_swift_type {
    ($ty:ty, $metadata:ident) => {
        unsafe impl SwiftMetadata for $ty {
            #[inline]
            fn metadata() -> *const TypeMetadata {
                abi::$metadata()
            }
        }

        unsafe impl SwiftType for $ty {}

        impl_swift_memcpy_value!($ty);
    };
}

#[link(name = "swiftCore")]
unsafe extern "C" {
    #[link_name = "$sSiSHsMc"]
    static INT_HASHABLE: u8;

    #[link_name = "$sSSSHsMc"]
    static STRING_HASHABLE: u8;
}

impl_swift_hashable!(isize = descriptor(&raw const INT_HASHABLE).cast());
impl_swift_hashable!(super::String = descriptor(&raw const STRING_HASHABLE).cast());

impl_swift_type!(bool, bool_metadata);
impl_swift_type!(isize, int_metadata);
impl_swift_type!(usize, uint_metadata);
impl_swift_type!(i8, int8_metadata);
impl_swift_type!(u8, uint8_metadata);
impl_swift_type!(i16, int16_metadata);
impl_swift_type!(u16, uint16_metadata);
impl_swift_type!(i32, int32_metadata);
impl_swift_type!(u32, uint32_metadata);
impl_swift_type!(i64, int64_metadata);
impl_swift_type!(u64, uint64_metadata);
impl_swift_type!(f32, float_metadata);
impl_swift_type!(f64, double_metadata);

/// `CMTime` is imported from C, so Swift has no exported metadata accessor for
/// it and the runtime resolves it from the mangled name instead.
#[cfg(feature = "cm")]
unsafe impl SwiftMetadata for crate::cm::Time {
    #[inline]
    fn metadata() -> *const TypeMetadata {
        static CACHE: abi::MetadataCache = abi::MetadataCache::new();
        CACHE.get(|| unsafe { abi::type_by_mangled_name("So6CMTimea") })
    }
}

#[cfg(feature = "cm")]
unsafe impl SwiftType for crate::cm::Time {}

#[cfg(feature = "cm")]
impl_swift_memcpy_value!(crate::cm::Time);

#[cfg(feature = "cm")]
unsafe impl SwiftMetadata for crate::cm::TimeRange {
    #[inline]
    fn metadata() -> *const TypeMetadata {
        static CACHE: abi::MetadataCache = abi::MetadataCache::new();
        CACHE.get(|| unsafe { abi::type_by_mangled_name("So11CMTimeRangea") })
    }
}

#[cfg(feature = "cm")]
unsafe impl SwiftType for crate::cm::TimeRange {}

#[cfg(feature = "cm")]
impl_swift_memcpy_value!(crate::cm::TimeRange);

#[cfg(test)]
mod tests {
    use super::*;

    crate::define_swift_marker!(CachedString = mangled "SS");

    /// A marker must hand back exactly what an uncached lookup resolves, and
    /// keep handing back the same pointer once it has cached one.
    #[test]
    fn a_cached_marker_matches_an_uncached_lookup() {
        let direct = unsafe { abi::type_by_mangled_name("SS") };
        assert!(!direct.is_null(), "Swift.String metadata must resolve");
        assert_eq!(direct, abi::string_metadata(), "the mangled name is right");

        // The first call fills the cache; the second must read it back.
        assert_eq!(direct, CachedString::metadata());
        assert_eq!(direct, CachedString::metadata());
    }

    /// The cached witness table must describe the same type as the free
    /// functions that re-derive it from the metadata on every call.
    #[test]
    fn cached_witnesses_agree_with_the_derived_ones() {
        let metadata = abi::string_metadata();
        let witnesses = unsafe { abi::ValueWitnesses::new(metadata) };

        assert_eq!(metadata, witnesses.metadata());
        assert_eq!(unsafe { abi::value_layout(metadata) }, witnesses.layout());
        assert_eq!(
            unsafe { abi::is_bitwise_takable(metadata) },
            witnesses.is_bitwise_takable()
        );
    }

    /// A class reference has to satisfy `SwiftType`'s contract against the
    /// runtime's own witnesses, since that is what lets `arc::R` stand in for a
    /// class-typed value.
    #[cfg(all(
        any(target_os = "macos", all(target_os = "ios", not(target_abi = "sim"))),
        feature = "dk"
    ))]
    #[test]
    fn a_class_reference_matches_the_swift_value_layout() {
        use crate::{arc, swift::dock_kit::AccessoryManager};

        let metadata = <AccessoryManager as SwiftMetadata>::metadata();
        assert!(!metadata.is_null(), "class metadata must resolve");
        assert_eq!(
            metadata,
            <arc::R<AccessoryManager> as SwiftMetadata>::metadata()
        );

        let layout = unsafe { abi::value_layout(metadata) };
        assert_eq!(size_of::<arc::R<AccessoryManager>>(), layout.size);
        assert_eq!(size_of::<arc::R<AccessoryManager>>(), layout.stride);

        // `C?` must be the null-niche `Option`, not a wider tagged layout.
        let optional = <Option<arc::R<AccessoryManager>> as SwiftMetadata>::metadata();
        let optional_layout = unsafe { abi::value_layout(optional) };
        assert_eq!(
            size_of::<Option<arc::R<AccessoryManager>>>(),
            optional_layout.size
        );
    }

    /// The point of the whole exercise: a class now composes with the generic
    /// value machinery, which could not name one before.
    #[cfg(all(
        any(target_os = "macos", all(target_os = "ios", not(target_abi = "sim"))),
        feature = "dk"
    ))]
    #[test]
    fn a_swift_array_can_hold_class_references() {
        use crate::{arc, swift, swift::dock_kit::AccessoryManager};

        let shared = AccessoryManager::shared();
        let array = swift::Array::from_slice(&[shared.clone(), shared.clone()]);
        assert_eq!(2, array.len());

        // Reading back through the value witnesses must hand out live, retained
        // references rather than borrowed or over-released ones.
        for index in 0..array.len() {
            let element: arc::R<AccessoryManager> = array.get(index).expect("in bounds");
            assert_eq!(shared.as_ptr(), element.as_ptr());
            let _ = element.is_system_tracking_enabled();
        }

        drop(array);
        let _ = shared.is_system_tracking_enabled();
    }

    #[test]
    fn rust_primitives_match_swift_value_witness_layouts() {
        fn check<T: SwiftType>() {
            let layout = unsafe { abi::value_layout(T::metadata()) };
            assert_eq!(core::mem::size_of::<T>(), layout.size);
            assert_eq!(core::mem::size_of::<T>(), layout.stride);
        }

        check::<bool>();
        check::<isize>();
        check::<usize>();
        check::<i8>();
        check::<u8>();
        check::<i16>();
        check::<u16>();
        check::<i32>();
        check::<u32>();
        check::<i64>();
        check::<u64>();
        check::<f32>();
        check::<f64>();
    }

    /// Guards both the mangled name and that Rust's `cm::Time` really matches
    /// the layout Swift will copy through.
    #[cfg(feature = "cm")]
    #[test]
    fn imported_cm_time_resolves_and_matches_layout() {
        let metadata = crate::cm::Time::metadata();
        assert!(!metadata.is_null(), "CMTime metadata must resolve");

        let layout = unsafe { abi::value_layout(metadata) };
        assert_eq!(core::mem::size_of::<crate::cm::Time>(), layout.size);
        assert_eq!(core::mem::size_of::<crate::cm::Time>(), layout.stride);

        let metadata = crate::cm::TimeRange::metadata();
        assert!(!metadata.is_null(), "CMTimeRange metadata must resolve");
        let layout = unsafe { abi::value_layout(metadata) };
        assert_eq!(core::mem::size_of::<crate::cm::TimeRange>(), layout.size);
    }
}
