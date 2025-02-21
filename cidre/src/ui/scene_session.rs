use crate::{
    arc, define_cls, define_obj_type, ns,
    objc::{self, Obj},
    ui,
};

define_obj_type!(
    #[doc(alias = "UISceneConfiguration")]
    pub SceneCfg(ns::Id)
);

impl arc::A<SceneCfg> {
    #[objc::msg_send(initWithName:sessionRole:)]
    pub fn init_with_name_role(
        self,
        name: Option<&ns::String>,
        session_role: &ui::SceneSessionRole,
    ) -> arc::R<SceneCfg>;
}

impl SceneCfg {
    define_cls!(UI_SCENE_CONFIGURATION);

    pub fn with_name_role(
        name: Option<&ns::String>,
        session_role: &ui::SceneSessionRole,
    ) -> arc::R<Self> {
        Self::alloc().init_with_name_role(name, session_role)
    }

    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<&ns::String>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(role)]
    pub fn role(&self) -> &ui::SceneSessionRole;

    #[objc::msg_send(delegateClass)]
    pub fn delegate_class(&self) -> Option<&ns::Class<ns::Id>>;

    #[objc::msg_send(setDelegateClass:)]
    pub fn set_delegate_class<T: Obj>(&mut self, val: Option<&ns::Class<T>>);

    #[objc::msg_send(userInfo)]
    pub fn user_info(&self) -> Option<&ns::Dictionary<ns::String, ns::Id>>;
}

define_obj_type!(
    #[doc(alias = "UISceneSession")]
    pub SceneSession(ns::Id)
);

impl SceneSession {
    #[objc::msg_send(scene)]
    pub fn scene(&self) -> Option<&ui::Scene>;

    #[objc::msg_send(role)]
    pub fn role(&self) -> &ui::SceneSessionRole;
}

unsafe extern "C" {
    static UI_SCENE_CONFIGURATION: &'static objc::Class<SceneCfg>;
}
