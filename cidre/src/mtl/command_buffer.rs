use std::ffi::c_void;

use crate::{arc, blocks, define_mtl, define_obj_type, msg_send, mtl, ns, objc};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Status {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Error {
    None = 0,
    Internal = 1,
    Timeout = 2,
    PageFault = 3,
    AccessRevoked = 4,
    NotPermitted = 7,
    OutOfMemory = 8,
    InvalidResource = 9,
    Memoryless = 10,
    StackOverflow = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum DispatchType {
    Serial,
    Concurrent,
}

define_obj_type!(CommandBuffer(ns::Id));

impl CommandBuffer {
    define_mtl!(device, label, set_label, push_debug_group, pop_debug_group);

    #[objc::msg_send(commandQueue)]
    pub fn command_queue(&self) -> &mtl::CommandQueue;

    #[objc::msg_send(enqueue)]
    pub fn enqueue(&self);

    #[objc::msg_send(commit)]
    pub fn commit(&self);

    #[objc::msg_send(waitUntilScheduled)]
    pub fn wait_untint_scheduled(&self);

    #[objc::msg_send(waitUntilCompleted)]
    pub fn wait_until_completed(&self);

    pub fn add_scheduled_handler<F>(&self, block: &'static mut blocks::Block<F>)
    where
        F: FnOnce(&Self) + Send + 'static,
    {
        unsafe { wsel_addScheduledHandler(self, block.as_ptr()) }
    }

    pub fn add_completion_handler<F>(&self, block: &'static mut blocks::Block<F>)
    where
        F: FnOnce(&Self) + Send + 'static,
    {
        unsafe { wsel_addCompletedHandler(self, block.as_ptr()) }
    }

    pub fn blit_command_encoder<'new>(&self) -> Option<&'new mut mtl::BlitCommandEncoder> {
        msg_send!("mtl", self, sel_blitCommandEncoder)
    }

    #[inline]
    pub fn compute_command_encoder<'new>(&self) -> Option<&'new mut mtl::ComputeCommandEncoder> {
        msg_send!("mtl", self, sel_computeCommandEncoder)
    }

    #[inline]
    pub fn compute_command_encoder_with_descriptor<'new>(
        &self,
        descriptor: &mtl::ComputePassDescriptor,
    ) -> Option<&'new mut mtl::ComputeCommandEncoder> {
        msg_send!(
            "mtl",
            self,
            sel_computeCommandEncoderWithDescriptor,
            descriptor
        )
    }

    #[objc::msg_send(newRenderCommandEncoderWithDescriptor:)]
    pub fn render_command_encoder_with_descriptor_ar(
        &self,
        descriptor: &mtl::RenderPassDescriptor,
    ) -> Option<arc::Rar<mtl::RenderCommandEncoder>>;

    #[objc::rar_retain()]
    pub fn render_command_encoder_with_descriptor(
        &self,
        descriptor: &mtl::RenderPassDescriptor,
    ) -> Option<arc::R<mtl::RenderCommandEncoder>>;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn wsel_addScheduledHandler(id: &ns::Id, rb: *mut c_void);
    fn wsel_addCompletedHandler(id: &ns::Id, rb: *mut c_void);
}
