use std::{ffi::c_void, mem::transmute};

use crate::{blocks::Block, cf, define_obj_type, mps::graph, msg_send, ns};

define_obj_type!(ExecutionDescriptor(ns::Id));

impl ExecutionDescriptor {
    /// A notification when graph executable execution: has been scheduled
    pub fn scheduled_handler<F>(&self) -> Option<&Block<F>>
    where
        F: FnOnce(&cf::ArrayOf<graph::TensorData>, Option<&cf::Error>),
    {
        unsafe { transmute(rsel_scheduledHandler(self)) }
    }

    pub fn set_scheduled_handler<F>(&self, block: Option<&'static mut Block<F>>)
    where
        F: FnOnce(&cf::ArrayOf<graph::TensorData>, Option<&cf::Error>),
    {
        unsafe {
            let ptr = block.map_or(std::ptr::null_mut(), |b| b.as_ptr());
            wsel_setScheduledHandler(self, ptr)
        }
    }

    /// A notification when graph executable execution: has finished
    /// If no error, the results produced by the graph operation.
    /// If Graph has not yet allocated the results this will be NSNull
    pub fn completion_handler<F>(&self) -> Option<&Block<F>>
    where
        F: FnOnce(&cf::ArrayOf<graph::TensorData>, Option<&cf::Error>),
    {
        unsafe { transmute(rsel_completionHandler(self)) }
    }

    pub fn set_completion_handler<F>(&self, block: Option<&'static mut Block<F>>)
    where
        F: FnOnce(&cf::ArrayOf<graph::TensorData>, Option<&cf::Error>),
    {
        unsafe {
            let ptr = block.map_or(std::ptr::null_mut(), |b| b.as_ptr());
            wsel_setCompletionHandler(self, ptr)
        }
    }

    pub fn wait_until_completed(&self) -> bool {
        msg_send!("mpsg", self, sel_waitUntilCompleted)
    }

    pub fn set_wait_until_completed(&self, value: bool) {
        msg_send!("mpsg", self, sel_setWaitUntilCompleted, value)
    }
}

define_obj_type!(Executable(ns::Id));

impl Executable {
    pub fn options(&self) -> graph::Options {
        todo!()
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_scheduledHandler(id: &ns::Id) -> *mut c_void;
    fn wsel_setScheduledHandler(id: &ns::Id, block: *mut c_void);

    fn rsel_completionHandler(id: &ns::Id) -> *mut c_void;
    fn wsel_setCompletionHandler(id: &ns::Id, block: *mut c_void);
}
