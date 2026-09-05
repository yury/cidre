use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIButtonConfiguration")]
    pub ButtonCfg(ns::Id),
    UI_BUTTON_CONFIGURATION
);

impl ButtonCfg {
    #[objc::available(ios = 15.0)]
    #[objc::msg_send(plainButtonConfiguration)]
    pub fn plain() -> arc::R<Self>;

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(tintedButtonConfiguration)]
    pub fn tinted() -> arc::R<Self>;

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(filledButtonConfiguration)]
    pub fn filled() -> arc::R<Self>;

    #[objc::msg_send(glassButtonConfiguration)]
    #[objc::available(ios = 26.0)]
    pub fn glass() -> arc::R<Self>;

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setTitle:)]
    pub fn set_title(&mut self, val: Option<&ns::String>);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(title)]
    pub fn title(&self) -> Option<arc::R<ns::String>>;

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ui::Image>);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setImagePadding:)]
    pub fn set_image_padding(&mut self, val: cg::Float);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setBaseForegroundColor:)]
    pub fn set_base_fg_color(&mut self, val: Option<&ui::Color>);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setCornerStyle:)]
    pub fn set_corner_style(&mut self, val: CornerStyle);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setButtonSize:)]
    pub fn set_button_size(&mut self, val: Size);
}

unsafe extern "C" {
    static UI_BUTTON_CONFIGURATION: &'static objc::Class<ButtonCfg>;
}

#[doc(alias = "UIButtonConfigurationSize")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum Size {
    Medium,
    Small,
    Mini,
    Large,
}

#[doc(alias = "UIButtonConfigurationCornerStyle")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum CornerStyle {
    Fixed = -1,
    Dynamic,
    Small,
    Medium,
    Large,
    Capsule,
}
