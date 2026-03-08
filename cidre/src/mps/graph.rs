mod core;
#[cfg(feature = "blocks")]
use crate::blocks;
use crate::mtl;
use crate::{arc, define_obj_type, ns, objc};

pub use self::core::PaddingMode;
pub use self::core::PaddingStyle;
pub use self::core::ReductionMode;
pub use self::core::ShapedType;
pub use self::core::TensorNamedDataLayout;
pub use self::core::Type as GraphType;

mod device;
pub use device::Device;
pub use device::DeviceType;

mod executable;
pub use executable::CompletionHandler as ExecutableCompletionHandler;
pub use executable::Executable;
pub use executable::ExecutionDesc as ExecutableExecutionDesc;
pub use executable::ScheduledHandler as ExecutableScheduledHandler;

mod tensor_data;
pub use tensor_data::TensorData;

mod tensor;
pub use tensor::Tensor;

mod operation;
pub use operation::Op;

mod memory_ops;
pub use memory_ops::VariableOp;

mod convolution_ops;
pub use convolution_ops::Conv2dOpDesc;

mod resize_ops;
pub use resize_ops::ResizeMode;
pub use resize_ops::ResizeNearestRoundingMode;

mod activation_ops;
mod arithmetic_ops;
mod gather_ops;
mod matrix_multiplication_ops;
mod normalization_ops;
mod shape_ops;

mod rnn_ops;
pub use rnn_ops::RnnActivation;
pub use rnn_ops::SingleGateRnnDesc;

/// Options to be utilized by the graph
#[doc(alias = "MPSGraphOptions")]
#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
#[repr(u64)]
pub enum Opts {
    None = 0,

    /// Synchronize results using a blit encoder if on a GPU
    #[default]
    SynchronizeResults = 1,

    /// Prints more logging info
    Verbose = 2,
}

#[doc(alias = "MPSGraphOptimization")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u64)]
pub enum Optimization {
    /// Default optimizations
    Level0 = 0,

    /// Additional Optimizations
    Level1 = 1,
}

impl Default for Optimization {
    #[inline]
    fn default() -> Self {
        Self::Level0
    }
}

#[doc(alias = "MPSGraphOptimizationProfile")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u64)]
pub enum OptimizationProfile {
    /// Default, optimize for performance
    Performance = 0,

    /// Optimize for power efficiency
    Efficiency = 1,
}

impl Default for OptimizationProfile {
    #[inline]
    fn default() -> Self {
        Self::Performance
    }
}

/// Execution events that can be used with shared events
#[doc(alias = "MPSGraphExecutionStage")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u64)]
pub enum ExecutionStage {
    Completed = 0,
}

impl Default for ExecutionStage {
    #[inline]
    fn default() -> Self {
        Self::Completed
    }
}

/// A dictionary of tensors and correspondiing tensorData for them
#[doc(alias = "MPSGraphTensorDataDictionary")]
pub type TensorDataDictionary = ns::Dictionary<Tensor, TensorData>;

/// A dictionary of tensors and correspondiing shapes for them
#[doc(alias = "MPSGraphTensorShapedTypeDictionary")]
pub type TensorShapedTypeDictionary = ns::Dictionary<Tensor, ShapedType>;

#[cfg(feature = "blocks")]
#[doc(alias = "MPSGraphCompletionHandler")]
pub type CompletionHandler = blocks::ResultCh<TensorDataDictionary>;

#[cfg(feature = "blocks")]
#[doc(alias = "MPSGraphScheduledHandler")]
pub type ScheduledHandler = blocks::ResultCh<TensorDataDictionary>;

#[cfg(feature = "blocks")]
#[doc(alias = "MPSGraphCompilationCompletionHandler")]
pub type CompilationCompletionHandler = blocks::ResultCh<Executable>;

define_obj_type!(
    #[doc(alias = "MPSGraphCompilationDescriptor")]
    pub CompilationDesc(ns::Id),
    MPS_GRAPH_COMPILATION_DESCRIPTOR
);

