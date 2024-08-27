use crate::{api, arc, cm, ns, objc};

/// NSValueAVFoundationExtensions
impl ns::Value {
    #[objc::msg_send(valueWithCMTime:)]
    #[api::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn with_cm_time(range: &cm::Time) -> arc::R<ns::Value>;

    #[objc::msg_send(CMTimeValue)]
    #[api::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn cm_time_value(&self) -> cm::Time;

    #[objc::msg_send(valueWithCMTimeRange:)]
    #[api::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn with_cm_time_range(range: &cm::TimeRange) -> arc::R<ns::Value>;

    #[objc::msg_send(CMTimeRangeValue)]
    #[api::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn cm_time_range_value(&self) -> cm::TimeRange;

    #[objc::msg_send(valueWithCMTimeMapping:)]
    #[api::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn with_cm_time_mapping(range: &cm::TimeMapping) -> arc::R<ns::Value>;

    #[objc::msg_send(CMTimeMapping)]
    #[api::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn cm_time_mapping_value(&self) -> cm::TimeMapping;
}
