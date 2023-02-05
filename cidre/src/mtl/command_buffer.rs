use std::ffi::c_void;

use crate::{arc, blocks, define_mtl, define_obj_type, mtl, ns, objc};

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

    #[objc::msg_send(addScheduledHandler:)]
    fn _add_scheduled_handler(&self, block: *mut c_void);

    pub fn add_scheduled_handler<F>(&self, block: &'static mut blocks::Block<F>)
    where
        F: FnOnce(&Self) + Send + 'static,
    {
        self._add_scheduled_handler(block.as_ptr());
    }

    #[objc::msg_send(addCompletionHandler:)]
    fn _add_completion_handler(&self, block: *mut c_void);

    pub fn add_completion_handler<F>(&self, block: &'static mut blocks::Block<F>)
    where
        F: FnOnce(&Self) + Send + 'static,
    {
        self._add_completion_handler(block.as_ptr());
    }

    #[objc::msg_send(blitCommandEncoder)]
    pub fn blit_command_encoder_ar(&self) -> Option<arc::Rar<mtl::BlitCommandEncoder>>;

    #[objc::rar_retain()]
    pub fn blit_command_encoder(&self) -> Option<arc::R<mtl::BlitCommandEncoder>>;

    #[objc::msg_send(computeCommandEncoder)]
    pub fn compute_command_encoder_ar(&self) -> Option<arc::Rar<mtl::ComputeCommandEncoder>>;

    #[objc::rar_retain()]
    pub fn compute_command_encoder(&self) -> Option<arc::R<mtl::ComputeCommandEncoder>>;

    #[objc::msg_send(computeCommandEncoderWithDescriptor:)]
    pub fn compute_command_encoder_with_descriptor_ar(
        &self,
        descriptor: &mtl::ComputePassDescriptor,
    ) -> Option<arc::Rar<mtl::ComputeCommandEncoder>>;

    #[objc::rar_retain()]
    pub fn compute_command_encoder_with_descriptor(
        &self,
        descriptor: &mtl::ComputePassDescriptor,
    ) -> Option<arc::R<mtl::ComputeCommandEncoder>>;

    #[objc::msg_send(renderCommandEncoderWithDescriptor:)]
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
