use crate::{define_mtl, define_obj_type, ns};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum MinMagFilter {
    Nearest = 0,
    Linear = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum MipFilter {
    NotMipmapped = 0,
    Nearest = 1,
    Linear = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum AddressMode {
    ClampToEdge = 0,
    MirrorClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
    ClampToZero = 4,
    ClampToBorderColor = 5,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum BorderColor {
    /// {0,0,0,0}
    TransparentBlack = 0,
    /// {0,0,0,1}
    OpaqueBlack = 1,
    /// {1,1,1,1}
    OpaqueWhite = 2,
}

define_obj_type!(Descriptor(ns::Id));

define_obj_type!(State(ns::Id));

impl State {
    define_mtl!(device, label, gpu_resource_id);
}
