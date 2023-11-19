use crate::{define_obj_type, ns};

define_obj_type!(pub LayerVideoGravity(ns::String));

impl LayerVideoGravity {
    #[inline]
    pub fn resize_aspect() -> &'static Self {
        unsafe { AVLayerVideoGravityResizeAspect }
    }

    #[inline]
    pub fn resize_aspect_fill() -> &'static Self {
        unsafe { AVLayerVideoGravityResizeAspectFill }
    }

    #[inline]
    pub fn resize() -> &'static Self {
        unsafe { AVLayerVideoGravityResize }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVLayerVideoGravityResizeAspect: &'static LayerVideoGravity;
    static AVLayerVideoGravityResizeAspectFill: &'static LayerVideoGravity;
    static AVLayerVideoGravityResize: &'static LayerVideoGravity;
}
