use crate::{cf, define_cf_type};

define_cf_type!(UUID(cf::Type));

impl UUID {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFUUIDGetTypeID() }
    }

    /// CFUUIDCreate
    #[inline]
    pub fn new_in(alloc: Option<&cf::Allocator>) -> Option<cf::Retained<UUID>> {
        unsafe { CFUUIDCreate(alloc) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let uuid = cf::UUID::new();
    /// uuid.show();
    /// let uuid2 = cf::UUID::new();
    /// assert!(!uuid.equal(&uuid2));
    /// ```
    #[inline]
    pub fn new() -> cf::Retained<UUID> {
        unsafe { Self::new_in(None).unwrap_unchecked() }
    }

    #[inline]
    pub fn from_cf_string_in(
        uuid_str: &cf::String,
        alloc: Option<&cf::Allocator>,
    ) -> Option<cf::Retained<UUID>> {
        unsafe { CFUUIDCreateFromString(alloc, uuid_str) }
    }

    #[inline]
    pub fn to_cf_string_in(&self, alloc: Option<&cf::Allocator>) -> Option<cf::Retained<cf::String>> {
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
