#[cfg(feature = "ca")]
use crate::ca;
use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIView")]
    pub View(ns::Id), UI_VIEW
);

impl View {
    #[cfg(feature = "ca")]
    #[objc::msg_send(layer)]
    pub fn layer(&self) -> &ca::Layer;

    #[objc::msg_send(backgroundColor)]
    pub fn background_color(&self) -> Option<&ui::Color>;

    #[objc::msg_send(setBackgroundColor:)]
    pub fn set_background_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(isHidden)]
    pub fn is_hidden(&self) -> bool;

    #[objc::msg_send(setHidden:)]
    pub fn set_hidden(&self, val: bool);
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_VIEW: &'static objc::Class<View>;
}
