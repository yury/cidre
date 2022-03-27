use std::ops::{Index, IndexMut};

use crate::cf::Retained;
use crate::define_obj_type;

use crate::ns::Id;

define_obj_type!(Descriptor(Id));
define_obj_type!(ColorAttachmentDescriptor(Id));
define_obj_type!(DepthAttachmentDescriptor(Id));
define_obj_type!(StencilAttachmentDescriptor(Id));

impl Descriptor {
    #[inline]
    pub fn new<'new>() -> Retained<'new, Descriptor> {
        unsafe { MTLRenderPassDescriptor_new() }
    }

    #[inline]
    pub fn default<'autoreleased>() -> &'autoreleased Descriptor {
        unsafe { MTLRenderPassDescriptor_renderPassDescriptor() }
    }

    #[inline]
    pub fn color_attachments(&self) -> &ColorAttachmentDescriptorArray {
        unsafe { rsel_colorAttachments(self) }
    }

    #[inline]
    pub fn depth_attachment(&self) -> &DepthAttachmentDescriptor {
        unsafe { rsel_depthAttachment(self) }
    }

    #[inline]
    pub fn set_depth_attachemnt(&mut self, value: &DepthAttachmentDescriptor) {
        unsafe { wsel_setDepthAttachment(self, Some(value)) }
    }

    #[inline]
    pub fn reset_depth_attachemnt(&mut self) {
        unsafe { wsel_setDepthAttachment(self, None) }
    }

    #[inline]
    pub fn stencil_attachment(&self) -> &StencilAttachmentDescriptor {
        unsafe { rsel_stencilAttachment(self) }
    }

    #[inline]
    pub fn set_stencil_attachemnt(&mut self, value: &StencilAttachmentDescriptor) {
        unsafe { wsel_setStencilAttachment(self, Some(value)) }
    }

    #[inline]
    pub fn reset_stencil_attachemnt(&mut self) {
        unsafe { wsel_setStencilAttachment(self, None) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLRenderPassDescriptor_new<'new>() -> Retained<'new, Descriptor>;
    fn MTLRenderPassDescriptor_renderPassDescriptor<'autoreleased>() -> &'autoreleased Descriptor;

    fn rsel_colorAttachments(id: &Id) -> &ColorAttachmentDescriptorArray;

    fn rsel_depthAttachment(id: &Id) -> &DepthAttachmentDescriptor;
    fn wsel_setDepthAttachment(id: &mut Id, value: Option<&DepthAttachmentDescriptor>);

    fn rsel_stencilAttachment(id: &Id) -> &StencilAttachmentDescriptor;
    fn wsel_setStencilAttachment(id: &mut Id, value: Option<&StencilAttachmentDescriptor>);
}

define_obj_type!(ColorAttachmentDescriptorArray(Id));

impl ColorAttachmentDescriptorArray {
    pub fn set_at(&mut self, index: usize, value: &ColorAttachmentDescriptor) {
        unsafe {
            MTLRenderPassColorAttachmentDescriptorArray_setObjectAtIndexedSubscript(
                self,
                Some(value),
                index,
            )
        }
    }

    pub fn reset_at(&mut self, index: usize) {
        unsafe {
            MTLRenderPassColorAttachmentDescriptorArray_setObjectAtIndexedSubscript(
                self, None, index,
            )
        }
    }
}

impl Index<usize> for ColorAttachmentDescriptorArray {
    type Output = ColorAttachmentDescriptor;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe { MTLRenderPassColorAttachmentDescriptorArray_objectAtIndexedSubscript(self, index) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn MTLRenderPassColorAttachmentDescriptorArray_objectAtIndexedSubscript(
        id: &Id,
        index: usize,
    ) -> &ColorAttachmentDescriptor;
    // wsel_ab(, MTLRenderPassColorAttachmentDescriptorArray *, setObject, MTLRenderPassColorAttachmentDescriptor * _Nullable, atIndexedSubscript, NSUInteger)
    fn MTLRenderPassColorAttachmentDescriptorArray_setObjectAtIndexedSubscript(
        id: &mut Id,
        value: Option<&ColorAttachmentDescriptor>,
        index: usize,
    );
}
