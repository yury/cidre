use core::{marker::PhantomData, ptr::NonNull};

use super::{
    Int,
    abi::{self, TypeMetadata},
};

#[repr(C)]
struct NativeArrayStorage<T> {
    metadata: usize,
    ref_counts: usize,
    count: Int,
    capacity_and_flags: usize,
    elements: [T; 0],
}

/// A retained native Swift `Array` value.
#[derive(Debug)]
#[repr(transparent)]
pub struct Array<T> {
    storage: NonNull<NativeArrayStorage<T>>,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Array<T> {}
unsafe impl<T: Sync> Sync for Array<T> {}

impl<T> Array<T> {
    /// Takes ownership of a native Swift `Array` storage pointer.
    ///
    /// # Safety
    ///
    /// `storage` must be a non-null native Swift array storage object. It must be
    /// valid to release with `swift_bridgeObjectRelease`.
    #[inline]
    pub unsafe fn from_raw(storage: *mut ()) -> Self {
        let storage = NonNull::new(storage.cast()).expect("swift array storage must not be null");
        Self {
            storage,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_raw(&self) -> *mut () {
        self.storage.as_ptr().cast()
    }

    #[inline]
    pub fn into_raw(self) -> *mut () {
        let raw = self.as_raw();
        std::mem::forget(self);
        raw
    }

    #[inline]
    pub fn len(&self) -> usize {
        let count = unsafe { self.storage.as_ref().count };
        debug_assert!(count >= 0);
        count as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.elements(), self.len()) }
    }

    /// Allocates a native Swift array and copies `values` into it.
    ///
    /// # Safety
    ///
    /// `element` must be the Swift type metadata for `T`, and `T` must have the
    /// same in-memory layout as that Swift element type.
    #[inline]
    pub unsafe fn from_slice_with_metadata(values: &[T], element: *const TypeMetadata) -> Self
    where
        T: Copy,
    {
        let (storage, elements) =
            unsafe { abi::allocate_uninitialized_array(values.len(), element) };
        unsafe {
            core::ptr::copy_nonoverlapping(values.as_ptr(), elements.cast(), values.len());
            Self::from_raw(storage)
        }
    }

    /// Calls Swift's `Array.count` getter with explicit element metadata.
    ///
    /// # Safety
    ///
    /// `element` must be the Swift type metadata for this array's element type.
    #[inline]
    pub unsafe fn swift_count_with_metadata(&self, element: *const TypeMetadata) -> Int {
        unsafe { abi::array_count(self.as_raw().cast_const(), element) }
    }

    #[inline]
    fn elements(&self) -> *const T {
        unsafe { core::ptr::addr_of!((*self.storage.as_ptr()).elements).cast() }
    }
}

impl Array<Int> {
    #[inline]
    pub fn from_slice(values: &[Int]) -> Self {
        unsafe { Self::from_slice_with_metadata(values, abi::int_metadata()) }
    }

    #[inline]
    pub fn swift_count(&self) -> Int {
        unsafe { self.swift_count_with_metadata(abi::int_metadata()) }
    }
}

impl<T> Clone for Array<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            abi::bridge_object_retain(self.as_raw() as usize);
            Self::from_raw(self.as_raw())
        }
    }
}

impl<T> Drop for Array<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { abi::bridge_object_release(self.as_raw() as usize) }
    }
}

#[cfg(test)]
mod tests {
    use super::Array;
    use crate::swift::Int;

    #[test]
    fn int_array_roundtrips_native_storage() {
        let array = Array::<Int>::from_slice(&[1, 2, 3]);

        assert_eq!(3, array.len());
        assert_eq!(&[1, 2, 3], array.as_slice());
        assert_eq!(3, array.swift_count());
    }
}
