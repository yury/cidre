use crate::{define_obj_type, ns, objc};

define_obj_type!(
    /// The registration of a scene accessory with a view controller, to
    /// monitor it or unregister it.
    #[doc(alias = "UISceneAccessoryRegistration")]
    pub SceneAccessoryRegistration(ns::Id)
);

impl SceneAccessoryRegistration {
    #[objc::msg_send(isAvailable)]
    #[objc::available(ios = 27.0)]
    pub fn is_available(&self) -> bool;

    #[objc::msg_send(isEnabled)]
    #[objc::available(ios = 27.0)]
    pub fn is_enabled(&self) -> bool;

    #[objc::msg_send(setEnabled:)]
    #[objc::available(ios = 27.0)]
    pub fn set_enabled(&mut self, val: bool);
}
