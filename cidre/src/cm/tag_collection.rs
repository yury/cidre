use crate::{api, arc, cf, cm, define_cf_type, os};

/// Errors returned from the cm::TagCollection routines.
#[doc(alias = "CMTagCollectionError")]
pub mod err {
    use crate::os::Error;

    /// When caller passes incorrect input or output parameters.
    #[doc(alias = "kCMTagCollectionError_ParamErr")]
    pub const PARAM_ERR: Error = Error::new_unchecked(-15740);

    /// Returned if a necessary allocation failed.
    #[doc(alias = "kCMTagCollectionError_AllocationFailed")]
    pub const ALLOC_FAILED: Error = Error::new_unchecked(-15741);

    //// Returned if some kind of internal implementation error occurred.
    #[doc(alias = "kCMTagCollectionError_InternalError")]
    pub const INTERNAL: Error = Error::new_unchecked(-15742);

    /// Returned if the tag is kCMTagInvalid.
    #[doc(alias = "kCMTagCollectionError_InvalidTag")]
    pub const INVALID_TAG: Error = Error::new_unchecked(-15743);

    /// Returned if the CFDictionary being deserialized is not valid to create a cm::TagCollection.
    #[doc(alias = "kCMTagCollectionError_InvalidTagCollectionDictionary")]
    pub const INVALID_TAG_COLLECTION_DICTIONARY: Error = Error::new_unchecked(-15744);

    /// Returned if structure of the CFData being deserialized is not valid to create a cm::TagCollection.
    #[doc(alias = "kCMTagCollectionError_InvalidTagCollectionData")]
    pub const INVALID_TAG_COLLECTION_DATA: Error = Error::new_unchecked(-15745);

    /// Returned if a search for a cm::Tag in the collection failed, including if the collection is empty.
    #[doc(alias = "kCMTagCollectionError_TagNotFound")]
    pub const TAG_NOT_FOUND: Error = Error::new_unchecked(-15746);

    /// Returned if the CFData deserialized to create a cm::TagCollection has an unknown version.
    #[doc(alias = "kCMTagCollectionError_InvalidTagCollectionDataVersion")]
    pub const INVALID_TAG_COLLECTION_DATA_VERSION: Error = Error::new_unchecked(-15747);

    /// Returned if the buffer size to retrieve cm::Tags is smaller than necessary.
    #[doc(alias = "kCMTagCollectionError_ExhaustedBufferSize")]
    pub const EXHAUSTED_BUF_SIZE: Error = Error::new_unchecked(-15748);

    /// Returned if the function is not yet implemented.
    #[doc(alias = "kCMTagCollectionError_NotYetImplemented")]
    pub const NOT_YET_IMPLEMENTED: Error = Error::new_unchecked(-15749);
}

define_cf_type!(
    #[doc(alias = "CMTagCollectionRef")]
    TagCollection(cf::Type)
);

define_cf_type!(
    #[doc(alias = "CMMutableTagCollectionRef")]
    TagCollectionMut(TagCollection)
);

impl TagCollection {
    #[doc(alias = "CMTagCollectionGetTypeID")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CMTagCollectionGetTypeID() }
    }

    #[doc(alias = "CMTagCollectionCreate")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub unsafe fn create_in(
        tags: *const cm::Tag,
        tag_count: cm::ItemCount,
        out: *mut Option<arc::R<Self>>,
        allocator: Option<&cf::Allocator>,
    ) -> os::Status {
        unsafe { CMTagCollectionCreate(allocator, tags, tag_count, out) }
    }

    #[doc(alias = "CMTagCollectionCreate")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn with_tags(tags: &[cm::Tag]) -> Result<arc::R<Self>, os::Error> {
        unsafe {
            let mut res = None;
            Self::create_in(tags.as_ptr(), tags.len() as _, &mut res, None).result()?;
            Ok(res.unwrap_unchecked())
        }
    }

    #[doc(alias = "CMTagCollectionGetCount")]
    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn count(&self) -> cm::ItemCount {
        unsafe { CMTagCollectionGetCount(self) }
    }

    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn len(&self) -> usize {
        self.count() as _
    }

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn is_empty(&self) -> bool {
        // for versions below available self.len() is unsafe
        #[allow(unused_unsafe)]
        unsafe {
            self.len() == 0
        }
    }

    #[doc(alias = "CMTagCollectionContainsTag")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn contains(&self, val: cm::Tag) -> bool {
        unsafe { CMTagCollectionContainsTag(self, val) }
    }

    #[doc(alias = "CMTagCollectionContainsTagsOfCollection")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn contains_collection(&self, val: &cm::TagCollection) -> bool {
        unsafe { CMTagCollectionContainsTagsOfCollection(self, val) }
    }

    #[doc(alias = "CMTagCollectionContainsSpecifiedTags")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn contains_tags(&self, tags: &[cm::Tag]) -> bool {
        unsafe { CMTagCollectionContainsSpecifiedTags(self, tags.as_ptr(), tags.len() as _) }
    }

    #[doc(alias = "CMTagCollectionContainsCategory")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn contains_catogery(&self, val: cm::TagCategory) -> bool {
        unsafe { CMTagCollectionContainsCategory(self, val) }
    }

    // TODO: more api
}

#[link(name = "CoreMedia", kind = "framework")]
#[api::weak]
unsafe extern "C-unwind" {

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTagCollectionGetTypeID() -> cf::TypeId;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTagCollectionCreate(
        allocator: Option<&cf::Allocator>,
        tags: *const cm::Tag,
        tag_count: cm::ItemCount,
        collection_out: *mut Option<arc::R<cm::TagCollection>>,
    ) -> os::Status;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTagCollectionGetCount(tag_collection: &TagCollection) -> cm::ItemCount;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTagCollectionContainsTag(tag_collection: &TagCollection, val: cm::Tag) -> bool;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTagCollectionContainsTagsOfCollection(
        tag_collection: &TagCollection,
        contained_tag_collection: &TagCollection,
    ) -> bool;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTagCollectionContainsSpecifiedTags(
        tag_collection: &TagCollection,
        contained_tags: *const cm::Tag,
        contained_tag_count: cm::ItemCount,
    ) -> bool;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTagCollectionContainsCategory(
        tag_collection: &TagCollection,
        category: cm::TagCategory,
    ) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::cm;

    #[test]
    fn basics() {
        let empty = cm::TagCollection::with_tags(&[]).unwrap();
        assert!(empty.is_empty());

        let tag = cm::Tag::with_f64(cm::TagCategory::MediaType, 0.0);
        assert!(!empty.contains(tag));
        let empty2 = cm::TagCollection::with_tags(&[]).unwrap();

        assert!(empty.contains_collection(&empty2));

        assert!(empty.contains_tags(&[]));
        assert!(!empty.contains_tags(&[tag]));
    }
}
