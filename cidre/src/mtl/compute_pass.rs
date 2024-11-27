use crate::{arc, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTLComputePassDescriptor")]
    pub Desc(ns::Id),
    MTL_COMPUTE_PASS_DESCRIPTOR
);

impl Desc {
    #[objc::msg_send(dispatchType)]
    pub fn dispatch_type(&self) -> mtl::DispatchType;

    #[objc::msg_send(setDispatchType:)]
    pub fn set_dispatch_type(&mut self, val: mtl::DispatchType);

    #[objc::msg_send(sampleBufferAttachments)]
    pub fn sample_buf_attaches(&self) -> arc::R<SampleBufAttachDescArray>;
}

extern "C" {
    static MTL_COMPUTE_PASS_DESCRIPTOR: &'static objc::Class<Desc>;
}

define_obj_type!(
    #[doc(alias = "MTLComputePassSampleBufferAttachmentDescriptorArray")]
    pub SampleBufAttachDescArray(ns::Id)
);

impl SampleBufAttachDescArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get(&self, index: usize) -> arc::R<SampleBufAttachDesc>;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set(&mut self, val: Option<&SampleBufAttachDesc>, index: usize);
}

define_obj_type!(
    #[doc(alias = "MTLComputePassSampleBufferAttachmentDescriptor")]
    pub SampleBufAttachDesc(ns::Id)
);

impl SampleBufAttachDesc {
    #[objc::msg_send(sampleBuffer)]
    pub fn sample_buffer(&self) -> Option<arc::R<mtl::CounterSampleBuf>>;

    #[objc::msg_send(setSampleBuffer:)]
    pub fn set_sample_bufer(&mut self, val: Option<&mtl::CounterSampleBuf>);

    #[objc::msg_send(startOfEncoderSampleIndex)]
    pub fn start_of_encoder_sample_index(&self) -> usize;

    #[objc::msg_send(setStartOfEncoderSampleIndex:)]
    pub fn set_start_of_encoder_sample_index(&mut self, val: usize);

    #[objc::msg_send(endOfEncoderSampleIndex)]
    pub fn end_of_encoder_sample_index(&self) -> usize;

    #[objc::msg_send(endOfEncoderSampleIndex:)]
    pub fn set_end_of_encoder_sample_index(&mut self, val: usize);
}

#[cfg(test)]
mod tests {
    use crate::{mtl, objc};
    #[test]
    fn basics() {
        objc::ar_pool(|| {
            let cpd = mtl::ComputePassDesc::new();

            assert_eq!(cpd.dispatch_type(), mtl::DispatchType::Serial);
        })
    }
}
