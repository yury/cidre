use crate::{arc, define_obj_type, ns, objc, ui};

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

    /// The array of all windows associated with this ui::WindowScene
    #[objc::msg_send(windows)]
    pub fn windows(&self) -> arc::R<ns::Array<ui::Window>>;

    #[objc::msg_send(keyWindow)]
    pub fn key_window(&self) -> Option<arc::R<ui::Window>>;

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
