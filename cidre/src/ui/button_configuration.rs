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

    #[objc::msg_send(prominentGlassButtonConfiguration)]
    #[objc::available(ios = 26.0)]
    pub fn prominent_glass() -> arc::R<Self>;

    #[objc::msg_send(clearGlassButtonConfiguration)]
    #[objc::available(ios = 26.0)]
    pub fn clear_glass() -> arc::R<Self>;

    #[objc::msg_send(prominentClearGlassButtonConfiguration)]
    #[objc::available(ios = 26.0)]
    pub fn prominent_clear_glass() -> arc::R<Self>;

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(grayButtonConfiguration)]
    pub fn gray() -> arc::R<Self>;

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
    #[objc::msg_send(setTitleLineBreakMode:)]
    pub fn set_title_line_break_mode(&mut self, val: ui::LineBreakMode);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setSubtitle:)]
    pub fn set_subtitle(&mut self, val: Option<&ns::String>);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(subtitle)]
    pub fn subtitle(&self) -> Option<arc::R<ns::String>>;

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setTitleAlignment:)]
    pub fn set_title_alignment(&mut self, val: TitleAlignment);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setImagePlacement:)]
    pub fn set_image_placement(&mut self, val: ui::DirectionalRectEdge);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setBaseBackgroundColor:)]
    pub fn set_base_bg_color(&mut self, val: Option<&ui::Color>);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setPreferredSymbolConfigurationForImage:)]
    pub fn set_preferred_symbol_cfg_for_image(&mut self, val: Option<&ui::ImageSymbolCfg>);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setShowsActivityIndicator:)]
    pub fn set_shows_activity_indicator(&mut self, val: bool);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setTitlePadding:)]
    pub fn set_title_padding(&mut self, val: cg::Float);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setBackground:)]
    pub fn set_background(&mut self, val: &ui::BgCfg);

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(contentInsets)]
    pub fn content_insets(&self) -> ui::DirectionalEdgeInsets;

    #[objc::available(ios = 15.0)]
    #[objc::msg_send(setContentInsets:)]
    pub fn set_content_insets(&mut self, val: ui::DirectionalEdgeInsets);

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

#[doc(alias = "UIButtonConfigurationTitleAlignment")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum TitleAlignment {
    Automatic = 0,
    Leading = 1,
    Center = 2,
    Trailing = 3,
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
