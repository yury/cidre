use std::ffi::c_void;

use crate::{define_mtl, define_obj_type};

use crate::objc::block::CompletionHandlerA;
use crate::ns::Id;

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

define_obj_type!(CommandBuffer(Id));

impl CommandBuffer {
    define_mtl!(device, mut label);

    pub fn enqueue(&self) {
        unsafe { wsel_enqueue(self) }
    }

    pub fn commit(&self) {
        unsafe { wsel_commit(self) }
    }

    pub fn wait_untint_scheduled(&self) {
        unsafe { wsel_waitUntilScheduled(self) }
    }

    pub fn wait_until_completed(&self) {
        unsafe { wsel_waitUntilCompleted(self) }
    }

    pub fn add_scheduled_handler<T>(&self, block: T)
    where T: Fn(&CommandBuffer)
    {
        unsafe {
            sel_addScheduledHandler(self, block.into_raw())
        }
    }

    pub fn add_completion_handler<T>(&self, block: T)
    where T: Fn(&CommandBuffer)
    {
        unsafe {
            sel_addCompletedHandler(self, block.into_raw())
        }
    }
}

extern "C" {
    fn wsel_enqueue(id: &Id);
    fn wsel_commit(id: &Id);
    fn wsel_waitUntilScheduled(id: &Id);
    fn wsel_waitUntilCompleted(id: &Id);
    fn sel_addScheduledHandler(id: &Id, rb: *const c_void);
    fn sel_addCompletedHandler(id: &Id, rb: *const c_void);

}
