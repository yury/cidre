use crate::{api, arc, cf, cm, define_cf_type, os};

/// The os::Errors returned from the CMTaggedBufferGroup routines.
#[doc(alias = "CMTaggedBufferGroupError")]
pub mod err {
    use crate::os::Error;

    #[doc(alias = "kCMTaggedBufferGroupError_ParamErr")]
    pub const PARAM_ERR: Error = Error::new_unchecked(-15780);

    #[doc(alias = "kCMTaggedBufferGroupError_AllocationFailed")]
    pub const ALLOC_FAILED: Error = Error::new_unchecked(-15781);

    #[doc(alias = "kCMTaggedBufferGroupError_InternalError")]
    pub const INTERNAL: Error = Error::new_unchecked(-15782);
}

define_cf_type!(
    #[doc(alias = "CMTaggedBufferGroupRef")]
    TaggedBufGroup(cf::Type)
);

impl TaggedBufGroup {
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn get_type_id() -> cf::Type {
        unsafe { CMTaggedBufferGroupGetTypeID() }
    }

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    unsafe fn _create_in(
        tag_collections: &cf::ArrayOf<cm::TagCollection>,
        buffers: &cf::ArrayOf<cf::Type>,
        group_out: *mut Option<arc::R<TaggedBufGroup>>,
        allocator: Option<&cf::Allocator>,
    ) -> os::Status {
        unsafe { CMTaggedBufferGroupCreate(allocator, tag_collections, buffers, group_out) }
    }

    #[doc(alias = "CMTaggedBufferGroupCreate")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn create_in(
        tag_collections: &cf::ArrayOf<cm::TagCollection>,
        buffers: &cf::ArrayOf<cf::Type>,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<Option<arc::R<Self>>> {
        unsafe {
            let mut res = None;
            Self::_create_in(tag_collections, buffers, &mut res, allocator).to_result_option(res)
        }
    }

    #[doc(alias = "CMTaggedBufferGroupCreate")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn with(
        tag_collections: &cf::ArrayOf<cm::TagCollection>,
        buffers: &cf::ArrayOf<cf::Type>,
    ) -> os::Result<arc::R<Self>> {
        unsafe {
            let res = Self::create_in(tag_collections, buffers, None)?;
            Ok(res.unwrap_unchecked())
        }
    }

    #[doc(alias = "CMTaggedBufferGroupGetCount")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn count(&self) -> cm::ItemCount {
        unsafe { CMTaggedBufferGroupGetCount(self) }
    }

    #[doc(alias = "CMTaggedBufferGroupGetCount")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn len(&self) -> usize {
        self.count() as _
    }

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    #[inline]
    pub fn is_empty(&self) -> bool {
        #[allow(unused_unsafe)]
        unsafe {
            self.len() == 0
        }
    }
}

impl cm::SampleBuf {
    #[doc(alias = "CMSampleBufferGetTaggedBufferGroup")]
    #[doc(alias = "taggedBuffers")]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    pub fn tagged_bufs(&self) -> Option<&cm::TaggedBufGroup> {
        unsafe { CMSampleBufferGetTaggedBufferGroup(self) }
    }
}

#[link(name = "CoreMedia", kind = "framework")]
#[api::weak]
unsafe extern "C-unwind" {
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTaggedBufferGroupGetTypeID() -> cf::Type;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTaggedBufferGroupCreate(
        allocator: Option<&cf::Allocator>,
        tag_collections: &cf::ArrayOf<cm::TagCollection>,
        buffers: &cf::ArrayOf<cf::Type>,
        group_out: *mut Option<arc::R<TaggedBufGroup>>,
    ) -> os::Status;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMTaggedBufferGroupGetCount(group: &TaggedBufGroup) -> cm::ItemCount;

    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0, watchos = 10.0, visionos = 1.0)]
    fn CMSampleBufferGetTaggedBufferGroup(sbuf: &cm::SampleBuf) -> Option<&cm::TaggedBufGroup>;
}

#[cfg(test)]
mod tests {
    use crate::{cf, cm};

    #[test]
    fn basics() {
        let group = cm::TaggedBufGroup::with(&cf::ArrayOf::new(), &cf::ArrayOf::new()).unwrap();
        assert!(group.is_empty());
    }
}
