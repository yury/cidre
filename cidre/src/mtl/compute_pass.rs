use crate::{arc, define_obj_type, mtl, ns, objc};

define_obj_type!(pub Desc(ns::Id), MTL_COMPUTE_PASS_DESCRIPTOR);

impl Desc {
    #[objc::msg_send(dispatchType)]
    pub fn dispatch_type(&self) -> mtl::DispatchType;

    #[objc::msg_send(setDispatchType:)]
    pub fn set_dispatch_type(&mut self, val: mtl::DispatchType);

    #[objc::msg_send(sampleBufferAttachments)]
    pub fn sample_buf_attaches(&self) -> &SampleBufAttachDescArray;

    #[objc::msg_send(sampleBufferAttachments)]
    pub fn sample_buf_attaches_mut(&mut self) -> &mut SampleBufAttachDescArray;
}

extern "C" {
    static MTL_COMPUTE_PASS_DESCRIPTOR: &'static objc::Class<Desc>;
}

define_obj_type!(pub SampleBufAttachDescArray(ns::Id));

impl SampleBufAttachDescArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get_at(&self, index: usize) -> Option<&SampleBufAttachDesc>;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set_object_at(&mut self, val: Option<&SampleBufAttachDesc>, index: usize);

    #[inline]
    pub fn set_at(&mut self, index: usize, val: &SampleBufAttachDesc) {
        self.set_object_at(Some(val), index)
    }

    #[inline]
    pub fn reset_at(&mut self, index: usize) {
        self.set_object_at(None, index)
    }
}

define_obj_type!(pub SampleBufAttachDesc(ns::Id));

impl SampleBufAttachDesc {
    #[objc::msg_send(sampleBuffer)]
    pub fn sample_buffer(&self) -> Option<&mtl::CounterSampleBuf>;

    #[objc::msg_send(setSampleBuffer:)]
    pub fn set_sample_bufer(&mut self, val: Option<&mtl::CounterSampleBuf>);
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
