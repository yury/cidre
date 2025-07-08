use crate::{api, define_obj_type, objc, ui};

#[cfg(target_os = "ios")]
use crate::arc;

define_obj_type!(
    #[doc(alias = "UIWindowSceneActivationRequestOptions")]
    pub WindowSceneActivationRequestOpts(ui::SceneActivationRequestOpts),
    UI_WINDOW_SCENE_ACTIVATION_REQUEST_OPTIONS,
    #[api::available(ios = 15.0)]
);

#[cfg(target_os = "ios")]
impl WindowSceneActivationRequestOpts {
    #[objc::msg_send(placement)]
    pub fn placement(&self) -> Option<arc::R<ui::WindowScenePlacement>>;

    #[objc::msg_send(setPlacement:)]
    pub fn set_placement(&mut self, val: Option<&ui::WindowScenePlacement>);
}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_WINDOW_SCENE_ACTIVATION_REQUEST_OPTIONS:
        &'static objc::Class<WindowSceneActivationRequestOpts>;
}
