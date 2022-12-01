use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

use crate::{cf, define_cf_type};

define_cf_type!(Data(cf::Type));
define_cf_type!(MutableData(Data));

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
    ) -> Option<cf::Retained<cf::Data>> {
        unsafe { CFDataCreate(allocator, bytes, length) }
    }

    #[inline]
    pub fn new(bytes: *const u8, length: cf::Index) -> Option<cf::Retained<cf::Data>> {
        Self::new_in(bytes, length, None)
    }

    #[inline]
    pub fn length(&self) -> cf::Index {
        unsafe { CFDataGetLength(self) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length() as _
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn mutable_copy_in(
        &self,
        capacity: cf::Index,
        allocator: Option<&cf::Allocator>,
    ) -> Option<cf::Retained<MutableData>> {
        unsafe { CFDataCreateMutableCopy(allocator, capacity, self) }
    }

    #[inline]
    pub fn mutable_copy(&self, capacity: usize) -> cf::Retained<MutableData> {
        unsafe { self.mutable_copy_in(capacity as _, None).unwrap_unchecked() }
    }

    #[inline]
    pub fn bytes_ptr(&self) -> *const u8 {
        unsafe { CFDataGetBytePtr(self) }
    }

    #[inline]
    pub fn copy_bytes(&self, offset: usize, buffer: &mut [u8]) {
        unsafe {
            CFDataGetBytes(
                self,
                cf::Range::new(offset as _, buffer.len() as _),
                buffer.as_mut_ptr(),
            )
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { &*slice_from_raw_parts(self.as_ptr() as _, self.len()) }
    }
}

impl MutableData {
    #[inline]
    pub fn new_in(
        capacity: cf::Index,
        allocator: Option<&cf::Allocator>,
    ) -> Option<cf::Retained<cf::MutableData>> {
        unsafe { CFDataCreateMutable(allocator, capacity) }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> cf::Retained<MutableData> {
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
        unsafe { &mut *slice_from_raw_parts_mut(self.bytes_ptr_mut() as *mut u8, self.len()) }
    }
}

/// ```
/// use cidre::cf;
/// let data = cf::Retained::from(&[1u8][..]);
/// assert_eq!(data.len(), 1);
/// data.show();
/// ```
impl From<&[u8]> for cf::Retained<Data> {
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
    ) -> Option<cf::Retained<cf::Data>>;
    fn CFDataGetLength(data: &Data) -> cf::Index;
    fn CFDataCreateMutable(
        allocator: Option<&cf::Allocator>,
        capacity: cf::Index,
    ) -> Option<cf::Retained<cf::MutableData>>;
    fn CFDataAppendBytes(data: &MutableData, bytes: *const u8, length: cf::Index);
    fn CFDataCreateMutableCopy(
        allocator: Option<&cf::Allocator>,
        capacity: cf::Index,
        data: &Data,
    ) -> Option<cf::Retained<MutableData>>;

    fn CFDataGetBytePtr(data: &cf::Data) -> *const u8;
    fn CFDataGetBytes(data: &cf::Data, range: cf::Range, buffer: *mut u8);

    fn CFDataGetMutableBytePtr(data: &mut cf::MutableData) -> *mut u8;
    fn CFDataSetLength(data: &mut cf::MutableData, length: cf::Index);
}
