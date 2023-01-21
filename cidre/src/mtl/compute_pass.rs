use crate::{
    arc, define_cls, define_obj_type, ns,
    objc::{self, Class},
};

use super::{CounterSampleBuffer, DispatchType};

define_obj_type!(Descriptor(ns::Id));

impl arc::A<Descriptor> {
    #[objc::msg_send(init)]
    pub fn init(self) -> arc::R<Descriptor>;
}

impl Descriptor {
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
    pub fn sample_buffer_attachments(&self) -> &SampleBufferAttachmentDescriptorArray;

    #[objc::msg_send(sampleBufferAttachments)]
    pub fn sample_buffer_attachments_mut(&mut self) -> &mut SampleBufferAttachmentDescriptorArray;
}

extern "C" {
    static MTL_COMPUTE_PASS_DESCRIPTOR: &'static Class<Descriptor>;
}

define_obj_type!(SampleBufferAttachmentDescriptorArray(ns::Id));

impl SampleBufferAttachmentDescriptorArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get_at(&self, index: usize) -> Option<&SampleBufferAttachmentDescriptor>;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set_at(&mut self, index: usize, value: &SampleBufferAttachmentDescriptor);

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set_option_at(&mut self, index: usize, value: Option<&SampleBufferAttachmentDescriptor>);

    #[inline]
    pub fn reset_at(&mut self, index: usize) {
        self.set_option_at(index, None)
    }
}

define_obj_type!(SampleBufferAttachmentDescriptor(ns::Id));

impl SampleBufferAttachmentDescriptor {
    #[objc::msg_send(sampleBuffer)]
    pub fn sample_buffer(&self) -> Option<&CounterSampleBuffer>;

    #[objc::msg_send(setSampleBuffer:)]
    pub fn set_sample_bufer(&mut self, value: Option<&CounterSampleBuffer>);
}

#[cfg(test)]
mod tests {
    use crate::{mtl, objc};
    #[test]
    fn basics() {
        objc::autoreleasepool(|| {
            let cpd = mtl::ComputePassDescriptor::new();

            assert_eq!(cpd.dispatch_type(), mtl::DispatchType::Serial);
        })
    }
}
