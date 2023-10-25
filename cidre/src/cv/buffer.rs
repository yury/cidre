use crate::{arc, cf, define_cf_type};

#[doc(alias = "CVAttachmentMode")]
#[derive(Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum AttachMode {
    ShouldNotPropagate = 0,
    ShouldPropagate = 1,
}

define_cf_type!(Buf(cf::Type));

impl Buf {
    #[inline]
    pub fn attach<'a>(
        &'a self,
        key: &cf::String,
        attachment_mode: AttachMode,
    ) -> Option<&'a cf::Type> {
        unsafe { CVBufferGetAttachment(self, key, attachment_mode) }
    }

    #[inline]
    pub fn set_attach(&mut self, key: &cf::String, value: &cf::Type, attachment_mode: AttachMode) {
        unsafe { CVBufferSetAttachment(self, key, value, attachment_mode) }
    }

    #[inline]
    pub fn remove_attach(&mut self, key: &cf::String) {
        unsafe { CVBufferRemoveAttachment(self, key) }
    }

    #[inline]
    pub fn remove_all_attachs(&mut self) {
        unsafe { CVBufferRemoveAllAttachments(self) }
    }

    #[inline]
    pub fn set_attachs(&mut self, the_attachments: &cf::Dictionary, attachment_mode: AttachMode) {
        unsafe { CVBufferSetAttachments(self, the_attachments, attachment_mode) }
    }

    #[inline]
    pub fn propagate_attachs(&self, destination_buffer: &mut Buf) {
        unsafe { CVBufferPropagateAttachments(self, destination_buffer) }
    }

    #[inline]
    pub fn copy_attachs(&self, attachment_mode: AttachMode) -> Option<arc::R<cf::Dictionary>> {
        unsafe { CVBufferCopyAttachments(self, attachment_mode) }
    }

    #[inline]
    pub fn copy_attach(
        &self,
        key: &cf::String,
        attachment_mode: AttachMode,
    ) -> Option<arc::R<cf::Type>> {
        unsafe { CVBufferCopyAttachment(self, key, attachment_mode) }
    }

    #[inline]
    pub fn has_attach(&self, key: &cf::String) -> bool {
        unsafe { CVBufferHasAttachment(self, key) }
    }
}

extern "C" {

    // static kCVBufferPropagatedAttachmentsKey: &'static cf::String;
    // static kCVBufferNonPropagatedAttachmentsKey: &'static cf::String;

    // static kCVBufferMovieTimeKey: &'static cf::String;
    // static kCVBufferTimeValueKey: &'static cf::String;
    // static kCVBufferTimeScaleKey: &'static cf::String;

    fn CVBufferSetAttachment(
        buffer: &mut Buf,
        key: &cf::String,
        value: &cf::Type,
        attachment_mode: AttachMode,
    );
    fn CVBufferRemoveAttachment(buffer: &mut Buf, key: &cf::String);
    fn CVBufferRemoveAllAttachments(buffer: &mut Buf);
    fn CVBufferSetAttachments(
        buffer: &mut Buf,
        the_attachments: &cf::Dictionary,
        attachment_mode: AttachMode,
    );
    fn CVBufferPropagateAttachments(source_buffer: &Buf, destination_buffer: &mut Buf);
    fn CVBufferCopyAttachments(
        buffer: &Buf,
        attachment_mode: AttachMode,
    ) -> Option<arc::R<cf::Dictionary>>;
    fn CVBufferCopyAttachment(
        buffer: &Buf,
        key: &cf::String,
        attachment_mode: AttachMode,
    ) -> Option<arc::R<cf::Type>>;
    fn CVBufferHasAttachment(buffer: &Buf, key: &cf::String) -> bool;
    fn CVBufferGetAttachment<'a>(
        buffer: &'a Buf,
        key: &cf::String,
        attachment_mode: AttachMode,
    ) -> Option<&'a cf::Type>;

}
