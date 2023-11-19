use crate::{define_obj_type, objc};

define_obj_type!(pub Type(objc::Id));

define_obj_type!(pub ShapedType(Type));

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
#[doc(alias = "MPSGraphTensorNamedDataLayout")]
pub enum TensorNamedDataLayout {
    /// LayoutNCHW
    Nchw = 0,

    /// LayoutNHWC
    Nhwc = 1,

    /// LayoutOIHW
    Oihw = 2,

    /// LayoutHWIO
    Hwio = 3,

    /// LayoutCHW
    Chw = 4,

    /// LayoutHWC
    Hwc = 5,

    /// LayoutHW
    Hw = 6,
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
