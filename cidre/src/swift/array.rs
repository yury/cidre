use core::{fmt, marker::PhantomData};

use super::{FromSwift, SwiftMetadata, SwiftType, ToSwift, abi, value::Storage};

/// An owned Swift `Array` value.
///
/// The array keeps its native one-word Swift ABI representation. Element
/// access and construction go through Swift standard-library ABI entries and
/// value witnesses, so this holds any element a binding can name: primitives,
/// class references, nontrivial values such as [`super::String`], and the
/// wrappers around values whose layout is only known at runtime.
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

impl<T: SwiftMetadata> Array<T> {
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

    /// The element count.
    ///
    /// Swift reads this straight out of the buffer header — an optimized
    /// `a.count` on a native array is one `ldr` at [`ARRAY_COUNT_OFFSET`], and
    /// being loop-invariant it usually leaves the loop entirely. So does this,
    /// falling back to the `count` getter only for a buffer bridged from
    /// `NSArray`, which has no header to read.
    ///
    /// [`ARRAY_COUNT_OFFSET`]: abi::ARRAY_COUNT_OFFSET
    #[inline]
    pub fn len(&self) -> usize {
        match self.native_count() {
            Some(count) => count,
            None => self.swift_count(),
        }
    }

    /// The count as Swift's own getter reports it, which is the authority for
    /// a bridged buffer and the check the header read is validated against.
    #[inline]
    fn swift_count(&self) -> usize {
        let count =
            unsafe { abi::array_count(self.as_raw().cast_const(), Self::element_metadata()) };
        debug_assert!(count >= 0);
        count as usize
    }

    /// The address of the first element in native storage, or `None` when
    /// there is no buffer to address.
    ///
    /// The stride still has to be Swift's, which is what
    /// [`FromSwift::IS_BITWISE_COPY`](crate::swift::FromSwift::IS_BITWISE_COPY)
    /// promises for the types that reach this.
    #[inline]
    fn native_elements(&self) -> Option<*const u8> {
        let storage = self.storage;
        if storage == 0 || storage & abi::ARRAY_BRIDGED_TAG != 0 || !header_layout_is_current() {
            return None;
        }
        let base = (storage & abi::ARRAY_STORAGE_MASK) as *const u8;
        Some(unsafe { base.add(abi::ARRAY_ELEMENTS_OFFSET) })
    }

