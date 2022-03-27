use std::ffi::c_void;

use crate::cf::Retained;
use crate::{define_mtl, define_obj_type};

use crate::ns::Id;
use crate::objc::block::CompletionHandlerA;

use super::{BlitCommandEncoder, CommandQueue, ComputeCommandEncoder};

#[repr(usize)]
pub enum Status {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

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

define_obj_type!(CommandBuffer(Id));

impl CommandBuffer {
    define_mtl!(device, label, set_label, push_debug_group, pop_debug_group);

    #[inline]
    pub fn command_queue(&self) -> &CommandQueue {
        unsafe { rsel_commandQueue(self) }
    }

    #[inline]
    pub fn enqueue(&self) {
        unsafe { wsel_enqueue(self) }
    }

    #[inline]
    pub fn commit(&self) {
        unsafe { wsel_commit(self) }
    }

    #[inline]
    pub fn wait_untint_scheduled(&self) {
        unsafe { wsel_waitUntilScheduled(self) }
    }

    #[inline]
    pub fn wait_until_completed(&self) {
        unsafe { wsel_waitUntilCompleted(self) }
    }

    pub fn add_scheduled_handler<B>(&self, block: B)
    where
        B: FnOnce(&Self) + Send + 'static,
    {
        unsafe { sel_addScheduledHandler(self, block.into_raw()) }
    }

    pub fn add_completion_handler<B>(&self, block: B)
    where
        B: FnOnce(&Self) + Send + 'static,
    {
        unsafe { sel_addCompletedHandler(self, block.into_raw()) }
    }

    #[inline]
    pub fn blit_command_encoder<'new>(&self) -> Option<Retained<'new, BlitCommandEncoder>> {
        unsafe { rsel_blitCommandEncoder(self) }
    }

    #[inline]
    pub fn compute_command_encoder<'new>(&self) -> Option<Retained<'new, ComputeCommandEncoder>> {
        unsafe { rsel_computeCommandEncoder(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_commandQueue(id: &Id) -> &CommandQueue;
    fn wsel_enqueue(id: &Id);
    fn wsel_commit(id: &Id);
    fn wsel_waitUntilScheduled(id: &Id);
    fn wsel_waitUntilCompleted(id: &Id);
    fn sel_addScheduledHandler(id: &Id, rb: *const c_void);
    fn sel_addCompletedHandler(id: &Id, rb: *const c_void);

    fn rsel_blitCommandEncoder<'new>(id: &Id) -> Option<Retained<'new, BlitCommandEncoder>>;
    fn rsel_computeCommandEncoder<'new>(id: &Id) -> Option<Retained<'new, ComputeCommandEncoder>>;
}
