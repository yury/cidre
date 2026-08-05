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

/// A Rust value that can be read out of a borrowed Swift value.
///
/// This ties a Rust type to the Swift type it is read from, which the old
/// convention — an inherent `copy_from_ptr` plus a separately named marker
/// passed alongside it — left to the binding author to keep consistent. Getting
/// that pair wrong is a value-witness misuse, not a compile error, so the
/// association belongs in the type system.
///
/// # Safety
///
/// [`from_swift`](Self::from_swift) must read a value of [`Self::Swift`] at the
/// pointer without taking ownership of it: the caller still destroys the Swift
/// value afterwards.
pub(crate) unsafe trait FromSwift: Sized {
    /// The Swift type a value is read from.
    type Swift: SwiftMetadata;

    /// # Safety
    ///
    /// `value` must point to an initialized `Self::Swift` that outlives the
    /// call.
    unsafe fn from_swift(value: *const ()) -> Self;
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
            $crate::swift::abi::call_int_to_int($accessor as *const (), 0)
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
    };
}

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
unsafe impl SwiftMetadata for crate::cm::TimeRange {
    #[inline]
    fn metadata() -> *const TypeMetadata {
        static CACHE: abi::MetadataCache = abi::MetadataCache::new();
        CACHE.get(|| unsafe { abi::type_by_mangled_name("So11CMTimeRangea") })
    }
}

#[cfg(feature = "cm")]
unsafe impl SwiftType for crate::cm::TimeRange {}

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
