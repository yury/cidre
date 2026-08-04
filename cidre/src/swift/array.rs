use core::{fmt, marker::PhantomData, mem::MaybeUninit};

use super::{SwiftMetadata, SwiftType, abi};

/// An owned Swift `Array` value.
///
/// The array keeps its native one-word Swift ABI representation. Element
/// access and construction go through Swift standard-library ABI entries and
/// value witnesses, so this also supports nontrivial values such as
/// [`super::String`].
#[repr(transparent)]
pub struct Array<T> {
    storage: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Array<T> {}
unsafe impl<T: Sync> Sync for Array<T> {}

impl<T> Array<T> {
    /// Takes ownership of a raw Swift `Array<T>` ABI value.
    ///
    /// # Safety
    ///
    /// `storage` must be a valid owned Swift `Array<T>` value, and it must be
    /// valid to release with `swift_bridgeObjectRelease`.
    #[inline]
    pub unsafe fn from_raw(storage: *mut ()) -> Self {
        Self {
            storage: storage as usize,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn as_raw(&self) -> *mut () {
        self.storage as *mut ()
    }

    #[inline]
    pub fn into_raw(self) -> *mut () {
        let raw = self.as_raw();
        core::mem::forget(self);
        raw
    }
}

impl<T: SwiftType> Array<T> {
    /// `T`'s Swift metadata.
    ///
    /// Some element types resolve their metadata through a runtime mangled-name
    /// lookup, which returns null when the name does not resolve. Every use
    /// below dereferences the metadata, so check it once here instead of
    /// faulting inside a value witness.
    #[inline]
    fn element_metadata() -> *const abi::TypeMetadata {
        let metadata = T::metadata();
        assert!(!metadata.is_null(), "Swift type metadata must exist");
        metadata
    }

    /// Allocates a Swift array and copies the values through `T`'s Swift value
    /// witness table.
    #[inline]
    pub fn from_slice(values: &[T]) -> Self {
        unsafe {
            let metadata = Self::element_metadata();
            let (storage, elements) = abi::allocate_uninitialized_array(values.len(), metadata);

            // `SwiftType` guarantees `T` matches the Swift type's size and
            // alignment, so a Rust slice is already laid out at the Swift
            // stride and the runtime can copy the whole run in one call.
            debug_assert_eq!(
                core::mem::size_of::<T>(),
                abi::value_layout(metadata).stride
            );
            abi::array_initialize_with_copy(
                elements,
                values.as_ptr().cast(),
                values.len(),
                metadata,
            );

            Self::from_raw(storage)
        }
    }

    /// Returns the element count through Swift's `Array.count` getter.
    #[inline]
    pub fn len(&self) -> usize {
        let count =
            unsafe { abi::array_count(self.as_raw().cast_const(), Self::element_metadata()) };
        debug_assert!(count >= 0);
        count as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Copies one element through Swift's generic `Array.subscript` getter.
    #[inline]
    pub fn get(&self, index: usize) -> Option<T> {
        if index >= self.len() {
            return None;
        }

        Some(unsafe { self.get_unchecked(index) })
    }

    /// Copies one element without checking that `index` is in bounds.
    ///
    /// # Safety
    ///
    /// `index` must be less than [`Self::len`].
    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> T {
        let mut value = MaybeUninit::<T>::uninit();
        unsafe {
            abi::array_get(
                self.as_raw().cast_const(),
                index as isize,
                value.as_mut_ptr().cast(),
                Self::element_metadata(),
            );
            value.assume_init()
        }
    }

    #[inline]
    pub fn iter(&self) -> ArrayIter<'_, T> {
        ArrayIter {
            array: self,
            range: 0..self.len(),
        }
    }

    #[inline]
    pub fn to_vec(&self) -> Vec<T> {
        self.iter().collect()
    }
}

unsafe impl<T: SwiftType> SwiftMetadata for Array<T> {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        unsafe { abi::array_metadata(Self::element_metadata()) }
    }
}

unsafe impl<T: SwiftType> SwiftType for Array<T> {}

impl<T> Clone for Array<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            abi::bridge_object_retain(self.storage);
            Self::from_raw(self.as_raw())
        }
    }
}

impl<T> Drop for Array<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { abi::bridge_object_release(self.storage) }
    }
}

impl<T: SwiftType + fmt::Debug> fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: SwiftType + PartialEq> PartialEq for Array<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs == rhs)
    }
}

impl<T: SwiftType + Eq> Eq for Array<T> {}

impl<T: SwiftType> From<&[T]> for Array<T> {
    #[inline]
    fn from(values: &[T]) -> Self {
        Self::from_slice(values)
    }
}

impl<T: SwiftType, const N: usize> From<&[T; N]> for Array<T> {
    #[inline]
    fn from(values: &[T; N]) -> Self {
        Self::from_slice(values)
    }
}

/// An iterator that copies values out of a Swift `Array` through its subscript
/// getter.
pub struct ArrayIter<'a, T> {
    array: &'a Array<T>,
    range: core::ops::Range<usize>,
}

impl<T: SwiftType> Iterator for ArrayIter<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.range
            .next()
            .map(|index| unsafe { self.array.get_unchecked(index) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<T: SwiftType> DoubleEndedIterator for ArrayIter<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range
            .next_back()
            .map(|index| unsafe { self.array.get_unchecked(index) })
    }
}

impl<T: SwiftType> ExactSizeIterator for ArrayIter<'_, T> {}
impl<T: SwiftType> core::iter::FusedIterator for ArrayIter<'_, T> {}

impl<'a, T: SwiftType> IntoIterator for &'a Array<T> {
    type Item = T;
    type IntoIter = ArrayIter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::Array;
    use crate::swift::String;

    #[test]
    fn int_array_uses_swift_subscript() {
        let array = Array::<isize>::from_slice(&[1, 2, 3]);

        assert_eq!(3, array.len());
        assert_eq!(Some(1), array.get(0));
        assert_eq!(Some(3), array.get(2));
        assert_eq!(None, array.get(3));
        assert_eq!(vec![1, 2, 3], array.to_vec());
    }

    #[test]
    fn string_array_copies_nontrivial_values_with_value_witnesses() {
        let array = {
            let values = [
                String::from("small"),
                String::from("a longer Swift string from Rust 🦀"),
            ];
            Array::from_slice(&values)
        };

        assert_eq!(
            vec![
                std::string::String::from("small"),
                std::string::String::from("a longer Swift string from Rust 🦀"),
            ],
            array
                .iter()
                .map(|value| value.to_string())
                .collect::<Vec<_>>()
        );

        let clone = array.clone();
        drop(array);
        assert_eq!(
            "a longer Swift string from Rust 🦀",
            clone.get(1).unwrap().to_string()
        );
    }

    #[test]
    fn nested_array_uses_generic_metadata_accessor() {
        let first = Array::<isize>::from_slice(&[1, 2]);
        let second = Array::<isize>::from_slice(&[3]);
        let nested = Array::from_slice(&[first, second]);

        assert_eq!(vec![1, 2], nested.get(0).unwrap().to_vec());
        assert_eq!(vec![3], nested.get(1).unwrap().to_vec());
    }
}
