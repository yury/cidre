use crate::{
    cf::{self, Retained, Type},
    define_cf_type,
};

#[repr(u32)]
pub enum AttachmentMode {
    ShouldNotPropagate = 0,
    ShouldPropagate = 1,
}

define_cf_type!(Buffer(Type));

impl Buffer {
    #[inline]
    pub fn set_attachment(
        &mut self,
        key: &cf::String,
        value: &cf::Type,
        attachment_mode: AttachmentMode,
    ) {
        unsafe { CVBufferSetAttachment(self, key, value, attachment_mode) }
    }

    #[inline]
    pub fn remove_attachment(&mut self, key: &cf::String) {
        unsafe { CVBufferRemoveAttachment(self, key) }
    }

    #[inline]
    pub fn remove_all_attachments(&mut self) {
        unsafe { CVBufferRemoveAllAttachments(self) }
    }

    #[inline]
    pub fn set_attachments(
        &mut self,
        the_attachments: &cf::Dictionary,
        attachment_mode: AttachmentMode,
    ) {
        unsafe { CVBufferSetAttachments(self, the_attachments, attachment_mode) }
    }

    #[inline]
    pub fn propagate_attachments(&self, destination_buffer: &mut Buffer) {
        unsafe { CVBufferPropagateAttachments(self, destination_buffer) }
    }

    #[inline]
    pub fn copy_attachments<'a>(
        &self,
        attachment_mode: AttachmentMode,
    ) -> Option<Retained<'a, cf::Dictionary>> {
        unsafe { CVBufferCopyAttachments(self, attachment_mode) }
    }

    #[inline]
    pub fn copy_attachment<'a>(
        &self,
        key: &cf::String,
        attachment_mode: AttachmentMode,
    ) -> Option<Retained<'a, Type>> {
        unsafe { CVBufferCopyAttachment(self, key, attachment_mode) }
    }

    #[inline]
    pub fn has_attachment(&self, key: &cf::String) -> bool {
        unsafe { CVBufferHasAttachment(self, key) }
    }
}

extern "C" {

    static kCVBufferPropagatedAttachmentsKey: &'static cf::String;
    static kCVBufferNonPropagatedAttachmentsKey: &'static cf::String;

    static kCVBufferMovieTimeKey: &'static cf::String;
    static kCVBufferTimeValueKey: &'static cf::String;
    static kCVBufferTimeScaleKey: &'static cf::String;

    fn CVBufferSetAttachment(
        buffer: &mut Buffer,
        key: &cf::String,
        value: &cf::Type,
        attachment_mode: AttachmentMode,
    );
    fn CVBufferRemoveAttachment(buffer: &mut Buffer, key: &cf::String);
    fn CVBufferRemoveAllAttachments(buffer: &mut Buffer);
    fn CVBufferSetAttachments(
        buffer: &mut Buffer,
        the_attachments: &cf::Dictionary,
        attachment_mode: AttachmentMode,
    );
    fn CVBufferPropagateAttachments(source_buffer: &Buffer, destination_buffer: &mut Buffer);
    fn CVBufferCopyAttachments<'a>(
        buffer: &Buffer,
        attachment_mode: AttachmentMode,
    ) -> Option<Retained<'a, cf::Dictionary>>;
    fn CVBufferCopyAttachment<'a>(
        buffer: &Buffer,
        key: &cf::String,
        attachment_mode: AttachmentMode,
    ) -> Option<Retained<'a, Type>>;
    fn CVBufferHasAttachment(buffer: &Buffer, key: &cf::String) -> bool;

}
