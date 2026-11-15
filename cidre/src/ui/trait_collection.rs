use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    pub TraitCollection(ns::Id),
    sym UITraitCollection
);

impl TraitCollection {
    /// A collection specifying only the horizontal size class, for overrides.
    #[objc::msg_send(traitCollectionWithHorizontalSizeClass:)]
    pub fn with_horizontal_size_class(val: ui::SizeClass) -> arc::R<Self>;

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

    /// The edge where the system places the vertical bar, whether or not
    /// one is visible; unspecified where the system never places one.
    #[objc::msg_send(verticalBarEdge)]
    #[objc::available(ios = 27.1, tvos = 27.1, visionos = 27.1)]
    pub fn vertical_bar_edge(&self) -> ui::VerticalBarEdge;

    #[objc::msg_send(displayGamut)]
    pub fn display_gamut(&self) -> ui::DisplayGamut;

    /// Whether the collection is from a view in a tab accessory, and how the
    /// accessory is laid out.
    #[objc::msg_send(tabAccessoryEnvironment)]
    #[objc::available(ios = 26.0)]
    pub fn tab_accessory_env(&self) -> ui::TabAccessoryEnv;

    // pub fn scene_capture_state(&self) -> ui::SceneCa
}

define_obj_type!(
    #[doc(alias = "UIMutableTraits")]
    pub MutableTraits(ns::Id)
);

impl MutableTraits {
    #[objc::msg_send(horizontalSizeClass)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn horizontal_size_class(&self) -> ui::SizeClass;

    #[objc::msg_send(setHorizontalSizeClass:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_horizontal_size_class(&mut self, val: ui::SizeClass);

    #[objc::msg_send(verticalSizeClass)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn vertical_size_class(&self) -> ui::SizeClass;

    #[objc::msg_send(setVerticalSizeClass:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_vertical_size_class(&mut self, val: ui::SizeClass);

    #[objc::msg_send(userInterfaceStyle)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn ui_style(&self) -> ui::Style;

    #[objc::msg_send(setUserInterfaceStyle:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_ui_style(&mut self, val: ui::Style);

    #[objc::msg_send(displayScale)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn display_scale(&self) -> cg::Float;

    #[objc::msg_send(setDisplayScale:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn set_display_scale(&mut self, val: cg::Float);
}

define_obj_type!(
    #[doc(alias = "UITraitOverrides")]
    pub TraitOverrides(MutableTraits)
);

impl TraitOverrides {
    #[objc::msg_send(containsTrait:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn contains_trait(&self, trait_class: &objc::Class<ns::Id>) -> bool;

    #[objc::msg_send(removeTrait:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn remove_trait(&mut self, trait_class: &objc::Class<ns::Id>);
}
