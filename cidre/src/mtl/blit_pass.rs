use crate::{arc, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTLBlitPassSampleBufferAttachmentDescriptor")]
    pub SampleBufAttachDesc(ns::Id)
);

impl SampleBufAttachDesc {
    #[objc::msg_send(sampleBuffer)]
    pub fn sample_buf(&self) -> Option<arc::R<mtl::CounterSampleBuf>>;

    #[objc::msg_send(setSampleBuffer:)]
    pub fn set_sample_buf(&mut self, val: Option<&mtl::CounterSampleBuf>);

    #[objc::msg_send(startOfEncoderSampleIndex)]
    pub fn start_of_encoder_sample_index(&self) -> usize;

    #[objc::msg_send(setStartOfEncoderSampleIndex:)]
    pub fn set_start_of_encoder_sample_index(&mut self, value: usize);

    #[objc::msg_send(endOfEncoderSampleIndex)]
    pub fn end_of_encoder_sample_index(&self) -> usize;

    #[objc::msg_send(setEndOfEncoderSampleIndex:)]
    pub fn set_end_of_encoder_sample_index(&mut self, value: usize);
}

define_obj_type!(
    #[doc(alias = "MTLBlitPassSampleBufferAttachmentDescriptorArray")]
    pub SampleBufAttachDescArray(ns::Id)
);

impl SampleBufAttachDescArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get(&self, attachment_index: usize) -> arc::R<SampleBufAttachDesc>;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set(&mut self, val: Option<&SampleBufAttachDesc>, attachment_index: usize);
}

define_obj_type!(
    #[doc(alias = "MTLBlitPassDescriptor")]
    pub Desc(ns::Id),
    MTL_BLIT_PASS_DESCRIPTOR
);

/// Represents a collection of attachments to be used to create a concrete blit command encoder
impl Desc {
    /// An array of sample buffers and associated sample indices.
    #[objc::msg_send(sampleBufferAttachments)]
    pub fn sample_buf_attaches(&self) -> arc::R<SampleBufAttachDescArray>;
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL_BLIT_PASS_DESCRIPTOR: &'static objc::Class<Desc>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let bpd = mtl::BlitPassDesc::new();
        let attaches = bpd.sample_buf_attaches();
        let mut attach = attaches.get(0);
        attach.set_start_of_encoder_sample_index(0);
        assert!(attach.sample_buf().is_none());
    }
}
