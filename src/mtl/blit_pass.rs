use crate::{define_obj_type, ns::Id};

define_obj_type!(SampleBufferAttachmentDescriptor(Id));

define_obj_type!(SampleBufferAttachmentDescriptorArray(Id));

define_obj_type!(Descriptor(Id));

impl Descriptor {
    /// # Examples
    ///
    /// ```
    /// use cidre::mtl;
    ///
    /// let bpd = mtl::BlitPassDescriptor::default();
    /// ```
    pub fn default<'ar>() -> &'ar Descriptor {
        unsafe { MTLBlitPassDescriptor_blitPassDescriptor() }
    }

    pub fn sample_buffer_attachments(&self) -> &SampleBufferAttachmentDescriptorArray {
        unsafe { MTLBlitPassDescriptor_sampleBufferAttachments(self) }
    }
}

extern "C" {
    fn MTLBlitPassDescriptor_blitPassDescriptor<'ar>() -> &'ar Descriptor;
    fn MTLBlitPassDescriptor_sampleBufferAttachments(
        id: &Descriptor,
    ) -> &SampleBufferAttachmentDescriptorArray;
}