    /// The count read out of native storage, or `None` when there is none to
    /// read: an `NSArray` buffer, or a header that has moved.
    #[inline]
    fn native_count(&self) -> Option<usize> {
        let storage = self.storage;
        if storage == 0 || storage & abi::ARRAY_BRIDGED_TAG != 0 || !header_layout_is_current() {
            return None;
        }
        let base = (storage & abi::ARRAY_STORAGE_MASK) as *const u8;
        Some(unsafe { base.add(abi::ARRAY_COUNT_OFFSET).cast::<usize>().read() })
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Whether native array storage still keeps its count where
/// [`abi::ARRAY_COUNT_OFFSET`] says.
///
/// The header is the standard library's own business, so rather than trust the
/// offset, check it once against the authority — Swift's `count` getter, on an
/// array built here for the purpose — and cache the answer for the process.
/// Every reader falls back to the getter if it ever stops matching, so a
/// standard library that moves its header costs speed rather than correctness.
///
/// Deliberately checked once globally rather than per element type: the header
/// is a `HeapObject` and an `_ArrayBody`, neither of which depends on what the
/// array holds.
fn header_layout_is_current() -> bool {
    use core::sync::atomic::{AtomicU8, Ordering::Relaxed};
    const UNKNOWN: u8 = 0;
    const CURRENT: u8 = 1;
    const MOVED: u8 = 2;

    static STATE: AtomicU8 = AtomicU8::new(UNKNOWN);

    match STATE.load(Relaxed) {
        CURRENT => true,
        MOVED => false,
        _ => {
            // A count no plausible stray word would equal, so a header that has
            // moved reads as moved rather than passing by coincidence.
            let probe = Array::<isize>::from_slice(&[-7; 23]);
            let stored = unsafe {
                ((probe.storage & abi::ARRAY_STORAGE_MASK) as *const u8)
                    .add(abi::ARRAY_COUNT_OFFSET)
                    .cast::<usize>()
                    .read()
            };
            let current =
                probe.storage & abi::ARRAY_BRIDGED_TAG == 0 && stored == probe.swift_count();
            STATE.store(if current { CURRENT } else { MOVED }, Relaxed);
            current
        }
    }
}

impl<T: ToSwift> Array<T> {
    /// Allocates a Swift array and copies the values through `T`'s Swift value
    /// witness table.
    #[inline]
    pub fn from_slice(values: &[T]) -> Self {
        unsafe {
            let metadata = Self::element_metadata();
            let (storage, elements) = abi::allocate_uninitialized_array(values.len(), metadata);

            if T::IS_CONTIGUOUS {
                // The Rust slice is already laid out at the Swift stride, so
                // the runtime can copy the whole run in one call.
                debug_assert_eq!(size_of::<T>(), abi::value_layout(metadata).stride);
                abi::array_initialize_with_copy(
                    elements,
                    values.as_ptr().cast(),
                    values.len(),
                    metadata,
                );
            } else {
                write_elements::<T, _, _>(elements, metadata, values.iter());
            }

            Self::from_raw(storage)
        }
    }

    /// Allocates a Swift array and copies in the values the iterator yields.
    ///
    /// Building the buffer in place is what keeps a caller that has to make its
    /// elements — retaining borrowed references, reading each one out of Swift —
    /// from collecting them into a `Vec` first.
    pub fn from_iter(values: impl ExactSizeIterator<Item = T>) -> Self {
        unsafe {
            let metadata = Self::element_metadata();
            let (storage, elements) = abi::allocate_uninitialized_array(values.len(), metadata);
            write_elements::<T, _, _>(elements, metadata, values);
            Self::from_raw(storage)
        }
    }
}

impl<T: FromSwift> Array<T> {
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
        // Plain data in native storage is just a load, which is what Swift's
        // own subscript compiles to. The branch folds away for element types
        // that cannot take it.
        if T::IS_BITWISE_COPY
            && let Some(base) = self.native_elements()
        {
            debug_assert_eq!(
                core::mem::size_of::<T>(),
                unsafe { abi::value_layout(Self::element_metadata()) }.stride,
                "a bitwise-copy element must be laid out at Swift's stride"
            );
            return unsafe { base.cast::<T>().add(index).read() };
        }

        unsafe {
            // The subscript hands back an owned element, so the scratch buffer
            // is only there for `T` to take it out of.
            let mut scratch = Storage::<T>::new();
            abi::array_get(
                self.as_raw().cast_const(),
                index as isize,
                scratch.as_mut_ptr(),
                Self::element_metadata(),
            );
            T::take_swift(scratch.as_mut_ptr())
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

impl<T: SwiftType> Array<T> {
    /// Borrows the elements as a Rust slice, when the array is backed by native
    /// Swift storage.
    ///
    /// This is what makes `array[i]` possible without a Swift call per element:
    /// take the slice once and index it. Only an element whose Rust type *is*
    /// the Swift value can be borrowed this way, which is what [`SwiftType`]
    /// states. Note that it borrows rather than copies, which is why
    /// [`std::ops::Index`] is deliberately not implemented on `Array` itself —
    /// Swift's own subscript hands back an owned `+1` element, and an array
    /// bridged from `NSArray` has no buffer to borrow at all, which `Index`
    /// would have no way to report.
    ///
    /// Returns `None` for a bridged array, for an element type Swift lays out
    /// differently than Rust does, and if the standard library's array header
    /// ever moves.
    pub fn as_slice(&self) -> Option<&[T]> {
        // Native storage whose header is where it should be, which is what
        // makes the count below readable and the elements addressable.
        let count = self.native_count()?;

        // `SwiftType` promises the layouts match, but the array is only
        // borrowable as `[T]` if Swift also strides it the way Rust does.
        let layout = unsafe { abi::value_layout(Self::element_metadata()) };
        if layout.stride != core::mem::size_of::<T>() || layout.align > 16 {
            return None;
        }

        unsafe {
            let base = (self.storage & abi::ARRAY_STORAGE_MASK) as *const u8;
            Some(core::slice::from_raw_parts(
                base.add(abi::ARRAY_ELEMENTS_OFFSET).cast::<T>(),
                count,
            ))
        }
    }
}

/// Copies each value into its slot of a freshly allocated element buffer.
///
/// # Safety
///
/// `elements` must be uninitialized storage for as many values of `metadata`
/// as the iterator yields.
unsafe fn write_elements<T, B, I>(elements: *mut (), metadata: *const abi::TypeMetadata, values: I)
where
    T: ToSwift,
    B: core::borrow::Borrow<T>,
    I: Iterator<Item = B>,
{
    unsafe {
        let stride = abi::value_layout(metadata).stride;
        for (index, value) in values.enumerate() {
            value
                .borrow()
                .copy_to_swift(elements.cast::<u8>().add(index * stride).cast());
        }
    }
}

/// An array is one word whatever it holds, so it is its own Swift value.
unsafe impl<T: SwiftMetadata> SwiftMetadata for Array<T> {
    #[inline]
    fn metadata() -> *const abi::TypeMetadata {
        unsafe { abi::array_metadata(Self::element_metadata()) }
    }
}

unsafe impl<T: SwiftMetadata> SwiftType for Array<T> {}

crate::impl_swift_memcpy_value!(Array<T>, <T: SwiftMetadata>);

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

impl<T: FromSwift + fmt::Debug> fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}

impl<T: FromSwift + PartialEq> PartialEq for Array<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs == rhs)
    }
}

impl<T: FromSwift + Eq> Eq for Array<T> {}

impl<T: ToSwift> From<&[T]> for Array<T> {
    #[inline]
    fn from(values: &[T]) -> Self {
        Self::from_slice(values)
    }
}

