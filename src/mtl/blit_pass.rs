use crate::{define_obj_type, ns::Id};

define_obj_type!(SampleBufferAttachmentDescriptor(Id));

define_obj_type!(SampleBufferAttachmentDescriptorArray(Id));

define_obj_type!(Descriptor(Id));

/// Represents a collection of attachments to be used to create a concrete blit command encoder
impl Descriptor {
    /// Create an autoreleased default frame buffer descriptor
    /// # Examples
    ///
    /// ```
    /// use cidre::mtl;
    ///
    /// let mut bpd = mtl::BlitPassDescriptor::default();
    /// let attachments = bpd.sample_buffer_attachments_mut();
    /// ```
    pub fn default<'ar>() -> &'ar mut Descriptor {
        unsafe { MTLBlitPassDescriptor_blitPassDescriptor() }
    }

    /// An array of sample buffers and associated sample indices.
    pub fn sample_buffer_attachments(&self) -> &SampleBufferAttachmentDescriptorArray {
        unsafe { MTLBlitPassDescriptor_rsel_sampleBufferAttachments(self) }
    }

    /// An array of sample buffers and associated sample indices.
    pub fn sample_buffer_attachments_mut(&mut self) -> &mut SampleBufferAttachmentDescriptorArray {
        unsafe { MTLBlitPassDescriptor_rsel_sampleBufferAttachments(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLBlitPassDescriptor_blitPassDescriptor<'ar>() -> &'ar mut Descriptor;
    fn MTLBlitPassDescriptor_rsel_sampleBufferAttachments(
        id: &Descriptor,
    ) -> &mut SampleBufferAttachmentDescriptorArray;
}
