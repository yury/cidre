use std::ffi::c_void;

use crate::{blocks::Block, cf, define_obj_type, mps::graph, ns, objc};

define_obj_type!(pub ExecutionDesc(ns::Id));

impl ExecutionDesc {
    /// A notification when graph executable execution: has been scheduled
    #[objc::msg_send(scheduledHandler)]
    pub fn scheduled_handler<F>(&self) -> Option<&Block<F>>
    where
        F: FnOnce(&cf::ArrayOf<graph::TensorData>, Option<&cf::Error>);

    #[objc::msg_send(setScheduledHandler:)]
    fn _set_scheduled_handler(&self, block: *mut c_void);

    pub fn set_scheduled_handler<F>(&self, block: Option<&'static mut Block<F>>)
    where
        F: FnOnce(&cf::ArrayOf<graph::TensorData>, Option<&cf::Error>),
    {
        let ptr = block.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr());
        self._set_scheduled_handler(ptr)
    }

    /// A notification when graph executable execution: has finished
    /// If no error, the results produced by the graph operation.
    /// If Graph has not yet allocated the results this will be NSNull
    #[objc::msg_send(completionHandler)]
    pub fn ch<F>(&self) -> Option<&Block<F>>
    where
        F: FnOnce(&cf::ArrayOf<graph::TensorData>, Option<&cf::Error>);

    #[objc::msg_send(setCompletionHandler:)]
    fn _set_ch(&self, block: *mut c_void);

    pub fn set_ch<F>(&self, block: Option<&'static mut Block<F>>)
    where
        F: FnOnce(&cf::ArrayOf<graph::TensorData>, Option<&cf::Error>),
    {
        let ptr = block.map_or(std::ptr::null_mut(), |b| b.as_mut_ptr());
        self._set_ch(ptr)
    }

    #[objc::msg_send(waitUntilCompleted)]
    pub fn wait_until_completed(&self) -> bool;

    #[objc::msg_send(setWaitUntilCompleted:)]
    pub fn set_wait_until_completed(&self, val: bool);
}

define_obj_type!(pub Executable(ns::Id));

impl Executable {
    #[objc::msg_send(options)]
    pub fn options(&self) -> graph::Options;
}
