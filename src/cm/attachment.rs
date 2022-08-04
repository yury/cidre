use crate::{cf, define_cf_type};

define_cf_type!(Bearer(cf::Type));

#[repr(u32)]
pub enum Mode {
    ShouldNotPropagate = 0,
    ShouldPropagate = 1,
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
    /// Given a cv::Buffer, set_attachment is equivalent to CVBufferSetAttachment.
    pub fn set_attachment(&mut self, key: &cf::String, value: &cf::Type, attachment_mode: Mode) {
        unsafe { CMSetAttachment(self, key, value, attachment_mode) }
    }

    /// Returns a specific attachment of a cm::AttachmentBearer
    ///
    /// You can attach any cf object to a cm::AttachmentBearer to store
    /// additional information. get_attachment retrieves an attachment identified
    /// by a key.  Given a CVBufferRef, CMGetAttachment is equivalent to
    /// CVBufferGetAttachment.
    #[inline]
    pub fn get_attachment<'a>(&'a self, key: &cf::String, mode: *mut Mode) -> Option<&'a cf::Type> {
        unsafe { CMGetAttachment(self, key, mode) }
    }

    /// Removes a specific attachment of a CMAttachmentBearer
    ///
    /// Removes an attachment identified by a key. If found the attachment
    /// is removed and the retain count decremented.
    /// Given a CVBufferRef, CMRemoveAttachment is equivalent to
    /// CVBufferRemoveAttachment.
    #[inline]
    pub fn remove_attachment(&mut self, key: &cf::String) {
        unsafe { CMRemoveAttachment(self, key) }
    }

    /// Removes all attachments of a cm::AttachmentBearer
    #[inline]
    pub fn remove_all_attachments(&mut self) {
        unsafe { CMRemoveAllAttachments(self) }
    }

    #[inline]
    pub unsafe fn copy_dictionary_of_attachments(&self, allocator: Option<&cf::Allocator>, attachment_mode: Mode) -> Option<cf::Retained<cf::DictionaryOf<cf::String, cf::Type>>> {
        CMCopyDictionaryOfAttachments(allocator, self, attachment_mode)
    }

    #[inline]
    pub fn dictionary_of_attachments(&self, attachment_mode: Mode) -> Option<cf::Retained<cf::DictionaryOf<cf::String, cf::Type>>> {
        unsafe {
            self.copy_dictionary_of_attachments(None, attachment_mode)
        }
    }
}

extern "C" {
    fn CMSetAttachment(
        target: &Bearer,
        key: &cf::String,
        value: &cf::Type,
        attachment_mode: Mode,
    );

    fn CMGetAttachment<'a>(
        target: &'a Bearer,
        key: &cf::String,
        mode: *mut Mode,
    ) -> Option<&'a cf::Type>;

    fn CMRemoveAttachment(target: &Bearer, key: &cf::String);
    fn CMRemoveAllAttachments(target: &Bearer);

    fn CMCopyDictionaryOfAttachments(allocator: Option<&cf::Allocator>, target: &Bearer, attachment_mode: Mode) -> Option<cf::Retained<cf::DictionaryOf<cf::String, cf::Type>>>;
}
