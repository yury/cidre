use crate::{arc, cm, ns, objc};

impl ns::Value {
    #[objc::msg_send2(valueWithCMTimeRange:)]
    pub fn with_cm_time_range(range: &cm::TimeRange) -> arc::R<ns::Value>;
}
