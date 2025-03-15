use crate::{define_cls, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UIScene")]
    pub Scene(ui::Responder)
);

impl Scene {
    define_cls!(UI_SCENE);

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<&AnySceneDelegate>;

    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: SceneDelegate>(&mut self, val: Option<&D>);
}

#[objc::protocol(UISceneDelegate)]
pub trait SceneDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(scene:willConnectToSession:options:)]
    fn scene_will_connect_to_session(
        &mut self,
        scene: &Scene,
        session: &ui::SceneSession,
        options: &ui::SceneConnectionOpts,
    );

    #[objc::optional]
    #[objc::msg_send(sceneDidBecomeActive:)]
    fn scene_did_become_active(&mut self, scene: &Scene);

    #[objc::optional]
    #[objc::msg_send(sceneWillResignActive:)]
    fn scene_will_resign_active(&mut self, scene: &Scene);

    #[objc::optional]
    #[objc::msg_send(sceneDidEnterBackground:)]
    fn scene_will_enter_foreground(&mut self, scene: &Scene);
}

define_obj_type!(
    pub AnySceneDelegate(ns::Id)
);

impl SceneDelegate for AnySceneDelegate {}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_SCENE: &'static objc::Class<Scene>;
}
