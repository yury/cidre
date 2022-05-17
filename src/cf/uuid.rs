use crate::{cf, define_cf_type};

define_cf_type!(UUID(cf::Type));

impl UUID {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFUUIDGetTypeID() }
    }

    #[inline]
    pub fn create<'a>(alloc: Option<&cf::Allocator>) -> Option<cf::Retained<'a, UUID>> {
        unsafe { CFUUIDCreate(alloc) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let uuid = cf::UUID::new();
    /// uuid.show();
    /// ```
    #[inline]
    pub fn new<'a>() -> cf::Retained<'a, UUID> {
        unsafe { Self::create(None).unwrap_unchecked() }
    }

    #[inline]
    pub fn from_cf_string<'a>(
        alloc: Option<&cf::Allocator>,
        uuid_str: &cf::String,
    ) -> Option<cf::Retained<'a, UUID>> {
        unsafe { CFUUIDCreateFromString(alloc, uuid_str) }
    }

    #[inline]
    pub fn to_cf_string<'a>(
        &self,
        alloc: Option<&cf::Allocator>,
    ) -> Option<cf::Retained<'a, cf::String>> {
        unsafe { CFUUIDCreateString(alloc, self) }
    }
}

extern "C" {
    fn CFUUIDGetTypeID() -> cf::TypeId;
    fn CFUUIDCreate<'a>(alloc: Option<&cf::Allocator>) -> Option<cf::Retained<'a, UUID>>;
    fn CFUUIDCreateFromString<'a>(
        alloc: Option<&cf::Allocator>,
        uuid_str: &cf::String,
    ) -> Option<cf::Retained<'a, UUID>>;
    fn CFUUIDCreateString<'a>(
        alloc: Option<&cf::Allocator>,
        uuid: &UUID,
    ) -> Option<cf::Retained<'a, cf::String>>;
}
