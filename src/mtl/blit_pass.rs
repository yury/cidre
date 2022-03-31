use crate::{cf::runtime::Autoreleased, define_obj_type, ns::Id};

define_obj_type!(SampleBufferAttachmentDescriptor(Id));

define_obj_type!(SampleBufferAttachmentDescriptorArray(Id));

define_obj_type!(Descriptor(Id));

impl Descriptor {
    /// ```
    /// use cidre::mtl;
    ///
    /// let bpd = mtl::BlitPassDescriptor::default();
    /// ```
    pub fn default<'a>() -> Autoreleased<'a, Descriptor> {
        unsafe { MTLBlitPassDescriptor_blitPassDescriptor() }
    }

    pub fn sample_buffer_attachments(&self) -> &SampleBufferAttachmentDescriptorArray {
        unsafe { MTLBlitPassDescriptor_sampleBufferAttachments(self) }
    }
}

extern "C" {
    fn MTLBlitPassDescriptor_blitPassDescriptor<'a>() -> Autoreleased<'a, Descriptor>;
    fn MTLBlitPassDescriptor_sampleBufferAttachments(
        id: &Descriptor,
    ) -> &SampleBufferAttachmentDescriptorArray;
}
