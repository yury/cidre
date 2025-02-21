use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "AVLayerVideoGravity")]
    pub LayerVideoGravity(ns::String)
);

impl LayerVideoGravity {
    #[doc(alias = "AVLayerVideoGravityResizeAspect")]
    #[inline]
    pub fn resize_aspect() -> &'static Self {
        unsafe { AVLayerVideoGravityResizeAspect }
    }

    #[doc(alias = "AVLayerVideoGravityResizeAspectFill")]
    #[inline]
    pub fn resize_aspect_fill() -> &'static Self {
        unsafe { AVLayerVideoGravityResizeAspectFill }
    }

    #[doc(alias = "AVLayerVideoGravityResize")]
    #[inline]
    pub fn resize() -> &'static Self {
        unsafe { AVLayerVideoGravityResize }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
unsafe extern "C" {
    static AVLayerVideoGravityResizeAspect: &'static LayerVideoGravity;
    static AVLayerVideoGravityResizeAspectFill: &'static LayerVideoGravity;
    static AVLayerVideoGravityResize: &'static LayerVideoGravity;
}
