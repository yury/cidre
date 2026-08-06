use core::{fmt, marker::PhantomData};

use super::{Array, SwiftHashable, SwiftMetadata, ToSwift, abi};

/// An owned Swift `Set` value.
///
/// Like [`Array`], a set is one word in the Swift ABI, so this keeps that
/// representation. Building one goes through `Set.init(arrayLiteral:)`, which
/// is what Swift itself calls for a set literal, and needs the element's
/// `Hashable` conformance — hence the [`SwiftHashable`] bound.
#[repr(transparent)]
pub struct Set<T> {
    storage: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Set<T> {}
unsafe impl<T: Sync> Sync for Set<T> {}

impl<T> Set<T> {
    /// Takes ownership of a raw Swift `Set<T>` ABI value.
    ///
    /// # Safety
    ///
    /// `storage` must be a valid owned Swift `Set<T>` value, and it must be
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

impl<T> Set<T> {
    /// The empty set, which is Swift's own shared immortal storage rather than
    /// an allocation.
    #[inline]
    pub fn empty() -> Self {
        unsafe { Self::from_raw(abi::empty_set_storage()) }
    }
}

impl<T> Default for Set<T> {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl<T: ToSwift + SwiftHashable> Set<T> {
    /// Builds a set from the values, dropping duplicates the way Swift does.
    #[inline]
    pub fn from_slice(values: &[T]) -> Self {
        Self::from_array(Array::from_slice(values))
    }

    /// Builds a set from the values the iterator yields.
    #[inline]
    pub fn from_iter(values: impl ExactSizeIterator<Item = T>) -> Self {
        Self::from_array(Array::from_iter(values))
    }

    /// Builds a set from an array, consuming it, which is exactly what Swift's
    /// own set literal does.
    pub fn from_array(values: Array<T>) -> Self {
        unsafe {
            let metadata = T::metadata();
            assert!(!metadata.is_null(), "Swift type metadata must exist");
            Self::from_raw(abi::set_from_array(
                values.into_raw(),
                metadata,
                T::hashable_witness(),
            ))
        }
    }
}

/// A set is one word whatever it holds, so it is its own Swift value.
unsafe impl<T: SwiftMetadata> SwiftMetadata for Set<T> {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        let element = T::metadata();
        assert!(!element.is_null(), "Swift type metadata must exist");
        unsafe { abi::set_metadata(element) }
    }
}

unsafe impl<T: SwiftMetadata> super::SwiftType for Set<T> {}

crate::impl_swift_memcpy_value!(Set<T>, <T: SwiftMetadata>);

impl<T> Clone for Set<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            abi::bridge_object_retain(self.storage);
            Self::from_raw(self.as_raw())
        }
    }
}

impl<T> Drop for Set<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { abi::bridge_object_release(self.storage) }
    }
}

impl<T> fmt::Debug for Set<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("swift::Set").field(&self.as_raw()).finish()
    }
}

impl<T: ToSwift + SwiftHashable> From<&[T]> for Set<T> {
    #[inline]
    fn from(values: &[T]) -> Self {
        Self::from_slice(values)
    }
}

impl<T: ToSwift + SwiftHashable, const N: usize> From<&[T; N]> for Set<T> {
    #[inline]
    fn from(values: &[T; N]) -> Self {
        Self::from_slice(values)
    }
}

#[cfg(test)]
mod tests {
    use super::Set;
    use crate::swift;

    /// Building a set exercises the `Hashable` witness the runtime instantiates
    /// from the standard library's conformance descriptor.
    #[test]
    fn a_set_is_built_through_the_hashable_conformance() {
        let set = Set::<isize>::from_slice(&[1, 2, 2, 3]);
        assert!(!set.as_raw().is_null());
        assert_ne!(set.as_raw(), Set::<isize>::empty().as_raw());

        // A nontrivial element type has to go the same way.
        let strings = Set::from_slice(&[swift::String::from("alpha"), swift::String::from("beta")]);
        assert!(!strings.as_raw().is_null());

        // Cloning is a retain of the same storage, and dropping both is what
        // would double-release if it were not.
        let clone = strings.clone();
        drop(strings);
        assert!(!clone.as_raw().is_null());
    }
}
