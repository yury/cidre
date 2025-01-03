use crate::{arc, define_obj_type, ns, objc};

#[doc(alias = "NSTimeInterval")]
pub type TimeInterval = f64;

pub const TIME_INTERVAL_SINCE_1970: TimeInterval = 978307200.0f64;

define_obj_type!(
    #[doc(alias = "NSDate")]
    pub Date(ns::Id), NS_DATE
);

impl arc::A<Date> {
    #[objc::msg_send(initWithTimeIntervalSinceNow:)]
    pub fn init_with_time_interval_since_now(self, secs: ns::TimeInterval) -> arc::R<Date>;

    #[objc::msg_send(initWithTimeIntervalSince1970:)]
    pub fn init_with_time_interval_since_1970(self, secs: ns::TimeInterval) -> arc::R<Date>;
}

impl Date {
    #[inline]
    pub fn with_time_interval_since_now(secs: ns::TimeInterval) -> arc::R<Self> {
        Self::alloc().init_with_time_interval_since_now(secs)
    }

    #[inline]
    pub fn with_time_interval_since_1970(secs: ns::TimeInterval) -> arc::R<Self> {
        Self::alloc().init_with_time_interval_since_1970(secs)
    }

    #[objc::msg_send(timeIntervalSinceNow)]
    pub fn time_interval_since_now(&self) -> ns::TimeInterval;

    #[objc::msg_send(timeIntervalSince1970)]
    pub fn time_interval_since_1970(&self) -> ns::TimeInterval;

    #[objc::msg_send(distantFuture)]
    pub fn distant_future() -> arc::R<Self>;

    #[objc::msg_send(distantPast)]
    pub fn distant_past() -> arc::R<Self>;

    #[objc::msg_send(dateByAddingTimeInterval:)]
    pub fn add_time_interval(&self, val: TimeInterval) -> arc::R<Self>;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_DATE: &'static objc::Class<ns::Date>;
}

impl std::convert::TryFrom<std::time::SystemTime> for arc::R<Date> {
    type Error = std::time::SystemTimeError;

    fn try_from(value: std::time::SystemTime) -> Result<Self, Self::Error> {
        let secs = value.duration_since(std::time::UNIX_EPOCH)?.as_secs_f64();
        Ok(ns::Date::with_time_interval_since_1970(secs))
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::{arc, cf, ns};

    #[test]
    fn basics() {
        let d = ns::Date::with_time_interval_since_now(0.0);
        sleep(Duration::from_millis(10));
        assert_ne!(d.time_interval_since_now(), 0.0);
        let d = ns::Date::new();
        sleep(Duration::from_millis(10));
        assert_ne!(d.time_interval_since_now(), 0.0);

        let _d = ns::Date::with_time_interval_since_1970(0.0);
    }

    #[test]
    fn try_from() {
        let now = std::time::SystemTime::now();
        let ns_date: arc::R<ns::Date> = now.try_into().unwrap();
        let cf_date: arc::R<cf::Date> = now.try_into().unwrap();
        ns_date.is_equal(&cf_date.as_ns());
    }
}
