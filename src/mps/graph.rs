mod core;
use crate::cf;
use crate::define_obj_type;
use crate::msg_send;
use crate::ns;

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
pub use executable::ExecutionDescriptor as ExecutableExecutionDescriptor;

mod tensor_data;
pub use tensor_data::TensorData;

mod tensor;
pub use tensor::Tensor;

mod operation;
pub use operation::Operation;

mod memory_ops;
pub use memory_ops::VariableOp;

mod convolution_ops;
pub use convolution_ops::Convolution2DOpDescriptor;

mod resize_ops;
pub use resize_ops::ResizeMode;
pub use resize_ops::ResizeNearestRoundingMode;

mod activation_ops;
mod arithmetic_ops;
mod normalization_ops;
mod shape_ops;

/// Options to be utilized by the graph
#[doc(alias = "MPSGraphOptions")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u64)]
pub enum Options {
    None = 0,

    /// Synchronize results using a blit encoder if on a GPU
    SynchronizeResults = 1,

    /// Prints more logging info
    Verbose = 2,
}

impl Default for Options {
    #[inline]
    fn default() -> Self {
        Options::SynchronizeResults
    }
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
pub type TensorDataDictionary = cf::DictionaryOf<Tensor, TensorData>;

/// A dictionary of tensors and correspondiing shapes for them
#[doc(alias = "MPSGraphTensorShapedTypeDictionary")]
pub type TensorShapedTypeDictionary = cf::DictionaryOf<Tensor, ShapedType>;

define_obj_type!(Graph(ns::Id));

impl Graph {
    #[inline]
    pub fn new() -> cf::Retained<Graph> {
        unsafe { MPSGraph_new() }
    }

    #[inline]
    pub fn options(&self) -> Options {
        msg_send!("mpsg", self, sel_options)
    }

    #[inline]
    pub fn set_options(&mut self, value: Options) {
        msg_send!("mpsg", self, sel_setOptions, value)
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn MPSGraph_new() -> cf::Retained<Graph>;
}
