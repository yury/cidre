use crate::{api, arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    /// The configuration of a scene accessory: a scene the system presents
    /// alongside the view controller it is registered with.
    #[doc(alias = "UISceneAccessory")]
    pub SceneAccessory(ns::Id)
);

impl SceneAccessory {
    #[api::available(ios = 27.0)]
    crate::define_cls!(UI_SCENE_ACCESSORY);

    #[objc::msg_send(externalNonInteractiveSceneAccessoryWithConfiguration:)]
    #[objc::available(ios = 27.0)]
    pub fn external_non_interactive(scene_cfg: &ui::SceneCfg) -> arc::R<Self>;

    #[objc::msg_send(externalNonInteractiveSceneAccessoryWithConfiguration:userInfo:)]
    #[objc::available(ios = 27.0)]
    pub fn external_non_interactive_with_user_info(
        scene_cfg: &ui::SceneCfg,
        user_info: &ns::Id,
    ) -> arc::R<Self>;

    /// A scene that presents content during camera capture.
    #[objc::msg_send(cameraCaptureSceneAccessoryWithConfiguration:)]
    #[objc::available(ios = 27.1)]
    pub fn camera_capture(scene_cfg: &ui::SceneCfg) -> arc::R<Self>;

    #[objc::msg_send(cameraCaptureSceneAccessoryWithConfiguration:userInfo:)]
    #[objc::available(ios = 27.1)]
    pub fn camera_capture_with_user_info(
        scene_cfg: &ui::SceneCfg,
        user_info: &ns::Id,
    ) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_SCENE_ACCESSORY: &'static objc::Class<SceneAccessory>;
}
