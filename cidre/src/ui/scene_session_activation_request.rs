use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    #[doc(alias = "UISceneSessionActivationRequest")]
    pub SceneSessionActivationRequest(ns::Id)
);

impl SceneSessionActivationRequest {
    #[objc::available(ios = 17.0, tvos = 17.0)]
    crate::define_cls!(UI_SCENE_SESSION_ACTIVATION_REQUEST);

    /// The role to request. If the request was created using `with_session()`, this reflects
    /// the role of the `session`.
    #[objc::msg_send(role)]
    pub fn role(&self) -> arc::R<ui::SceneSessionRole>;

    /// The specific scene session that should be activated, if provided when creating the request.
    #[objc::msg_send(session)]
    pub fn session(&self) -> Option<arc::R<ui::SceneSession>>;

    /// An optional user activity to send to the newly activated scene.
    ///
    /// Only sessions with a matching `role` will have their `activationConditions` evaluated
    /// against the user activity's `targetContentIdentifier`.
    #[objc::msg_send(userActivity)]
    pub fn user_activity(&self) -> Option<arc::R<ns::UserActivity>>;

    #[objc::msg_send(options)]
    pub fn opts(&self) -> Option<arc::R<ui::SceneActivationRequestOpts>>;

    #[objc::msg_send(request)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn new() -> arc::R<Self>;

    #[objc::msg_send(requestWithRole:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn with_role(role: &ui::SceneSessionRole) -> arc::R<Self>;

    #[objc::msg_send(requestWithSession:)]
    #[objc::available(ios = 17.0, tvos = 17.0)]
    pub fn with_session(role: &ui::SceneSession) -> arc::R<Self>;
}

unsafe extern "C" {
    static UI_SCENE_SESSION_ACTIVATION_REQUEST: &'static objc::Class<SceneSessionActivationRequest>;
}
