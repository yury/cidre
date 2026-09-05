use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIListContentConfiguration")]
    pub ListContentCfg(ns::Id),
    UI_LIST_CONTENT_CONFIGURATION
);

impl ListContentCfg {
    #[objc::msg_send(cellConfiguration)]
    #[objc::available(ios = 14.0)]
    pub fn cell() -> arc::R<Self>;

    #[objc::msg_send(setText:)]
    pub fn set_text(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(setImage:)]
    pub fn set_image(&mut self, val: Option<&ui::Image>);
}

unsafe extern "C" {
    static UI_LIST_CONTENT_CONFIGURATION: &'static objc::Class<ListContentCfg>;
}
