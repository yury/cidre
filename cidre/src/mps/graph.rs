mod core;
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
pub use executable::Executable;
pub use executable::ExecutionDesc as ExecutableExecutionDesc;

mod tensor_data;
pub use tensor_data::TensorData;

mod tensor;
pub use tensor::Tensor;

mod operation;
pub use operation::Op;

mod memory_ops;
pub use memory_ops::VariableOp;

mod convolution_ops;
pub use convolution_ops::Convolution2dOpDesc;

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
pub use rnn_ops::RNNActivation;
pub use rnn_ops::SingleGateRNNDesc;

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

define_obj_type!(pub Graph(ns::Id), MPS_GRAPH);

impl Graph {
    #[objc::msg_send(options)]
    pub fn opts(&self) -> Opts;

    #[objc::msg_send(setOptions:)]
    pub fn set_opts(&mut self, value: Opts);
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    static MPS_GRAPH: &'static objc::Class<Graph>;
}
