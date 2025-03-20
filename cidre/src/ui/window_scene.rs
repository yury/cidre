use crate::{arc, cg, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIWindowScene")]
    pub WindowScene(ui::Scene)
);

impl WindowScene {
    #[objc::msg_send(screen)]
    pub fn screen(&self) -> arc::R<ui::Screen>;

    #[objc::msg_send(interfaceOrientation)]
    pub fn interface_orientation(&self) -> ui::Orientation;

    #[objc::msg_send(coordinateSpace)]
    pub fn coordinate_space(&self) -> arc::R<ui::AnyCoordinateSpace>;

    #[objc::msg_send(traitCollection)]
    pub fn trait_collection(&self) -> arc::R<ui::TraitCollection>;

    // TODO: ...

    #[objc::msg_send(sizeRestrictions)]
    pub fn size_restrictions(&self) -> Option<arc::R<SceneSizeRestrictions>>;

    /// The array of all windows associated with this ui::WindowScene
    #[objc::msg_send(windows)]
    pub fn windows(&self) -> arc::R<ns::Array<ui::Window>>;

    #[objc::msg_send(keyWindow)]
    pub fn key_window(&self) -> Option<arc::R<ui::Window>>;

    /// Additional window behaviors which may be platform specific. This property will be nil on unsupported platforms,
    /// otherwise will provide a mutable object for window behavior customization.
    #[objc::msg_send(windowingBehaviors)]
    pub fn windowing_behaviors(&self) -> Option<arc::R<ui::SceneWindowingBehaviors>>;

    #[objc::msg_send(isFullScreen)]
    pub fn is_full_screen(&self) -> bool;
}

#[objc::protocol(UIWindowSceneDelegate)]
pub trait WindowSceneDelegate: ui::SceneDelegate {
    #[objc::optional]
    #[objc::msg_send(window)]
    fn window(&self) -> Option<&ui::Window>;

    #[objc::optional]
    #[objc::msg_send(setWindow:)]
    fn set_window(&mut self, val: Option<&ui::Window>);

    #[objc::optional]
    #[objc::msg_send(windowScene:didUpdateCoordinateSpace:interfaceOrientation:traitCollection:)]
    fn window_scene_did_update_traits(
        &mut self,
        window_scene: &WindowScene,
        prev_coordinate_space: &ui::AnyCoordinateSpace,
        prev_orientation: ui::Orientation,
        prev_traits: &ui::TraitCollection,
    );
}

define_obj_type!(
    #[doc(alias = "UISceneSizeRestrictions")]
    pub SceneSizeRestrictions(ns::Id)
);

impl SceneSizeRestrictions {
    #[objc::msg_send(minimumSize)]
    pub fn min_size(&self) -> cg::Size;

    #[objc::msg_send(setMinimumSize:)]
    pub fn set_min_size(&mut self, val: cg::Size);

    #[objc::msg_send(maximumSize)]
    pub fn max_size(&self) -> cg::Size;

    #[objc::msg_send(setMaximumSize:)]
    pub fn set_max_size(&mut self, val: cg::Size);

    #[objc::msg_send(allowsFullScreen)]
    pub fn allows_full_screen(&self) -> bool;

    #[objc::msg_send(setAllowsFullScreen:)]
    pub fn set_allows_full_screen(&mut self, val: bool);
}

impl ui::SceneSessionRole {
    pub fn app() -> &'static Self {
        unsafe { UIWindowSceneSessionRoleApplication }
    }

    pub fn external_display_non_interactive() -> &'static Self {
        unsafe { UIWindowSceneSessionRoleExternalDisplayNonInteractive }
    }
}
unsafe extern "C" {
    static UIWindowSceneSessionRoleApplication: &'static ui::SceneSessionRole;
    static UIWindowSceneSessionRoleExternalDisplayNonInteractive: &'static ui::SceneSessionRole;
}
