use crate::{arc, cf, define_cf_type};

define_cf_type!(Bearer(cf::Type));

#[derive(Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Mode {
    NotPropagate = 0,
    Propagate = 1,
}

impl Bearer {
    ///  Sets or adds a attachment of a cm::AttachmentBearer
    ///
    /// You can attach any CF object to a cm::AttachmentBearer object to store
    /// additional information. set_attachment stores an attachment identified
    /// by a key. If the key doesn't exist, the attachment will be added.
    /// If the key does exist, the existing attachment will be replaced.
    /// In both cases the retain count of the attachment will be incremented.
    /// The value can be any cf::Type but nil has no defined behavior.
    /// Given a cv::Buf, 'set_attach' is equivalent to CVBufferSetAttachment.
    #[doc(alias = "CMSetAttachment")]
    pub fn set_attach(&mut self, key: &cf::String, val: &cf::Type, mode: Mode) {
        unsafe { CMSetAttachment(self, key, val, mode) }
    }

    /// Returns a specific attachment of a cm::AttachmentBearer
    ///
    /// You can attach any cf object to a cm::AttachmentBearer to store
    /// additional information. get_attachment retrieves an attachment identified
    /// by a key. Given a cv::Buf, CMGetAttachment is equivalent to
    /// CVBufferGetAttachment.
    #[doc(alias = "CMGetAttachment")]
    #[inline]
    pub fn attach<'a>(&'a self, key: &cf::String, mode: *mut Mode) -> Option<&'a cf::Type> {
        unsafe { CMGetAttachment(self, key, mode) }
    }

    /// Removes a specific attachment of a CMAttachmentBearer
    ///
    /// Removes an attachment identified by a key. If found the attachment
    /// is removed and the retain count decremented.
    /// Given a cv::Buff, CMRemoveAttachment is equivalent to
    /// CVBufferRemoveAttachment.
    #[doc(alias = "CMRemoveAttachment")]
    #[inline]
    pub fn remove_attach(&mut self, key: &cf::String) {
        unsafe { CMRemoveAttachment(self, key) }
    }

    /// Removes all attachments of a cm::AttachBearer
    #[doc(alias = "CMRemoveAllAttachments")]
    #[inline]
    pub fn remove_all_attaches(&mut self) {
        unsafe { CMRemoveAllAttachments(self) }
    }

    #[doc(alias = "CMCopyDictionaryOfAttachments")]
    #[inline]
    pub unsafe fn copy_dictionary_of_attaches_in(
        &self,
        mode: Mode,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>> {
        CMCopyDictionaryOfAttachments(allocator, self, mode)
    }

    #[doc(alias = "CMCopyDictionaryOfAttachments")]
    #[inline]
    pub fn dictionary_of_attaches(
        &self,
        mode: Mode,
    ) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>> {
        unsafe { self.copy_dictionary_of_attaches_in(mode, None) }
    }
}

extern "C-unwind" {
    fn CMSetAttachment(target: &Bearer, key: &cf::String, value: &cf::Type, attachment_mode: Mode);

    fn CMGetAttachment<'a>(
        target: &'a Bearer,
        key: &cf::String,
        mode: *mut Mode,
    ) -> Option<&'a cf::Type>;

    fn CMRemoveAttachment(target: &Bearer, key: &cf::String);
    fn CMRemoveAllAttachments(target: &Bearer);

    fn CMCopyDictionaryOfAttachments(
        allocator: Option<&cf::Allocator>,
        target: &Bearer,
        attachment_mode: Mode,
    ) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>>;
}
