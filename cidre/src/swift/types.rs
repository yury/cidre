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
}
