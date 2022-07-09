use crate::{cf, define_cf_type};

define_cf_type!(UUID(cf::Type));

impl UUID {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFUUIDGetTypeID() }
    }

    #[inline]
    pub fn create(alloc: Option<&cf::Allocator>) -> Option<cf::Retained<UUID>> {
        unsafe { CFUUIDCreate(alloc) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let uuid = cf::UUID::new();
    /// uuid.show();
    /// ```
    #[inline]
    pub fn new() -> cf::Retained<UUID> {
        unsafe { Self::create(None).unwrap_unchecked() }
    }

    #[inline]
    pub fn from_cf_string(
        alloc: Option<&cf::Allocator>,
        uuid_str: &cf::String,
    ) -> Option<cf::Retained<UUID>> {
        unsafe { CFUUIDCreateFromString(alloc, uuid_str) }
    }

    #[inline]
    pub fn to_cf_string(&self, alloc: Option<&cf::Allocator>) -> Option<cf::Retained<cf::String>> {
        unsafe { CFUUIDCreateString(alloc, self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFUUIDGetTypeID() -> cf::TypeId;
    fn CFUUIDCreate(alloc: Option<&cf::Allocator>) -> Option<cf::Retained<UUID>>;
    fn CFUUIDCreateFromString(
        alloc: Option<&cf::Allocator>,
        uuid_str: &cf::String,
    ) -> Option<cf::Retained<UUID>>;
    fn CFUUIDCreateString(
        alloc: Option<&cf::Allocator>,
        uuid: &UUID,
    ) -> Option<cf::Retained<cf::String>>;
}
