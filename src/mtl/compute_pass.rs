use std::ops::Index;

use crate::{define_obj_type, ns};

use super::{CounterSampleBuffer, DispatchType};

define_obj_type!(Descriptor(ns::Id));

impl Descriptor {
    /// ```
    /// use cidre::{mtl, objc};
    ///
    /// objc::autoreleasepool(|| {
    ///     let cpd = mtl::ComputePassDescriptor::default();
    ///
    ///     assert_eq!(cpd.dispatch_type(), mtl::DispatchType::Serial);
    /// })
    /// ```
    #[inline]
    pub fn default<'ar>() -> &'ar Descriptor {
        unsafe { MTLComputePassDescriptor_computePassDescriptor() }
    }

    #[inline]
    pub fn dispatch_type(&self) -> DispatchType {
        unsafe { rsel_dispatchType(self) }
    }

    #[inline]
    pub fn set_dispatch_type(&mut self, value: DispatchType) {
        unsafe { wsel_setDispatchType(self, value) }
    }

    #[inline]
    pub fn sample_buffer_attachments(&self) -> &SampleBufferAttachmentDescriptorArray {
        unsafe { MTLComputePassDescriptor_sampleBufferAttachments(self) }
    }

    #[inline]
    pub fn sample_buffer_attachments_mut(&mut self) -> &mut SampleBufferAttachmentDescriptorArray {
        unsafe { MTLComputePassDescriptor_sampleBufferAttachments(self) }
    }
}

extern "C" {
    fn MTLComputePassDescriptor_computePassDescriptor<'ar>() -> &'ar Descriptor;

    fn rsel_dispatchType(id: &ns::Id) -> DispatchType;
    fn wsel_setDispatchType(id: &mut ns::Id, value: DispatchType);

    fn MTLComputePassDescriptor_sampleBufferAttachments(
        id: &ns::Id,
    ) -> &mut SampleBufferAttachmentDescriptorArray;
}

define_obj_type!(SampleBufferAttachmentDescriptorArray(ns::Id));

impl SampleBufferAttachmentDescriptorArray {
    #[inline]
    pub fn get_at(&self, index: usize) -> &SampleBufferAttachmentDescriptor {
        unsafe {
            MTLComputePassSampleBufferAttachmentDescriptorArray_objectAtIndexedSubscript(
                self, index,
            )
        }
    }

    #[inline]
    pub fn set_at(&mut self, index: usize, value: &SampleBufferAttachmentDescriptor) {
        unsafe {
            MTLComputePassSampleBufferAttachmentDescriptorArray_setObjectAtIndexedSubscript(
                self,
                Some(value),
                index,
            )
        }
    }

    #[inline]
    pub fn reset_at(&mut self, index: usize) {
        unsafe {
            MTLComputePassSampleBufferAttachmentDescriptorArray_setObjectAtIndexedSubscript(
                self, None, index,
            )
        }
    }
}

impl Index<usize> for SampleBufferAttachmentDescriptorArray {
    type Output = SampleBufferAttachmentDescriptor;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get_at(index)
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLComputePassSampleBufferAttachmentDescriptorArray_objectAtIndexedSubscript(
        id: &ns::Id,
        index: usize,
    ) -> &SampleBufferAttachmentDescriptor;

    fn MTLComputePassSampleBufferAttachmentDescriptorArray_setObjectAtIndexedSubscript(
        id: &mut ns::Id,
        value: Option<&SampleBufferAttachmentDescriptor>,
        index: usize,
    );
}

define_obj_type!(SampleBufferAttachmentDescriptor(ns::Id));

impl SampleBufferAttachmentDescriptor {
    #[inline]
    pub fn sample_buffer(&self) -> Option<&CounterSampleBuffer> {
        unsafe { rsel_sampleBuffer(self) }
    }

    #[inline]
    pub fn set_sample_bufer(&mut self, value: Option<&CounterSampleBuffer>) {
        unsafe { wsel_setSampleBuffer(self, value) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_sampleBuffer(id: &ns::Id) -> Option<&CounterSampleBuffer>;
    fn wsel_setSampleBuffer(id: &mut ns::Id, value: Option<&CounterSampleBuffer>);
}
