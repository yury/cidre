use crate::{api, arc, define_obj_type, ns, objc, ui};

#[doc(alias = "UIContentUnavailableAlignment")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(isize)]
pub enum ContentUnavailableAlignment {
    Center = 0,
    Natural = 1,
}

define_obj_type!(
    #[doc(alias = "UIContentUnavailableConfiguration")]
    pub ContentUnavailableCfg(ns::Id)
);

impl ui::ContentConfiguration for ContentUnavailableCfg {}

impl ContentUnavailableCfg {
    #[api::available(ios = 17.0, tvos = 17.0)]
    crate::define_cls!(UI_CONTENT_UNAVAILABLE_CONFIGURATION);

    #[objc::msg_send(emptyConfiguration)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn empty() -> arc::R<Self>;

    #[objc::msg_send(loadingConfiguration)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn loading() -> arc::R<Self>;

    #[objc::msg_send(searchConfiguration)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn search() -> arc::R<Self>;

    #[objc::msg_send(setText:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_text(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(setSecondaryText:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_secondary_text(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(setImage:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_image(&mut self, val: Option<&ui::Image>);

    #[objc::msg_send(textProperties)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn text_props(&self) -> arc::R<ui::ContentUnavailableTextProps>;

    #[objc::msg_send(secondaryTextProperties)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn secondary_text_props(&self) -> arc::R<ui::ContentUnavailableTextProps>;

    #[objc::msg_send(setAlignment:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_alignment(&mut self, val: ContentUnavailableAlignment);

    #[objc::msg_send(setDirectionalLayoutMargins:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_directional_layout_margins(&mut self, val: ui::DirectionalEdgeInsets);

    #[objc::msg_send(button)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn button(&self) -> arc::R<ui::ButtonCfg>;

    #[objc::msg_send(setButton:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_button(&mut self, val: &ui::ButtonCfg);

    #[objc::msg_send(buttonProperties)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn button_props(&self) -> arc::R<ContentUnavailableButtonProps>;
}

define_obj_type!(
    #[doc(alias = "UIContentUnavailableButtonProperties")]
    pub ContentUnavailableButtonProps(ns::Id)
);

impl ContentUnavailableButtonProps {
    #[objc::msg_send(primaryAction)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn primary_action(&self) -> Option<arc::R<ui::Action>>;

    #[objc::msg_send(setPrimaryAction:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_primary_action(&mut self, val: Option<&ui::Action>);
}

unsafe extern "C" {
    static UI_CONTENT_UNAVAILABLE_CONFIGURATION: &'static objc::Class<ContentUnavailableCfg>;
}
