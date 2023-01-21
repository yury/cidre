use crate::{arc, define_cls_init, define_obj_type, ns, objc};

define_obj_type!(SampleBufferAttachmentDescriptor(ns::Id));
define_obj_type!(SampleBufferAttachmentDescriptorArray(ns::Id));

define_obj_type!(Descriptor(ns::Id));

define_cls_init!(Descriptor, MTL_BLIT_PASS_DESCRIPTOR);

/// Represents a collection of attachments to be used to create a concrete blit command encoder
impl Descriptor {
    /// An array of sample buffers and associated sample indices.
    #[objc::msg_send(sampleBufferAttachments)]
    pub fn sample_buffer_attachments(&self) -> &SampleBufferAttachmentDescriptorArray;

    /// An array of sample buffers and associated sample indices.
    #[objc::msg_send(sampleBufferAttachments)]
    pub fn sample_buffer_attachments_mut(&mut self) -> &mut SampleBufferAttachmentDescriptorArray;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_BLIT_PASS_DESCRIPTOR: &'static objc::Class<Descriptor>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let mut bpd = mtl::BlitPassDescriptor::new();
        let _attachments = bpd.sample_buffer_attachments_mut();
    }
}
