use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(Timer(ns::Id), NS_TIMER);

impl Timer {
    #[objc::msg_send(timeInterval)]
    pub fn time_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(invalidate)]
    pub fn invalidate(&self);

    #[objc::msg_send(isValid)]
    pub fn is_valid(&self) -> bool;

    #[objc::msg_send(fire)]
    pub fn fire(&self);
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_TIMER: &'static objc::Class<Timer>;
}
