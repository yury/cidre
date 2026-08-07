use core::{fmt, marker::PhantomData};

use super::{
    FromSwift, SwiftHashable, SwiftMetadata, SwiftOptional, ToSwift, abi,
    value::{Optional, Storage},
};

/// An owned Swift `Dictionary` value.
///
/// One word in the Swift ABI, like [`Array`](super::Array) and
/// [`Set`](super::Set). A lookup is generic over `Key: Hashable`, so it carries
/// the key's conformance as well as both types' metadata — which is what
/// [`SwiftHashable`] supplies.
#[repr(transparent)]
pub struct Dictionary<K, V> {
    storage: usize,
    _marker: PhantomData<(K, V)>,
}

unsafe impl<K: Send, V: Send> Send for Dictionary<K, V> {}
unsafe impl<K: Sync, V: Sync> Sync for Dictionary<K, V> {}

impl<K, V> Dictionary<K, V> {
    /// Takes ownership of a raw Swift `Dictionary<K, V>` ABI value.
    ///
    /// # Safety
    ///
    /// `storage` must be a valid owned Swift `Dictionary<K, V>` value, and it
    /// must be valid to release with `swift_bridgeObjectRelease`.
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

impl<K, V> Dictionary<K, V> {
    /// The empty dictionary, which is Swift's own shared immortal storage for
    /// `[:]` rather than an allocation.
    #[inline]
    pub fn empty() -> Self {
        unsafe { Self::from_raw(abi::empty_dictionary_storage()) }
    }
}

impl<K, V> Default for Dictionary<K, V> {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}

impl<K: ToSwift + SwiftHashable, V: FromSwift + SwiftOptional> Dictionary<K, V> {
    /// Looks `key` up through Swift's generic `Dictionary.subscript` getter.
    pub fn get(&self, key: &K) -> Option<V> {
        unsafe {
            let mut out = Storage::<Optional<V>>::new();
            let mut lookup = |key: *const ()| {
                abi::dictionary_get(
                    key,
                    self.as_raw(),
                    K::metadata(),
                    V::metadata(),
                    K::hashable_witness(),
                    out.as_mut_ptr(),
                )
            };

            // The subscript only borrows its key. A key whose Rust type is the
            // Swift value is lent straight to it; one that merely names a Swift
            // value has to be materialized first, and destroyed after.
            match key.as_swift_ptr() {
                Some(key) => lookup(key),
                None => {
                    let mut scratch = Storage::<K>::new();
                    key.copy_to_swift(scratch.as_mut_ptr());
                    lookup(scratch.as_ptr());
                    scratch.destroy();
                }
            }

            out.take()
        }
    }

    #[inline]
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

/// A container cannot cache `Self?`; see [`Array`](super::Array).
unsafe impl<K: SwiftHashable, V: SwiftMetadata> super::SwiftOptional for Dictionary<K, V> {}

/// A dictionary is one word whatever it holds, so it is its own Swift value.
unsafe impl<K: SwiftHashable, V: SwiftMetadata> SwiftMetadata for Dictionary<K, V> {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        let key = K::metadata();
        let value = V::metadata();
        assert!(
            !key.is_null() && !value.is_null(),
            "Swift type metadata must exist"
        );
        unsafe { abi::dictionary_metadata(key, value, K::hashable_witness()) }
    }
}

unsafe impl<K: SwiftHashable, V: SwiftMetadata> super::SwiftType for Dictionary<K, V> {}

crate::impl_swift_memcpy_value!(Dictionary<K, V>, <K: SwiftHashable, V: SwiftMetadata>);

impl<K, V> Clone for Dictionary<K, V> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            abi::bridge_object_retain(self.storage);
            Self::from_raw(self.as_raw())
        }
    }
}

impl<K, V> Drop for Dictionary<K, V> {
    #[inline]
    fn drop(&mut self) {
        unsafe { abi::bridge_object_release(self.storage) }
    }
}

impl<K, V> fmt::Debug for Dictionary<K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("swift::Dictionary")
            .field(&self.as_raw())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Dictionary;
    use crate::swift;

    /// The lookup path — key metadata, the `Hashable` witness the runtime
    /// instantiates, and reading the `Value?` the subscript writes back — has to
    /// work on the one dictionary that can be built without Swift's help.
    #[test]
    fn an_empty_dictionary_finds_nothing() {
        let dictionary = Dictionary::<isize, isize>::empty();
        assert_eq!(None, dictionary.get(&7));
        assert!(!dictionary.contains_key(&0));

        // A nontrivial key and value must take the same path.
        let strings = Dictionary::<swift::String, swift::String>::empty();
        assert_eq!(None, strings.get(&swift::String::from("missing")));
    }

    /// A container-valued dictionary has to be lookupable too.
    ///
    /// `Optional<V>` needs `V` to name a cache for its own optional, and a
    /// generic container cannot — which is what stopped these from compiling at
    /// all, and what the uncached default now covers. This is mostly a
    /// compile-time assertion; that it also runs is a bonus.
    #[test]
    fn dict_of_arrays() {
        let of_arrays = Dictionary::<isize, swift::Array<isize>>::empty();
        assert!(of_arrays.get(&1).is_none());
    }

    #[test]
    fn dict_of_sets() {
        let of_sets = Dictionary::<isize, swift::Set<swift::String>>::empty();
        assert!(of_sets.get(&1).is_none());
    }

    #[test]
    fn dict_of_dicts() {
        let nested = Dictionary::<swift::String, Dictionary<isize, isize>>::empty();
        assert!(nested.get(&swift::String::from("k")).is_none());
    }
}
