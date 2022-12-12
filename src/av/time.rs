use crate::{cf, cm, ns};

impl ns::Value {
    pub fn with_cm_time_range(range: &cm::TimeRange) -> cf::Retained<ns::Value> {
        unsafe { NSValue_valueWithCMTimeRange(*range) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn NSValue_valueWithCMTimeRange(range: cm::TimeRange) -> cf::Retained<ns::Value>;
}
