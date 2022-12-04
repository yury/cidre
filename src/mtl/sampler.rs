use crate::{define_mtl, define_obj_type, ext_msg_send, ns};

#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum MinMagFilter {
    Nearest = 0,
    Linear = 1,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum MipFilter {
    NotMipmapped = 0,
    Nearest = 1,
    Linear = 2,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum AddressMode {
    ClampToEdge = 0,
    MirrorClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
    ClampToZero = 4,
    ClampToBorderColor = 5,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum BorderColor {
    TransparentBlack = 0, // {0,0,0,0}
    OpaqueBlack = 1,      // {0,0,0,1}
    OpaqueWhite = 2,      // {1,1,1,1}
}

define_obj_type!(Descriptor(ns::Id));

define_obj_type!(State(ns::Id));

impl State {
    define_mtl!(device, label, gpu_resouce_id);
}
