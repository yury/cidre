use crate::{define_mtl, define_obj_type, ns, objc};

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

define_obj_type!(Desc(ns::Id));

impl Desc {
    define_mtl!(label, set_label);

    #[objc::msg_send(minFilter)]
    pub fn min_filter(&self) -> MinMagFilter;

    #[objc::msg_send(setMinFilter:)]
    pub fn set_min_filter(&mut self, val: MinMagFilter);

    #[objc::msg_send(magFilter)]
    pub fn mag_filter(&self) -> MinMagFilter;

    #[objc::msg_send(setMagFilter:)]
    pub fn set_mag_filter(&mut self, val: MinMagFilter);

    #[objc::msg_send(borderColor)]
    pub fn border_color(&self) -> BorderColor;

    #[objc::msg_send(setBorderColor:)]
    pub fn set_border_color(&mut self, val: BorderColor);

    #[objc::msg_send(normalizedCoordinates)]
    pub fn normalized_coordinates(&self) -> bool;

    #[objc::msg_send(setNormalizedCoordinates:)]
    pub fn set_normalized_coordinates(&mut self, val: bool);
}

define_obj_type!(State(ns::Id));

impl State {
    define_mtl!(device, label, gpu_resource_id);
}
