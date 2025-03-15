use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    pub TraitCollection(ns::Id),
    UI_TRAIT_COLLECTION
);

impl TraitCollection {
    #[objc::msg_send(userInterfaceIdiom)]
    pub fn ui_idiom(&self) -> ui::Idiom;

    #[objc::msg_send(userInterfaceStyle)]
    pub fn ui_style(&self) -> ui::Style;

    #[objc::msg_send(layoutDirection)]
    pub fn layout_direction(&self) -> ui::LayoutDirection;

    #[objc::msg_send(displayScale)]
    pub fn display_scale(&self) -> cg::Float;

    #[objc::msg_send(horizontalSizeClass)]
    pub fn horizontal_size_class(&self) -> ui::SizeClass;

    #[objc::msg_send(verticalSizeClass)]
    pub fn vertical_size_class(&self) -> ui::SizeClass;

    #[objc::msg_send(displayGamut)]
    pub fn display_gamut(&self) -> ui::DisplayGamut;

    // pub fn scene_capture_state(&self) -> ui::SceneCa
}

unsafe extern "C" {
    static UI_TRAIT_COLLECTION: &'static objc::Class<TraitCollection>;
}
