use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UILayoutGuide")]
    pub LayoutGuide(ns::Id),
    sym UILayoutGuide
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

ns::impl_layout_anchors!(LayoutGuide);
