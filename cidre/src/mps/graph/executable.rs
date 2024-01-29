use crate::{blocks, define_obj_type, mps::graph, ns, objc};

#[doc(alias = "MPSGraphExecutableCompletionHandler")]
pub type CompletionHandler = blocks::ResultCompletionHandler<ns::Array<graph::TensorData>>;

#[doc(alias = "MPSGraphExecutableScheduledHandler")]
pub type ScheduledHandler = blocks::ResultCompletionHandler<ns::Array<graph::TensorData>>;

define_obj_type!(pub ExecutionDesc(ns::Id));

impl ExecutionDesc {
    /// A notification when graph executable execution: has been scheduled
    #[objc::msg_send(scheduledHandler)]
    pub fn scheduled_handler(&self) -> Option<&graph::ExecutableScheduledHandler>;

    #[objc::msg_send(setScheduledHandler:)]
    fn set_scheduled_handler(&mut self, val: Option<&mut graph::ExecutableScheduledHandler>);

    /// A notification when graph executable execution: has finished
    /// If no error, the results produced by the graph operation.
    /// If Graph has not yet allocated the results this will be NSNull
    #[objc::msg_send(completionHandler)]
    pub fn completion_handler(&self) -> Option<&graph::ExecutableCompletionHandler>;

    #[objc::msg_send(setCompletionHandler:)]
    fn set_completion_handler(&mut self, val: Option<&mut graph::ExecutableCompletionHandler>);

    #[objc::msg_send(waitUntilCompleted)]
    pub fn wait_until_completed(&self) -> bool;

    #[objc::msg_send(setWaitUntilCompleted:)]
    pub fn set_wait_until_completed(&self, val: bool);
}

define_obj_type!(
    #[doc(alias = "MPSGraphExecutable")]
    pub Executable(ns::Id)
);

impl Executable {
    #[objc::msg_send(options)]
    pub fn opts(&self) -> graph::Opts;
}
