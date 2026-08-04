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

/// Declares a zero-sized marker naming a Swift type, and implements
/// [`SwiftMetadata`] for it.
///
/// Deliberately not [`SwiftType`]: a marker has none of the Swift value's
/// layout, so it may name a type but never stand in for one.
///
/// The three forms match how Swift publishes a type's metadata — a generated
/// accessor symbol, a mangled name resolved at runtime, or an opaque return
/// type's descriptor.
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
                $resolve
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
        unsafe { abi::type_by_mangled_name("So6CMTimea") }
    }
}

#[cfg(feature = "cm")]
unsafe impl SwiftType for crate::cm::Time {}

#[cfg(feature = "cm")]
unsafe impl SwiftMetadata for crate::cm::TimeRange {
    #[inline]
    fn metadata() -> *const TypeMetadata {
        unsafe { abi::type_by_mangled_name("So11CMTimeRangea") }
    }
}

#[cfg(feature = "cm")]
unsafe impl SwiftType for crate::cm::TimeRange {}

#[cfg(test)]
mod tests {
    use super::*;

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
