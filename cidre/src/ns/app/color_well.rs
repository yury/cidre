use crate::{arc, define_obj_type, ns, objc};
#[doc(alias = "NSColorWellStyle")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum ColorWellStyle {
    Default = 0,
    Minimal = 1,
    Expanded = 2,
}
define_obj_type!(#[doc(alias = "NSColorWell")] pub ColorWell(ns::Control), NS_COLOR_WELL);
impl ColorWell {
    #[objc::msg_send(colorWellWithStyle:)]
    #[objc::available(macos = 13.0)]
    pub fn with_style(style: ColorWellStyle) -> arc::R<Self>;
    #[objc::msg_send(color)]
    pub fn color(&self) -> arc::R<ns::Color>;
    #[objc::msg_send(setColor:)]
    pub fn set_color(&mut self, value: &ns::Color);
    #[objc::msg_send(supportsAlpha)]
    #[objc::available(macos = 14.0)]
    pub fn supports_alpha(&self) -> bool;
    #[objc::msg_send(setSupportsAlpha:)]
    #[objc::available(macos = 14.0)]
    pub fn set_supports_alpha(&mut self, value: bool);
    #[objc::msg_send(deactivate)]
    pub fn deactivate(&mut self);
}
unsafe extern "C" {
    static NS_COLOR_WELL: &'static objc::Class<ColorWell>;
}
