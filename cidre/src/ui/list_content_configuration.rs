use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIListContentConfiguration")]
    pub ListContentCfg(ns::Id),
    UI_LIST_CONTENT_CONFIGURATION
);

impl ListContentCfg {
    #[objc::msg_send(cellConfiguration)]
    #[objc::available(ios = 14.0)]
    pub fn cell() -> arc::R<Self>;

    #[objc::msg_send(subtitleCellConfiguration)]
    #[objc::available(ios = 14.0)]
    pub fn subtitle_cell() -> arc::R<Self>;

    #[objc::msg_send(valueCellConfiguration)]
    #[objc::available(ios = 14.0)]
    pub fn value_cell() -> arc::R<Self>;

    #[objc::msg_send(setText:)]
    pub fn set_text(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(setSecondaryText:)]
    pub fn set_secondary_text(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ui::Image>);

    /// The primary text's look; changes apply to this configuration.
    #[objc::msg_send(textProperties)]
    pub fn text_props(&self) -> arc::R<ListContentTextProps>;

    #[objc::msg_send(secondaryTextProperties)]
    pub fn secondary_text_props(&self) -> arc::R<ListContentTextProps>;

    #[objc::msg_send(imageProperties)]
    pub fn image_props(&self) -> arc::R<ListContentImageProps>;

    #[objc::msg_send(setDirectionalLayoutMargins:)]
    pub fn set_directional_layout_margins(&mut self, val: ui::DirectionalEdgeInsets);

    #[objc::msg_send(setImageToTextPadding:)]
    pub fn set_image_to_text_padding(&mut self, val: cg::Float);

    #[objc::msg_send(setTextToSecondaryTextVerticalPadding:)]
    pub fn set_text_to_secondary_text_vertical_padding(&mut self, val: cg::Float);
}

#[doc(alias = "UIListContentTextAlignment")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ListContentTextAlignment {
    Natural = 0,
    Center = 1,
    Justified = 2,
}

define_obj_type!(
    #[doc(alias = "UIListContentTextProperties")]
    pub ListContentTextProps(ns::Id)
);

impl ListContentTextProps {
    #[objc::msg_send(font)]
    pub fn font(&self) -> arc::R<ui::Font>;

    #[objc::msg_send(setFont:)]
    pub fn set_font(&mut self, val: &ui::Font);

    #[objc::msg_send(color)]
    pub fn color(&self) -> arc::R<ui::Color>;

    #[objc::msg_send(setColor:)]
    pub fn set_color(&mut self, val: &ui::Color);

    #[objc::msg_send(alignment)]
    pub fn alignment(&self) -> ListContentTextAlignment;

    #[objc::msg_send(setAlignment:)]
    pub fn set_alignment(&mut self, val: ListContentTextAlignment);

    #[objc::msg_send(lineBreakMode)]
    pub fn line_break_mode(&self) -> ui::LineBreakMode;

    #[objc::msg_send(setLineBreakMode:)]
    pub fn set_line_break_mode(&mut self, val: ui::LineBreakMode);

    #[objc::msg_send(numberOfLines)]
    pub fn number_of_lines(&self) -> ns::Integer;

    #[objc::msg_send(setNumberOfLines:)]
    pub fn set_number_of_lines(&mut self, val: ns::Integer);

    #[objc::msg_send(adjustsFontForContentSizeCategory)]
    pub fn adjusts_font_for_content_size_category(&self) -> bool;

    #[objc::msg_send(setAdjustsFontForContentSizeCategory:)]
    pub fn set_adjusts_font_for_content_size_category(&mut self, val: bool);
}

define_obj_type!(
    #[doc(alias = "UIListContentImageProperties")]
    pub ListContentImageProps(ns::Id)
);

impl ListContentImageProps {
    #[objc::msg_send(tintColor)]
    pub fn tint_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setTintColor:)]
    pub fn set_tint_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(preferredSymbolConfiguration)]
    pub fn preferred_symbol_cfg(&self) -> Option<arc::R<ui::ImageSymbolCfg>>;

    #[objc::msg_send(setPreferredSymbolConfiguration:)]
    pub fn set_preferred_symbol_cfg(&mut self, val: Option<&ui::ImageSymbolCfg>);

    #[objc::msg_send(maximumSize)]
    pub fn max_size(&self) -> cg::Size;

    #[objc::msg_send(setMaximumSize:)]
    pub fn set_max_size(&mut self, val: cg::Size);

    #[objc::msg_send(cornerRadius)]
    pub fn corner_radius(&self) -> cg::Float;

    #[objc::msg_send(setCornerRadius:)]
    pub fn set_corner_radius(&mut self, val: cg::Float);
}

define_obj_type!(
    /// A cell's or button's background.
    #[doc(alias = "UIBackgroundConfiguration")]
    pub BgCfg(ns::Id),
    UI_BACKGROUND_CONFIGURATION
);

impl BgCfg {
    #[objc::msg_send(clearConfiguration)]
    #[objc::available(ios = 14.0)]
    pub fn clear() -> arc::R<Self>;

    #[objc::msg_send(listCellConfiguration)]
    #[objc::available(ios = 18.0)]
    pub fn list_cell() -> arc::R<Self>;

    #[objc::msg_send(listHeaderConfiguration)]
    #[objc::available(ios = 18.0)]
    pub fn list_header() -> arc::R<Self>;

    #[objc::msg_send(backgroundColor)]
    pub fn bg_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setBackgroundColor:)]
    pub fn set_bg_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(cornerRadius)]
    pub fn corner_radius(&self) -> cg::Float;

    #[objc::msg_send(setCornerRadius:)]
    pub fn set_corner_radius(&mut self, val: cg::Float);

    #[objc::msg_send(backgroundInsets)]
    pub fn bg_insets(&self) -> ui::DirectionalEdgeInsets;

    #[objc::msg_send(setBackgroundInsets:)]
    pub fn set_bg_insets(&mut self, val: ui::DirectionalEdgeInsets);

    #[objc::msg_send(strokeColor)]
    pub fn stroke_color(&self) -> Option<arc::R<ui::Color>>;

    #[objc::msg_send(setStrokeColor:)]
    pub fn set_stroke_color(&mut self, val: Option<&ui::Color>);

    #[objc::msg_send(strokeWidth)]
    pub fn stroke_width(&self) -> cg::Float;

    #[objc::msg_send(setStrokeWidth:)]
    pub fn set_stroke_width(&mut self, val: cg::Float);

    #[objc::msg_send(customView)]
    pub fn custom_view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(setCustomView:)]
    pub fn set_custom_view(&mut self, val: Option<&ui::View>);
}

unsafe extern "C" {
    static UI_LIST_CONTENT_CONFIGURATION: &'static objc::Class<ListContentCfg>;
    static UI_BACKGROUND_CONFIGURATION: &'static objc::Class<BgCfg>;
}
