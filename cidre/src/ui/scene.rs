use crate::{define_cls, define_obj_type, ns, objc, ui};

define_obj_type!(
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
}

define_obj_type!(
    pub AnySceneDelegate(ns::Id)
);

impl SceneDelegate for AnySceneDelegate {}

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_SCENE: &'static objc::Class<Scene>;
}
