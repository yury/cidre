use crate::{arc, define_obj_type, ns, objc};

pub type TimeInterval = f64;

pub const TIME_INTERVAL_SINCE_1970: TimeInterval = 978307200.0f64;

define_obj_type!(Date(ns::Id), NS_DATE);

impl arc::A<Date> {
    #[objc::msg_send(initWithTimeIntervalSinceNow:)]
    pub fn init_with_time_interval_since_now(self, secs: ns::TimeInterval) -> arc::R<Date>;
}

impl Date {
    pub fn with_time_interval_since_now(secs: ns::TimeInterval) -> arc::R<Self> {
        Self::alloc().init_with_time_interval_since_now(secs)
    }

    #[objc::msg_send(timeIntervalSinceNow)]
    pub fn time_interval_since_now(&self) -> ns::TimeInterval;

    #[objc::msg_send(timeIntervalSince1970)]
    pub fn time_interval_since_1970(&self) -> ns::TimeInterval;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_DATE: &'static objc::Class<ns::Id>;
}
