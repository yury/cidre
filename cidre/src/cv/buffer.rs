use crate::{api, arc, cf, define_cf_type};

#[doc(alias = "CVAttachmentMode")]
#[derive(Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum AttachMode {
    ShouldNotPropagate = 0,
    ShouldPropagate = 1,
}

define_cf_type!(
    #[doc(alias = "CVBuffer")]
    Buf(cf::Type)
);

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
    pub fn set_attach(&mut self, key: &cf::String, val: &cf::Type, attachment_mode: AttachMode) {
        unsafe { CVBufferSetAttachment(self, key, val, attachment_mode) }
    }

    #[inline]
    pub fn remove_attach(&mut self, key: &cf::String) {
        unsafe { CVBufferRemoveAttachment(self, key) }
    }

    #[inline]
    pub fn remove_all_attaches(&mut self) {
        unsafe { CVBufferRemoveAllAttachments(self) }
    }

    #[inline]
    pub fn set_attaches(&mut self, attachments: &cf::Dictionary, attachment_mode: AttachMode) {
        unsafe { CVBufferSetAttachments(self, attachments, attachment_mode) }
    }

    #[inline]
    pub fn propagate_attaches(&self, destination_buffer: &mut Buf) {
        unsafe { CVBufferPropagateAttachments(self, destination_buffer) }
    }

    #[inline]
    pub fn copy_attaches(&self, attachment_mode: AttachMode) -> Option<arc::R<cf::Dictionary>> {
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

    /// Useful with the CoreVideo pool and texture cache APIs so that you can specify
    /// an initial set of default buffer attachments to automatically be attached to the buffer when it is created.
    #[doc(alias = "kCVBufferPropagatedAttachmentsKey")]
    #[api::available(macos = 10.4, ios = 4.0, visionos = 1.0)]
    pub fn propagated_attaches_key() -> &'static cf::String {
        unsafe { kCVBufferPropagatedAttachmentsKey }
    }

    /// Useful with the CoreVideo pool and texture cache APIs so that you can specify
    /// an initial set of default buffer attachments to automatically be attached to the buffer when it is created.
    #[doc(alias = "kCVBufferNonPropagatedAttachmentsKey")]
    #[api::available(macos = 10.4, ios = 4.0, visionos = 1.0)]
    pub fn not_propagated_attaches_key() -> &'static cf::String {
        unsafe { kCVBufferNonPropagatedAttachmentsKey }
    }

    #[doc(alias = "kCVBufferMovieTimeKey")]
    #[api::available(macos = 10.4, ios = 4.0, visionos = 1.0)]
    pub fn movie_time_key() -> &'static cf::String {
        unsafe { kCVBufferMovieTimeKey }
    }

    #[doc(alias = "kCVBufferTimeValueKey")]
    #[api::available(macos = 10.4, ios = 4.0, visionos = 1.0)]
    pub fn time_value_key() -> &'static cf::String {
        unsafe { kCVBufferTimeValueKey }
    }

    #[doc(alias = "kCVBufferTimeScaleKey")]
    #[api::available(macos = 10.4, ios = 4.0, visionos = 1.0)]
    pub fn time_scale_key() -> &'static cf::String {
        unsafe { kCVBufferTimeScaleKey }
    }
}

#[link(name = "CoreVideo", kind = "framework")]
#[api::weak]
extern "C-unwind" {

    #[api::available(macos = 10.4, ios = 4.0, visionos = 1.0)]
    static kCVBufferPropagatedAttachmentsKey: &'static cf::String;

    #[api::available(macos = 10.4, ios = 4.0, visionos = 1.0)]
    static kCVBufferNonPropagatedAttachmentsKey: &'static cf::String;

    #[api::available(macos = 10.4, ios = 4.0, visionos = 1.0)]
    static kCVBufferMovieTimeKey: &'static cf::String;
    #[api::available(macos = 10.4, ios = 4.0, visionos = 1.0)]
    static kCVBufferTimeValueKey: &'static cf::String;
    #[api::available(macos = 10.4, ios = 4.0, visionos = 1.0)]
    static kCVBufferTimeScaleKey: &'static cf::String;

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
