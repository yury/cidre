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
        scene: &mut Scene,
        session: &ui::SceneSession,
        options: &ui::SceneConnectionOpts,
    );

    #[objc::optional]
    #[objc::msg_send(sceneDidBecomeActive:)]
    fn scene_did_become_active(&mut self, scene: &mut Scene);

    #[objc::optional]
    #[objc::msg_send(sceneWillResignActive:)]
    fn scene_will_resign_active(&mut self, scene: &mut Scene);

    #[objc::optional]
    #[objc::msg_send(sceneDidEnterBackground:)]
    fn scene_will_enter_foreground(&mut self, scene: &mut Scene);
}

define_obj_type!(
    pub AnySceneDelegate(ns::Id)
);

impl SceneDelegate for AnySceneDelegate {}

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_SCENE: &'static objc::Class<Scene>;
}

pub mod notifications {
    use crate::ns;

    #[doc(alias = "UISceneWillConnectNotification")]
    pub fn will_connect() -> &'static ns::NotificationName {
        unsafe { UISceneWillConnectNotification }
    }

    #[doc(alias = "UISceneDidDisconnectNotification")]
    pub fn did_disconnect() -> &'static ns::NotificationName {
        unsafe { UISceneDidDisconnectNotification }
    }

    #[doc(alias = "UISceneDidActivateNotification")]
    pub fn did_activate() -> &'static ns::NotificationName {
        unsafe { UISceneDidActivateNotification }
    }

    #[doc(alias = "UISceneWillDeactivateNotification")]
    pub fn will_deactivate() -> &'static ns::NotificationName {
        unsafe { UISceneWillDeactivateNotification }
    }

    #[doc(alias = "UISceneWillEnterForegroundNotification")]
    pub fn will_enter_foreground() -> &'static ns::NotificationName {
        unsafe { UISceneWillEnterForegroundNotification }
    }

    #[doc(alias = "UISceneDidEnterBackgroundNotification")]
    pub fn did_enter_background() -> &'static ns::NotificationName {
        unsafe { UISceneDidEnterBackgroundNotification }
    }

    unsafe extern "C" {
        static UISceneWillConnectNotification: &'static ns::NotificationName;
        static UISceneDidDisconnectNotification: &'static ns::NotificationName;
        static UISceneDidActivateNotification: &'static ns::NotificationName;
        static UISceneWillDeactivateNotification: &'static ns::NotificationName;
        static UISceneWillEnterForegroundNotification: &'static ns::NotificationName;
        static UISceneDidEnterBackgroundNotification: &'static ns::NotificationName;
    }
}