impl CompilationDesc {
    #[objc::msg_send(disableTypeInference)]
    #[objc::available(macos = 12.0, ios = 15.0, tvos = 15.0)]
    pub fn disable_type_inference(&self);

    #[objc::msg_send(optimizationLevel)]
    #[objc::available(macos = 12.3, ios = 15.4, tvos = 15.4)]
    pub fn optimization_level(&self) -> Optimization;

    #[objc::msg_send(setOptimizationLevel:)]
    #[objc::available(macos = 12.3, ios = 15.4, tvos = 15.4)]
    pub fn set_optimization_level(&mut self, value: Optimization);

    #[objc::msg_send(waitForCompilationCompletion)]
    #[objc::available(macos = 13.0, ios = 16.0, tvos = 16.0)]
    pub fn wait_for_compilation_completion(&self) -> bool;

    #[objc::msg_send(setWaitForCompilationCompletion:)]
    #[objc::available(macos = 13.0, ios = 16.0, tvos = 16.0)]
    pub fn set_wait_for_compilation_completion(&mut self, value: bool);
}

define_obj_type!(
    #[doc(alias = "MPSGraphExecutionDescriptor")]
    pub ExecutionDesc(ns::Id),
    MPS_GRAPH_EXECUTION_DESCRIPTOR
);

impl ExecutionDesc {
    #[cfg(feature = "blocks")]
    #[objc::msg_send(scheduledHandler)]
    pub fn scheduled_handler(&self) -> Option<&ScheduledHandler>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setScheduledHandler:)]
    pub fn set_scheduled_handler(&mut self, val: Option<&mut ScheduledHandler>);

    #[cfg(feature = "blocks")]
    #[objc::msg_send(completionHandler)]
    pub fn completion_handler(&self) -> Option<&CompletionHandler>;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(setCompletionHandler:)]
    pub fn set_completion_handler(&mut self, val: Option<&mut CompletionHandler>);

    #[objc::msg_send(waitUntilCompleted)]
    pub fn wait_until_completed(&self) -> bool;

    #[objc::msg_send(setWaitUntilCompleted:)]
    pub fn set_wait_until_completed(&mut self, val: bool);

    #[objc::msg_send(compilationDescriptor)]
    #[objc::available(macos = 12.3, ios = 15.4, tvos = 15.4)]
    pub fn compilation_desc(&self) -> Option<arc::R<CompilationDesc>>;

    #[objc::msg_send(setCompilationDescriptor:)]
    #[objc::available(macos = 12.3, ios = 15.4, tvos = 15.4)]
    pub fn set_compilation_desc(&mut self, val: Option<&CompilationDesc>);
}

define_obj_type!(pub Graph(ns::Id), MPS_GRAPH);

impl Graph {
    #[objc::msg_send(options)]
    pub fn opts(&self) -> Opts;

    #[objc::msg_send(setOptions:)]
    pub fn set_opts(&mut self, value: Opts);

    #[objc::msg_send(compileWithDevice:feeds:targetTensors:targetOperations:compilationDescriptor:)]
    #[objc::available(macos = 12.0, ios = 15.0, tvos = 15.0)]
    pub fn compile_with_device(
        &self,
        device: Option<&Device>,
        feeds: &TensorShapedTypeDictionary,
        target_tensors: &ns::Array<Tensor>,
        target_operations: Option<&ns::Array<Op>>,
        compilation_descriptor: Option<&CompilationDesc>,
    ) -> arc::R<Executable>;

    #[objc::msg_send(runWithFeeds:targetTensors:targetOperations:)]
    pub fn run_with_feeds(
        &self,
        feeds: &TensorDataDictionary,
        target_tensors: &ns::Array<Tensor>,
        target_operations: Option<&ns::Array<Op>>,
    ) -> arc::R<TensorDataDictionary>;

    #[objc::msg_send(runWithMTLCommandQueue:feeds:targetTensors:targetOperations:)]
    pub fn run_with_cmd_queue(
        &self,
        command_queue: &mtl::CmdQueue,
        feeds: &TensorDataDictionary,
        target_tensors: &ns::Array<Tensor>,
        target_operations: Option<&ns::Array<Op>>,
    ) -> arc::R<TensorDataDictionary>;

