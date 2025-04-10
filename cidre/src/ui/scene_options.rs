use crate::{arc, define_obj_type, ns, objc, ui};

define_obj_type!(
    /// This object is vended to your application by UIKit when a ui::Scene connects to a session
    #[doc(alias = "UISceneConnectionOptions")]
    pub SceneConnectionOpts(ns::Id)
);

impl SceneConnectionOpts {
    /// A NSString containing the bundle ID of the originating application.
    /// non-nil if the originating application and this application share the same team identifier.
    #[objc::msg_send(sourceApplication)]
    pub fn src_app(&self) -> Option<arc::R<ns::String>>;

    /// The type of a handoff user activity if one is pending on connect.
    /// The delegate callbacks will be invoked for this activity when it is available.
    #[objc::msg_send(handoffUserActivityType)]
    pub fn handoff_user_activity_type(&self) -> Option<arc::R<ns::String>>;
}

#[doc(alias = "UISceneCollectionJoinBehavior")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum SceneCollectionJoinBehavior {
    /// The scene follows system settings for joining collections.
    Automatic,

    /// If requestingScene is set, add the new scene to its collection and activate it.
    /// Otherwise attempt to join a compatible collection.
    Preferred,

    /// Create a new collection for the scene, ignoring system settings.
    Disallowed,

    /// If requestingScene is set, add the new scene without deactivating the requestingScene. Otherwise behaves the same as preferred.
    /// For example, in Catalyst this could be used to open a link in a new tab in the background.
    PreferredWithoutActivating,
}

define_obj_type!(
    #[doc(alias = "UISceneActivationRequestOptions")]
    pub SceneActivationRequestOpts(ns::Id)
);

impl SceneActivationRequestOpts {
    #[objc::msg_send(requestingScene)]
    pub fn requesting_scene(&self) -> Option<arc::R<ui::Scene>>;

    /// Informs the system the interface instance the user interacted with to create the new interface for the purposes of system navigation.
    #[objc::msg_send(setRequestingScene:)]
    pub fn set_requesting_scene(&mut self, val: Option<&ui::Scene>);

    #[objc::msg_send(collectionJoinBehavior)]
    #[objc::available(maccatalyst = 14.0)]
    pub fn collection_join_behavior(&self) -> SceneCollectionJoinBehavior;

    /// A scene collection is a group of scenes that display together. In Catalyst, this is used to add windows to an NSWindowTabGroup.
    #[objc::msg_send(setCollectionJoinBehavior:)]
    #[objc::available(maccatalyst = 14.0)]
    pub fn set_collection_join_behavior(&mut self, val: SceneCollectionJoinBehavior);
}

define_obj_type!(
    #[doc(alias = "UISceneDestructionRequestOptions")]
    pub SceneDestructionRequestOpts(ns::Id)
);
