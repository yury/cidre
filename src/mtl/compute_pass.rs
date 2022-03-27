use std::ops::Index;

use crate::{define_obj_type, objc::Id};

use super::{CounterSampleBuffer, DispatchType};

define_obj_type!(Descriptor(Id));

impl Descriptor {
    /// ```
    /// use cidre::{mtl};
    ///
    /// let cpd = mtl::ComputePassDescriptor::default();
    ///
    /// assert_eq!(cpd.dispatch_type(), mtl::DispatchType::Serial);
    /// ```
    pub fn default<'autoreleased>() -> &'autoreleased Descriptor {
        unsafe { MTLComputePassDescriptor_computePassDescriptor() }
    }

    pub fn dispatch_type(&self) -> DispatchType {
        unsafe { rsel_dispatchType(self) }
    }

    pub fn set_dispatch_type(&mut self, value: DispatchType) {
        unsafe { wsel_setDispatchType(self, value) }
    }

    pub fn sample_buffer_attachments(&self) -> &SampleBufferAttachmentDescriptorArray {
        unsafe { MTLComputePassDescriptor_sampleBufferAttachments(self) }
    }
}

extern "C" {
    fn MTLComputePassDescriptor_computePassDescriptor<'autoreleased>() -> &'autoreleased Descriptor;

    fn rsel_dispatchType(id: &Id) -> DispatchType;
    fn wsel_setDispatchType(id: &mut Id, value: DispatchType);

    // rsel(, MTLComputePassDescriptor *, sampleBufferAttachments, MTLComputePassSampleBufferAttachmentDescriptorArray *)
    fn MTLComputePassDescriptor_sampleBufferAttachments(
        id: &Id,
    ) -> &SampleBufferAttachmentDescriptorArray;
}

define_obj_type!(SampleBufferAttachmentDescriptorArray(Id));

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
        id: &Id,
        index: usize,
    ) -> &SampleBufferAttachmentDescriptor;

    fn MTLComputePassSampleBufferAttachmentDescriptorArray_setObjectAtIndexedSubscript(
        id: &mut Id,
        value: Option<&SampleBufferAttachmentDescriptor>,
        index: usize,
    );
}

define_obj_type!(SampleBufferAttachmentDescriptor(Id));

impl SampleBufferAttachmentDescriptor {
    pub fn sample_buffer(&self) -> Option<&CounterSampleBuffer> {
        unsafe { rsel_sampleBuffer(self) }
    }

    pub fn set_sample_bufer(&mut self, value: Option<&CounterSampleBuffer>) {
        unsafe { wsel_setSampleBuffer(self, value) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_sampleBuffer(id: &Id) -> Option<&CounterSampleBuffer>;
    fn wsel_setSampleBuffer(id: &mut Id, value: Option<&CounterSampleBuffer>);
}
