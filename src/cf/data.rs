use crate::{cf, define_cf_type};

define_cf_type!(Data(cf::Type));
define_cf_type!(MutableData(Data));

impl Data {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFDataGetTypeID() }
    }

    pub fn create(
        allocator: Option<&cf::Allocator>,
        bytes: *const u8,
        length: cf::Index,
    ) -> Option<cf::Retained<cf::Data>> {
        unsafe { CFDataCreate(allocator, bytes, length) }
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
    pub unsafe fn create_mutable_copy(
        &self,
        allocator: Option<&cf::Allocator>,
        capacity: cf::Index,
    ) -> Option<cf::Retained<MutableData>> {
        CFDataCreateMutableCopy(allocator, capacity, self)
    }

    #[inline]
    pub fn mutable_copy(&self, capacity: usize) -> cf::Retained<MutableData> {
        unsafe {
            self.create_mutable_copy(None, capacity as _)
                .unwrap_unchecked()
        }
    }
}

impl MutableData {
    #[inline]
    pub fn create(
        allocator: Option<&cf::Allocator>,
        capacity: cf::Index,
    ) -> Option<cf::Retained<cf::MutableData>> {
        unsafe { CFDataCreateMutable(allocator, capacity) }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> cf::Retained<MutableData> {
        unsafe { Self::create(None, capacity as _).unwrap_unchecked() }
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
}

/// ```
/// use cidre::cf;
/// let data = cf::Retained::from(&[1u8][..]);
/// assert_eq!(data.len(), 1);
/// data.show();
/// ```
impl<'a> From<&[u8]> for cf::Retained<Data> {
    fn from(bytes: &[u8]) -> Self {
        unsafe { Data::create(None, bytes.as_ptr(), bytes.len() as _).unwrap_unchecked() }
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
}