    #[objc::msg_send(runAsyncWithFeeds:targetTensors:targetOperations:executionDescriptor:)]
    pub fn run_async_with_feeds(
        &self,
        feeds: &TensorDataDictionary,
        target_tensors: &ns::Array<Tensor>,
        target_operations: Option<&ns::Array<Op>>,
        execution_descriptor: Option<&ExecutionDesc>,
    ) -> arc::R<TensorDataDictionary>;

    #[objc::msg_send(runAsyncWithMTLCommandQueue:feeds:targetTensors:targetOperations:executionDescriptor:)]
    pub fn run_async_with_cmd_queue(
        &self,
        command_queue: &mtl::CmdQueue,
        feeds: &TensorDataDictionary,
        target_tensors: &ns::Array<Tensor>,
        target_operations: Option<&ns::Array<Op>>,
        execution_descriptor: Option<&ExecutionDesc>,
    ) -> arc::R<TensorDataDictionary>;
}

#[link(name = "mpsg", kind = "static")]
unsafe extern "C" {
    static MPS_GRAPH_COMPILATION_DESCRIPTOR: &'static objc::Class<CompilationDesc>;
    static MPS_GRAPH_EXECUTION_DESCRIPTOR: &'static objc::Class<ExecutionDesc>;
    static MPS_GRAPH: &'static objc::Class<Graph>;
}

#[cfg(not(target_os = "watchos"))]
#[link(name = "MetalPerformanceShadersGraph", kind = "framework")]
unsafe extern "C" {}

#[cfg(test)]
mod tests {
    use crate::{arc, mps, mps::graph, mtl, ns};

    fn f32_data(values: &[f32]) -> arc::R<ns::Data> {
        let bytes = unsafe {
            std::slice::from_raw_parts(values.as_ptr().cast::<u8>(), std::mem::size_of_val(values))
        };
        ns::Data::with_bytes(bytes)
    }

    #[test]
    fn basics() {
        let device = graph::Device::with_mtl_device(&mtl::Device::sys_default().unwrap());

        assert_eq!(device.device_type(), graph::DeviceType::Metal);
    }

    #[test]
    fn run_add_graph() {
        let mtl_device = mtl::Device::sys_default().unwrap();
        let graph_device = graph::Device::with_mtl_device(&mtl_device);
        let graph = graph::Graph::new();
        let shape = ns::arr![2];
        let lhs = graph.placeholder_with_shape(Some(shape.as_ref()), mps::DType::F32, None);
        let rhs = graph.placeholder_with_shape(Some(shape.as_ref()), mps::DType::F32, None);
        let out = graph.add(&lhs, &rhs, None);

        let lhs_data = graph::TensorData::with_data(
            &graph_device,
            &f32_data(&[1.0, 2.0]),
            shape.as_ref(),
            mps::DType::F32,
        );
        let rhs_data = graph::TensorData::with_data(
            &graph_device,
            &f32_data(&[3.0, 4.0]),
            shape.as_ref(),
            mps::DType::F32,
        );
        let feeds = ns::Dictionary::with_keys_values(
            &[lhs.as_ref(), rhs.as_ref()],
            &[lhs_data.as_ref(), rhs_data.as_ref()],
        );
        let target_tensors = ns::Array::from_slice(&[out.as_ref()]);
        let results = graph.run_with_feeds(&feeds, &target_tensors, None);
        let result = results.get(out.as_ref()).unwrap();
        let ndarray = result.mps_nd_array();
        let mut bytes = [0u8; std::mem::size_of::<f32>() * 2];
        let mut strides = [0isize; 1];
        ndarray.read_bytes(bytes.as_mut_ptr(), strides.as_mut_ptr());
        let values = unsafe { std::slice::from_raw_parts(bytes.as_ptr().cast::<f32>(), 2) };
        assert_eq!(values, &[4.0, 6.0]);
    }
}
