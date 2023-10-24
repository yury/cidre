use crate::{
    arc, define_cls, define_obj_type, ns,
    objc::{self, Class},
};

use super::{CounterSampleBuf, DispatchType};

define_obj_type!(Desc(ns::Id));

impl arc::A<Desc> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<Desc>;
}

impl Desc {
    define_cls!(MTL_COMPUTE_PASS_DESCRIPTOR);

    #[inline]
    pub fn new() -> arc::R<Self> {
        Self::alloc().init()
    }

    #[objc::msg_send(dispatchType)]
    pub fn dispatch_type(&self) -> DispatchType;

    #[objc::msg_send(setDispatchType:)]
    pub fn set_dispatch_type(&mut self, value: DispatchType);

    #[objc::msg_send(sampleBufferAttachments)]
    pub fn sample_buffer_attachments(&self) -> &SampleBufAttachDescArray;

    #[objc::msg_send(sampleBufferAttachments)]
    pub fn sample_buffer_attachments_mut(&mut self) -> &mut SampleBufAttachDescArray;
}

extern "C" {
    static MTL_COMPUTE_PASS_DESCRIPTOR: &'static Class<Desc>;
}

define_obj_type!(SampleBufAttachDescArray(ns::Id));

impl SampleBufAttachDescArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get_at(&self, index: usize) -> Option<&SampleBufAttachDesc>;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set_object_at(&mut self, object: Option<&SampleBufAttachDesc>, index: usize);

    #[inline]
    pub fn set_at(&mut self, index: usize, value: &SampleBufAttachDesc) {
        self.set_object_at(Some(value), index)
    }

    #[inline]
    pub fn reset_at(&mut self, index: usize) {
        self.set_object_at(None, index)
    }
}

define_obj_type!(SampleBufAttachDesc(ns::Id));

impl SampleBufAttachDesc {
    #[objc::msg_send(sampleBuffer)]
    pub fn sample_buffer(&self) -> Option<&CounterSampleBuf>;

    #[objc::msg_send(setSampleBuffer:)]
    pub fn set_sample_bufer(&mut self, value: Option<&CounterSampleBuf>);
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
