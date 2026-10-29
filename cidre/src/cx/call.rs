use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    /// A call the system knows of: a phone call, or a VoIP app's.
    #[doc(alias = "CXCall")]
    pub Call(ns::Id)
);

impl Call {
    #[objc::msg_send(UUID)]
    pub fn uuid(&self) -> arc::R<ns::Uuid>;

    #[objc::msg_send(isOutgoing)]
    pub fn is_outgoing(&self) -> bool;

    #[objc::msg_send(isOnHold)]
    pub fn is_on_hold(&self) -> bool;

    #[objc::msg_send(hasConnected)]
    pub fn has_connected(&self) -> bool;

    #[objc::msg_send(hasEnded)]
    pub fn has_ended(&self) -> bool;
}
