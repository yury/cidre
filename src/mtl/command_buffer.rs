use std::ffi::c_void;
use std::intrinsics::transmute;

use crate::cf::Retained;
use crate::cf::runtime::Autoreleased;
use crate::objc::Sel;
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
        unsafe {
            self.rsel(sel_commandQueue)
        }
    }

    #[inline]
    pub fn enqueue(&self) {
        unsafe { self.wsel(sel_enqueue) }
    }

    #[inline]
    pub fn commit(&self) {
        unsafe {
            self.wsel(sel_commit);
        }
    }

    #[inline]
    pub fn wait_untint_scheduled(&self) {
        unsafe { wsel_waitUntilScheduled(self) }
    }

    #[inline]
    pub fn wait_until_completed(&self) {
        unsafe {
            self.wsel(sel_waitUntilCompleted);
        }
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
    pub fn blit_command_encoder<'new>(&self) -> Option<Autoreleased<'new, BlitCommandEncoder>> {
        unsafe { self.rsel(sel_blitCommandEncoder) }
    }

    #[inline]
    pub fn compute_command_encoder<'new>(&self) -> Option<Autoreleased<'new, ComputeCommandEncoder>> {
        unsafe { self.rsel(sel_computeCommandEncoder) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {

    static sel_commit: &'static Sel;
    static sel_enqueue: &'static Sel;
    static sel_waitUntilCompleted: &'static Sel;
    static sel_blitCommandEncoder: &'static Sel;
    static sel_computeCommandEncoder: &'static Sel;
    static sel_commandQueue: &'static Sel;

    // fn rsel_commandQueue(id: &Id) -> &CommandQueue;
    // fn wsel_enqueue(id: &Id);
    fn wsel_waitUntilScheduled(id: &Id);
    fn sel_addScheduledHandler(id: &Id, rb: *const c_void);
    fn sel_addCompletedHandler(id: &Id, rb: *const c_void);

    // fn rsel_blitCommandEncoder<'new>(id: &Id) -> Option<Retained<'new, BlitCommandEncoder>>;
    // fn rsel_computeCommandEncoder<'new>(id: &Id) -> Option<Retained<'new, ComputeCommandEncoder>>;
}
