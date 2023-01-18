use std::ops::Index;

use crate::{
    arc, define_obj_type,
    mtl::msg_send,
    ns,
    objc::{self, Class, Obj},
};

use super::{CounterSampleBuffer, DispatchType};

define_obj_type!(Descriptor(ns::Id));

impl Descriptor {
    #[inline]
    pub fn default() -> arc::R<Descriptor> {
        unsafe { MTL_COMPUTE_PASS_DESCRIPTOR.alloc_init() }
    }

    #[inline]
    pub fn dispatch_type(&self) -> DispatchType {
        unsafe { self.call0(msg_send::dispatch_type) }
    }

    #[inline]
    pub fn set_dispatch_type(&mut self, value: DispatchType) {
        unsafe { self.call1(msg_send::set_dispatch_type, value) }
    }

    #[inline]
    pub fn sample_buffer_attachments(&self) -> &SampleBufferAttachmentDescriptorArray {
        unsafe { self.call0(msg_send::sample_buffer_attachments) }
    }

    #[inline]
    pub fn sample_buffer_attachments_mut(&mut self) -> &mut SampleBufferAttachmentDescriptorArray {
        unsafe { self.call0(msg_send::sample_buffer_attachments) }
    }
}

extern "C" {
    static MTL_COMPUTE_PASS_DESCRIPTOR: &'static Class<Descriptor>;
}

define_obj_type!(SampleBufferAttachmentDescriptorArray(ns::Id));

impl SampleBufferAttachmentDescriptorArray {
    #[inline]
    pub fn get_at(&self, index: usize) -> &SampleBufferAttachmentDescriptor {
        unsafe { self.call1(objc::msg_send::object_at_indexed_subscript, index) }
    }

    #[inline]
    pub fn set_at(&mut self, index: usize, value: &SampleBufferAttachmentDescriptor) {
        unsafe {
            self.call2(
                objc::msg_send::set_object_at_indexed_subscript,
                value,
                index,
            )
        }
    }

    #[inline]
    pub fn reset_at(&mut self, index: usize) {
        let none: Option<&SampleBufferAttachmentDescriptor> = None;
        unsafe { self.call2(objc::msg_send::set_object_at_indexed_subscript, none, index) }
    }
}

impl Index<usize> for SampleBufferAttachmentDescriptorArray {
    type Output = SampleBufferAttachmentDescriptor;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get_at(index)
    }
}

define_obj_type!(SampleBufferAttachmentDescriptor(ns::Id));

impl SampleBufferAttachmentDescriptor {
    #[inline]
    pub fn sample_buffer(&self) -> Option<&CounterSampleBuffer> {
        unsafe { self.call0(msg_send::sample_buffer) }
    }

    #[inline]
    pub fn set_sample_bufer(&mut self, value: Option<&CounterSampleBuffer>) {
        unsafe { self.call1(msg_send::set_sample_buffer, value) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{mtl, objc};
    #[test]
    fn basics() {
        objc::autoreleasepool(|| {
            let cpd = mtl::ComputePassDescriptor::default();

            assert_eq!(cpd.dispatch_type(), mtl::DispatchType::Serial);
        })
    }
}
