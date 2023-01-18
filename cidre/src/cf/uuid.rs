use crate::{arc, cf, define_cf_type};

define_cf_type!(UUID(cf::Type));

impl UUID {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFUUIDGetTypeID() }
    }

    /// CFUUIDCreate
    #[inline]
    pub fn new_in(alloc: Option<&cf::Allocator>) -> Option<arc::R<UUID>> {
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
    pub fn new() -> arc::R<UUID> {
        unsafe { Self::new_in(None).unwrap_unchecked() }
    }

    #[inline]
    pub fn from_cf_string_in(
        uuid_str: &cf::String,
        alloc: Option<&cf::Allocator>,
    ) -> Option<arc::R<UUID>> {
        unsafe { CFUUIDCreateFromString(alloc, uuid_str) }
    }

    #[inline]
    pub fn to_cf_string_in(&self, alloc: Option<&cf::Allocator>) -> Option<arc::R<cf::String>> {
        unsafe { CFUUIDCreateString(alloc, self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFUUIDGetTypeID() -> cf::TypeId;
    fn CFUUIDCreate(alloc: Option<&cf::Allocator>) -> Option<arc::R<UUID>>;
    fn CFUUIDCreateFromString(
        alloc: Option<&cf::Allocator>,
        uuid_str: &cf::String,
    ) -> Option<arc::R<UUID>>;
    fn CFUUIDCreateString(alloc: Option<&cf::Allocator>, uuid: &UUID)
        -> Option<arc::R<cf::String>>;
}
