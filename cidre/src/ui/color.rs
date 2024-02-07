use crate::{arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIColor")]
    pub Color(ns::Id), UI_COLOR
);

unsafe impl Send for Color {}

impl arc::A<Color> {
    #[objc::msg_send(initWithWhite:alpha:)]
    pub fn init_with_white_alpha(self, white: cg::Float, alpha: cg::Float) -> arc::R<Color>;
}

impl Color {
    #[inline]
    pub fn with_white_alpha(white: cg::Float, alpha: cg::Float) -> arc::R<Self> {
        Self::alloc().init_with_white_alpha(white, alpha)
    }

    #[objc::cls_msg_send(colorWithRed:green:blue:alpha:)]
    pub fn with_rgba_ar(r: cg::Float, g: cg::Float, b: cg::Float, a: cg::Float) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_rgba(r: cg::Float, g: cg::Float, b: cg::Float, a: cg::Float) -> arc::R<Self>;
}

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_COLOR: &'static objc::Class<Color>;
}
