use crate::{define_obj_type, ns, objc};

define_obj_type!(
    pub SceneWindowingBehaviors(ns::Id)
);

impl SceneWindowingBehaviors {
    #[objc::msg_send(closable)]
    pub fn is_closable(&self) -> bool;

    #[objc::msg_send(setClosable:)]
    pub fn set_closable(&mut self, val: bool);

    #[objc::msg_send(isMiniaturizable)]
    pub fn is_miniaturizable(&self) -> bool;

    #[objc::msg_send(setMiniaturizable:)]
    pub fn set_miniaturizable(&mut self, val: bool);
}
