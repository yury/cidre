use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

#[doc(alias = "MTLSamplerMinMagFilter")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum MinMagFilter {
    Nearest = 0,
    Linear = 1,
}

#[doc(alias = "MTLSamplerMipFilter")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum MipFilter {
    NotMipmapped = 0,
    Nearest = 1,
    Linear = 2,
}

#[doc(alias = "MTLSamplerAddressMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum AddrMode {
    ClampToEdge = 0,
    MirrorClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
    ClampToZero = 4,
    ClampToBorderColor = 5,
}

#[doc(alias = "MTLSamplerBorderColor")]
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

define_obj_type!(
    #[doc(alias = "MTLSamplerDescriptor")]
    pub Desc(ns::Id)
);

impl Desc {
    define_mtl!(set_label);

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

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

define_obj_type!(
    #[doc(alias = "MTLSamplerState")]
    pub State(ns::Id));

impl State {
    define_mtl!(gpu_res_id);

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;
}
