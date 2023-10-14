use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

use crate::{arc, cf, define_cf_type};

#[cfg(feature = "ns")]
use crate::ns;

define_cf_type!(Data(cf::Type));
define_cf_type!(DataMut(Data));

impl Data {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFDataGetTypeID() }
    }

    #[inline]
    pub fn new_in(
        bytes: *const u8,
        length: cf::Index,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::Data>> {
        unsafe { CFDataCreate(allocator, bytes, length) }
    }

    #[inline]
    pub fn new(bytes: *const u8, length: cf::Index) -> Option<arc::R<cf::Data>> {
        Self::new_in(bytes, length, None)
    }

    #[doc(alias = "length")]
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { CFDataGetLength(self) as _ }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn copy_mut_in(
        &self,
        capacity: cf::Index,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<DataMut>> {
        unsafe { CFDataCreateMutableCopy(allocator, capacity, self) }
    }

    #[inline]
    pub fn copy_mut(&self, capacity: usize) -> arc::R<DataMut> {
        unsafe { self.copy_mut_in(capacity as _, None).unwrap_unchecked() }
    }

    #[inline]
    pub fn bytes_ptr(&self) -> *const u8 {
        unsafe { CFDataGetBytePtr(self) }
    }

    #[inline]
    pub fn copy_bytes(&self, buffer: &mut [u8]) {
        unsafe { self.get_bytes(cf::Range::new(0, buffer.len() as _), buffer.as_mut_ptr()) }
    }

    #[inline]
    pub unsafe fn get_bytes(&self, range: cf::Range, buffer: *mut u8) {
        CFDataGetBytes(self, range, buffer)
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { &*slice_from_raw_parts(self.bytes_ptr() as _, self.len()) }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns(&self) -> &ns::Data {
        unsafe { std::mem::transmute(self) }
    }
}

impl DataMut {
    #[inline]
    pub fn new_in(
        capacity: cf::Index,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::DataMut>> {
        unsafe { CFDataCreateMutable(allocator, capacity) }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> arc::R<DataMut> {
        unsafe { Self::new_in(capacity as _, None).unwrap_unchecked() }
    }

    /// # Unsafety
    ///
    /// use [`.push_bytes()`]
    #[inline]
    pub unsafe fn append_bytes(&mut self, bytes: *const u8, length: cf::Index) {
        CFDataAppendBytes(self, bytes, length)
    }

    #[inline]
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        unsafe { self.append_bytes(bytes.as_ptr(), bytes.len() as _) }
    }

    /// use `as_slice_mut()`
    #[inline]
    pub unsafe fn bytes_ptr_mut(&mut self) -> *mut u8 {
        CFDataGetMutableBytePtr(self)
    }

    #[inline]
    pub fn set_len(&mut self, len: cf::Index) {
        unsafe { CFDataSetLength(self, len) }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *slice_from_raw_parts_mut(self.bytes_ptr_mut(), self.len()) }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns_mut(&mut self) -> &mut ns::DataMut {
        unsafe { std::mem::transmute(self) }
    }
}

/// ```
/// use cidre::{arc, cf};
/// let data: arc::R<cf::Data> = arc::R::from(&[1u8][..]);
/// assert_eq!(data.len(), 1);
/// data.show();
/// ```
impl From<&[u8]> for arc::R<Data> {
    fn from(bytes: &[u8]) -> Self {
        unsafe { Data::new(bytes.as_ptr(), bytes.len() as _).unwrap_unchecked() }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFDataGetTypeID() -> cf::TypeId;
    fn CFDataCreate(
        allocator: Option<&cf::Allocator>,
        bytes: *const u8,
        length: cf::Index,
    ) -> Option<arc::R<cf::Data>>;
    fn CFDataGetLength(data: &Data) -> cf::Index;
    fn CFDataCreateMutable(
        allocator: Option<&cf::Allocator>,
        capacity: cf::Index,
    ) -> Option<arc::R<cf::DataMut>>;
    fn CFDataAppendBytes(data: &DataMut, bytes: *const u8, length: cf::Index);
    fn CFDataCreateMutableCopy(
        allocator: Option<&cf::Allocator>,
        capacity: cf::Index,
        data: &Data,
    ) -> Option<arc::R<DataMut>>;

    fn CFDataGetBytePtr(data: &cf::Data) -> *const u8;
    fn CFDataGetBytes(data: &cf::Data, range: cf::Range, buffer: *mut u8);

    fn CFDataGetMutableBytePtr(data: &mut cf::DataMut) -> *mut u8;
    fn CFDataSetLength(data: &mut cf::DataMut, length: cf::Index);
}