impl<T: ToSwift, const N: usize> From<&[T; N]> for Array<T> {
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

impl<T: FromSwift> Iterator for ArrayIter<'_, T> {
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

impl<T: FromSwift> DoubleEndedIterator for ArrayIter<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.range
            .next_back()
            .map(|index| unsafe { self.array.get_unchecked(index) })
    }
}

impl<T: FromSwift> ExactSizeIterator for ArrayIter<'_, T> {}
impl<T: FromSwift> core::iter::FusedIterator for ArrayIter<'_, T> {}

impl<'a, T: FromSwift> IntoIterator for &'a Array<T> {
    type Item = T;
    type IntoIter = ArrayIter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::swift::{Array, String};

    /// The borrowed buffer has to agree with what Swift's own subscript
    /// returns, element for element — that is the whole guard on the hardcoded
    /// header offsets.
    #[test]
    fn a_borrowed_slice_agrees_with_the_swift_subscript() {
        let values: Vec<isize> = (0..64).map(|i| i * 7 - 13).collect();
        let array = Array::<isize>::from_slice(&values);

        let slice = array.as_slice().expect("a Rust-built array is native");
        assert_eq!(values.len(), slice.len());
        assert_eq!(values.as_slice(), slice);

        for index in 0..array.len() {
            assert_eq!(
                array.get(index).unwrap(),
                slice[index],
                "element {index} disagrees with Swift's subscript"
            );
        }
    }

    /// The header read and the direct element read have to agree with Swift's
    /// own getters, which is the whole warrant for skipping them.
    ///
    /// `as_slice` and `get` both take the fast path now, so comparing those two
    /// against each other no longer proves anything — the authority has to be
    /// Swift's `count` and `subscript`, called here on purpose.
    #[test]
    fn the_fast_path_agrees_with_swifts_own_getters() {
        use crate::swift::{SwiftMetadata, abi};

        for len in [0usize, 1, 2, 7, 64, 1000] {
            let values: Vec<isize> = (0..len as isize).map(|i| i * 31 - 17).collect();
            let array = Array::<isize>::from_slice(&values);

            let swift_count =
                unsafe { abi::array_count(array.as_raw().cast_const(), isize::metadata()) };
            assert_eq!(
                swift_count as usize,
                array.len(),
                "count disagrees at {len}"
            );
            assert_eq!(len, array.len());

            for index in 0..len {
                // Swift's subscript, called rather than skipped.
                let mut from_swift = 0isize;
                unsafe {
                    abi::array_get(
                        array.as_raw().cast_const(),
                        index as isize,
                        core::ptr::from_mut(&mut from_swift).cast(),
                        isize::metadata(),
                    );
                }
                assert_eq!(from_swift, unsafe { array.get_unchecked(index) });
                assert_eq!(from_swift, values[index]);
            }
        }
    }

    /// A nontrivial element must never take the bitwise path, or the copy is a
    /// reference nobody retained.
    #[test]
    fn nontrivial_elements_are_not_bitwise_copyable() {
        use crate::swift::FromSwift;

        assert!(<isize as FromSwift>::IS_BITWISE_COPY);
        assert!(!<String as FromSwift>::IS_BITWISE_COPY);
        assert!(!<Array<isize> as FromSwift>::IS_BITWISE_COPY);

        // Reading the same element repeatedly must keep the array's own
        // reference intact, which a stray bitwise copy would not.
        let array = Array::<String>::from_slice(&[String::from("held")]);
        for _ in 0..64 {
            assert_eq!("held", array.get(0).unwrap().to_string());
        }
        assert_eq!("held", array.get(0).unwrap().to_string());
    }

    /// An empty array is the standard library's shared singleton, which still
    /// has to produce a valid empty slice rather than a wild pointer.
    #[test]
    fn an_empty_array_borrows_as_an_empty_slice() {
        let array = Array::<isize>::from_slice(&[]);
        assert_eq!(Some(&[][..]), array.as_slice());
    }

    /// A nontrivial element type: the slice must borrow, so the array still
    /// owns every element and nothing is released twice.
    #[test]
    fn a_slice_of_nontrivial_values_only_borrows() {
        let array = Array::<String>::from_slice(&[
            String::from("alpha"),
            String::from("beta"),
            String::from("gamma"),
        ]);

        {
            let slice = array.as_slice().expect("native storage");
            assert_eq!(3, slice.len());
            assert_eq!("beta", slice[1].to_string());
        }

        // If the borrow had taken ownership, this would read released strings.
        assert_eq!("alpha", array.get(0).unwrap().to_string());
        assert_eq!("gamma", array.get(2).unwrap().to_string());
    }

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

/// A container cannot name a cache for `Self?`: a `static` written here would
/// be shared by every element type rather than one per instantiation, so the
/// first `Array<T>?` resolved would answer for all of them. The default
/// resolves each time instead, which is correct and no slower than before the
/// cache existed.
unsafe impl<T: SwiftMetadata> super::SwiftOptional for Array<T> {}
