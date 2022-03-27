use crate::{define_obj_type, objc::Id};

use super::DispatchType;

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
}

extern "C" {
    fn MTLComputePassDescriptor_computePassDescriptor<'autoreleased>() -> &'autoreleased Descriptor;

    fn rsel_dispatchType(id: &Id) -> DispatchType;
    fn wsel_setDispatchType(id: &mut Id, value: DispatchType);
    // rwsel(, id, dispatchType, setDispatchType, MTLDispatchType)
}

// define_obj_type!(AttachmentDescriptor(Id));
// define_obj_type!(ColorAttachmentDescriptor(AttachmentDescriptor));
// define_obj_type!(DepthAttachmentDescriptor(AttachmentDescriptor));
// define_obj_type!(StencilAttachmentDescriptor(AttachmentDescriptor));
