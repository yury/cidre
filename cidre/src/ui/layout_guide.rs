use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UILayoutGuide")]
    pub LayoutGuide(ns::Id),
    UI_LAYOUT_GUIDE
);

impl LayoutGuide {
    #[objc::msg_send(layoutFrame)]
    pub fn layout_frame(&self) -> cg::Rect;

    #[objc::msg_send(owningView)]
    pub fn owning_view(&self) -> Option<arc::R<ui::View>>;

    #[objc::msg_send(setOwningView:)]
    pub fn set_owning_view(&mut self, val: Option<&ui::View>);

    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setIdentifier:)]
    pub fn set_id(&mut self, val: &ns::String);
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_LAYOUT_GUIDE: &'static objc::Class<LayoutGuide>;
}
