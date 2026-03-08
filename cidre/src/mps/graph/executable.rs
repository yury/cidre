#[allow(unused_imports)]
use crate::api;
use crate::{arc, blocks, define_obj_type, mps::graph, mtl, ns, objc};

#[doc(alias = "MPSGraphExecutableCompletionHandler")]
pub type CompletionHandler = blocks::ResultCh<ns::Array<graph::TensorData>>;

#[doc(alias = "MPSGraphExecutableScheduledHandler")]
pub type ScheduledHandler = blocks::ResultCh<ns::Array<graph::TensorData>>;

define_obj_type!(
    #[doc(alias = "MPSGraphExecutableExecutionDescriptor")]
    pub ExecutionDesc(ns::Id),
    MPS_GRAPH_EXECUTABLE_EXECUTION_DESCRIPTOR
);

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
    pub Executable(ns::Id),
    MPS_GRAPH_EXECUTABLE
);

impl arc::A<Executable> {
    #[objc::msg_send(initWithMPSGraphPackageAtURL:compilationDescriptor:)]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0)]
    pub fn init_with_mps_graph_package_at_url(
        self,
        mps_graph_package_url: &ns::Url,
        compilation_descriptor: Option<&graph::CompilationDesc>,
    ) -> arc::R<Executable>;

    #[objc::msg_send(initWithCoreMLPackageAtURL:compilationDescriptor:)]
    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0)]
    pub fn init_with_core_ml_package_at_url(
        self,
        core_ml_package_url: &ns::Url,
        compilation_descriptor: Option<&graph::CompilationDesc>,
    ) -> arc::R<Executable>;
}

impl Executable {
    #[objc::msg_send(options)]
    pub fn opts(&self) -> graph::Opts;

    #[objc::msg_send(feedTensors)]
    pub fn feed_tensors(&self) -> Option<arc::R<ns::Array<graph::Tensor>>>;

    #[objc::msg_send(targetTensors)]
    pub fn target_tensors(&self) -> Option<arc::R<ns::Array<graph::Tensor>>>;

    #[objc::msg_send(specializeWithDevice:inputTypes:compilationDescriptor:)]
    pub fn specialize_with_device(
        &self,
        device: Option<&graph::Device>,
        input_types: &ns::Array<graph::GraphType>,
        compilation_descriptor: Option<&graph::CompilationDesc>,
    );

    #[objc::msg_send(getOutputTypesWithDevice:inputTypes:compilationDescriptor:)]
    #[api::available(macos = 13.2, ios = 16.3, tvos = 16.3)]
    pub fn output_types_with_device(
        &self,
        device: Option<&graph::Device>,
        input_types: &ns::Array<graph::GraphType>,
        compilation_descriptor: Option<&graph::CompilationDesc>,
    ) -> Option<arc::R<ns::Array<graph::ShapedType>>>;

    #[objc::msg_send(runWithMTLCommandQueue:inputsArray:resultsArray:executionDescriptor:)]
    pub fn run_with_cmd_queue(
        &self,
        command_queue: &mtl::CmdQueue,
        inputs_array: &ns::Array<graph::TensorData>,
        results_array: Option<&ns::Array<graph::TensorData>>,
        execution_descriptor: Option<&ExecutionDesc>,
    ) -> arc::R<ns::Array<graph::TensorData>>;

    #[objc::msg_send(runAsyncWithMTLCommandQueue:inputsArray:resultsArray:executionDescriptor:)]
    pub fn run_async_with_cmd_queue(
        &self,
        command_queue: &mtl::CmdQueue,
        inputs_array: &ns::Array<graph::TensorData>,
        results_array: Option<&ns::Array<graph::TensorData>>,
        execution_descriptor: Option<&ExecutionDesc>,
    ) -> arc::R<ns::Array<graph::TensorData>>;

    #[inline]
    #[api::available(macos = 14.0, ios = 17.0, tvos = 17.0)]
    pub fn with_mps_graph_package_at_url(
        mps_graph_package_url: &ns::Url,
        compilation_descriptor: Option<&graph::CompilationDesc>,
    ) -> arc::R<Self> {
        Self::alloc()
            .init_with_mps_graph_package_at_url(mps_graph_package_url, compilation_descriptor)
    }

    #[inline]
    #[api::available(macos = 15.0, ios = 18.0, tvos = 18.0)]
    pub fn with_core_ml_package_at_url(
        core_ml_package_url: &ns::Url,
        compilation_descriptor: Option<&graph::CompilationDesc>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_core_ml_package_at_url(core_ml_package_url, compilation_descriptor)
    }
}

#[link(name = "mpsg", kind = "static")]
unsafe extern "C" {
    static MPS_GRAPH_EXECUTABLE_EXECUTION_DESCRIPTOR: &'static objc::Class<ExecutionDesc>;
    static MPS_GRAPH_EXECUTABLE: &'static objc::Class<Executable>;
}
