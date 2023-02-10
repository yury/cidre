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

define_obj_type!(CmdBuf(ns::Id));

impl CmdBuf {
    define_mtl!(device, label, set_label, push_debug_group, pop_debug_group);

    #[objc::msg_send(commandQueue)]
    pub fn cmd_queue(&self) -> &mtl::CmdQueue;

    #[objc::msg_send(enqueue)]
    pub fn enque(&self);

    #[objc::msg_send(commit)]
    pub fn commit(&self);

    #[objc::msg_send(waitUntilScheduled)]
    pub fn wait_until_scheduled(&self);

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
    pub fn new_blit_cmd_enc_ar(&self) -> Option<&'ar mtl::BlitCmdEncoder>;

    #[objc::rar_retain()]
    pub fn new_blit_cmd_enc(&self) -> Option<arc::R<mtl::BlitCmdEncoder>>;

    #[objc::msg_send(computeCommandEncoder)]
    pub fn new_compute_cmd_enc_ar(&self) -> Option<&'ar mtl::ComputeCmdEncoder>;

    #[objc::rar_retain()]
    pub fn new_compute_cmd_enc(&self) -> Option<arc::R<mtl::ComputeCmdEncoder>>;

    #[objc::msg_send(computeCommandEncoderWithDescriptor:)]
    pub fn new_compute_cmd_enc_desc_ar(
        &self,
        descriptor: &mtl::ComputePassDescriptor,
    ) -> Option<&'ar mtl::ComputeCmdEncoder>;

    #[objc::rar_retain()]
    pub fn new_compute_cmd_enc_desc(
        &self,
        descriptor: &mtl::ComputePassDescriptor,
    ) -> Option<arc::R<mtl::ComputeCmdEncoder>>;

    #[objc::msg_send(renderCommandEncoderWithDescriptor:)]
    pub fn new_render_cmd_enc_ar(
        &self,
        descriptor: &mtl::RenderPassDescriptor,
    ) -> Option<&'ar mtl::RenderCmdEncoder>;

    #[objc::rar_retain()]
    pub fn new_render_cmd_enc(
        &self,
        descriptor: &mtl::RenderPassDescriptor,
    ) -> Option<arc::R<mtl::RenderCmdEncoder>>;
}
