use crate::{arc, define_obj_type, ns, objc};

pub type TimeInterval = f64;

pub const TIME_INTERVAL_SINCE_1970: TimeInterval = 978307200.0f64;

define_obj_type!(
    #[doc(alias = "NSDate")]
    pub Date(ns::Id), NS_DATE
);

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

    #[objc::cls_msg_send(dateWithTimeIntervalSince1970:)]
    pub fn with_time_interval_since_1970_ar(secs: ns::TimeInterval) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_time_interval_since_1970(secs: ns::TimeInterval) -> arc::R<Self>;

    #[objc::cls_msg_send(distantFuture)]
    pub fn distant_future_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn distant_future() -> arc::R<Self>;

    #[objc::cls_msg_send(distantPast)]
    pub fn distant_past_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn distant_past() -> arc::R<Self>;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_DATE: &'static objc::Class<ns::Date>;
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::ns;

    #[test]
    fn basics() {
        let d = ns::Date::with_time_interval_since_now(0.0);
        sleep(Duration::from_millis(10));
        assert_ne!(d.time_interval_since_now(), 0.0);
        let d = ns::Date::new();
        sleep(Duration::from_millis(10));
        assert_ne!(d.time_interval_since_now(), 0.0);

        let d = ns::Date::with_time_interval_since_1970(0.0);
    }
}
