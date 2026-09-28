use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSSwitch")]
    pub Switch(ns::Control),
    sym NSSwitch
);

impl Switch {
    #[objc::msg_send(state)]
    pub fn state(&self) -> ns::ControlStateValue;

    #[objc::msg_send(setState:)]
    pub fn set_state(&mut self, val: ns::ControlStateValue);
}
