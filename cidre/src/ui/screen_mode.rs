use crate::{cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIScreenMode")]
    pub ScreenMode(ns::Id)
);

impl ScreenMode {
    /// The width and height in pixels
    #[objc::msg_send(size)]
    pub fn size(&self) -> cg::Size;

    /// The aspect ratio of a single pixel. The ratio is defined as X/Y.
    #[objc::msg_send(pixelAspectRatio)]
    pub fn pixel_aspect_ratio(&self) -> cg::Float;
}
