use crate::{arc, cm, ns, objc};

impl ns::Value {
    #[objc::cls_msg_send(valueWithCMTimeRange:)]
    pub fn with_cm_time_range_ar(range: &cm::TimeRange) -> &'ar ns::Value;

    #[objc::cls_rar_retain]
    pub fn with_cm_time_range(range: &cm::TimeRange) -> arc::R<ns::Value>;
}
