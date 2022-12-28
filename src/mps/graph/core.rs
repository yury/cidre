use crate::{define_obj_type, objc};

define_obj_type!(Type(objc::Id));

define_obj_type!(ShapedType(Type));

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
#[doc(alias = "MPSGraphTensorNamedDataLayout")]
pub enum TensorNamedDataLayout {
    /// LayoutNCHW
    NCHW = 0,

    /// LayoutNHWC
    NHWC = 1,

    /// LayoutOIHW
    OIHW = 2,

    /// LayoutHWIO
    HWIO = 3,

    /// LayoutCHW
    CHW = 4,

    /// LayoutHWC
    HWC = 5,

    /// LayoutHW
    HW = 6,
}

#[doc(alias = "MPSGraphPaddingStyle")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum PaddingStyle {
    Explicit = 0,
    TfValid = 1,
    TfSame = 2,
    ExplicitOffset = 3,
    OnnxSameLower,
}

/// Tensor Padding mode
#[doc(alias = "MPSGraphPaddingMode")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum PaddingMode {
    Constant = 0,
    Reflect = 1,
    Symmetric = 2,
    ClampToEdge = 3,
    Zero = 4,
    Periodic = 5,
    AntiPeriodic = 6,
}

/// Reduction Mode
#[doc(alias = "MPSGraphReductionMode")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum ReductionMode {
    Min = 0,
    Max = 1,
    Sum = 2,
    Product = 3,
    ArgumentMin = 4,
    ArgumentMax = 5,
}
