use crate::{arc, cg, define_cls, define_obj_type, objc, ui};

#[doc(alias = "UIImageSymbolScale")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ImageSymbolScale {
    /// Use the system default size.
    Default = -1,
    /// Allow the system to pick a size based on the context.
    Unspecified = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
}

#[doc(alias = "UIImageSymbolWeight")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ImageSymbolWeight {
    Unspecified = 0,
    UltraLight = 1,
    Thin = 2,
    Light = 3,
    Regular = 4,
    Medium = 5,
    Semibold = 6,
    Bold = 7,
    Heavy = 8,
    Black = 9,
}

define_obj_type!(
    #[doc(alias = "UIImageSymbolConfiguration")]
    pub ImageSymbolCfg(ui::ImageCfg)
);

impl ImageSymbolCfg {
    define_cls!(UI_IMAGE_SYMBOL_CONFIGURATION);

    #[objc::msg_send(unspecifiedConfiguration)]
    pub fn unspecified() -> arc::R<Self>;

    #[objc::msg_send(configurationWithScale:)]
    pub fn with_scale(scale: ImageSymbolScale) -> arc::R<Self>;

    #[objc::msg_send(configurationWithPointSize:)]
    pub fn with_point_size(point_size: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(configurationWithWeight:)]
    pub fn with_weight(weight: ImageSymbolWeight) -> arc::R<Self>;

    #[objc::msg_send(configurationWithPointSize:weight:)]
    pub fn with_point_size_weight(point_size: cg::Float, weight: ImageSymbolWeight)
    -> arc::R<Self>;

    #[objc::msg_send(configurationWithPointSize:weight:scale:)]
    pub fn with_point_size_weight_scale(
        point_size: cg::Float,
        weight: ImageSymbolWeight,
        scale: ImageSymbolScale,
    ) -> arc::R<Self>;

    #[objc::msg_send(configurationWithTextStyle:)]
    pub fn with_text_style(text_style: &ui::TextStyle) -> arc::R<Self>;

    #[objc::msg_send(configurationWithTextStyle:scale:)]
    pub fn with_text_style_scale(
        text_style: &ui::TextStyle,
        scale: ImageSymbolScale,
    ) -> arc::R<Self>;

    /// Adjusts for Dynamic Type.
    #[objc::msg_send(configurationWithFont:)]
    pub fn with_font(font: &ui::Font) -> arc::R<Self>;

    #[objc::msg_send(configurationWithFont:scale:)]
    pub fn with_font_scale(font: &ui::Font, scale: ImageSymbolScale) -> arc::R<Self>;

    #[objc::msg_send(configurationWithHierarchicalColor:)]
    #[objc::available(ios = 15.0, tvos = 15.0, watchos = 8.0)]
    pub fn with_hierarchical_color(color: &ui::Color) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_IMAGE_SYMBOL_CONFIGURATION: &'static objc::Class<ImageSymbolCfg>;
}
