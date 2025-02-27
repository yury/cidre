#[cfg(feature = "cm")]
use crate::{arc, cm, ns, objc};

/// NSValueAVFoundationExtensions
#[cfg(feature = "cm")]
impl ns::Value {
    #[objc::msg_send(valueWithCMTime:)]
    #[objc::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn with_cm_time(range: &cm::Time) -> arc::R<ns::Value>;

    #[objc::msg_send(CMTimeValue)]
    #[objc::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn cm_time_value(&self) -> cm::Time;

    #[objc::msg_send(valueWithCMTimeRange:)]
    #[objc::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn with_cm_time_range(range: &cm::TimeRange) -> arc::R<ns::Value>;

    #[objc::msg_send(CMTimeRangeValue)]
    #[objc::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn cm_time_range_value(&self) -> cm::TimeRange;

    #[objc::msg_send(valueWithCMTimeMapping:)]
    #[objc::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn with_cm_time_mapping(range: &cm::TimeMapping) -> arc::R<ns::Value>;

    #[objc::msg_send(CMTimeMapping)]
    #[objc::available(macos = 10.7, ios = 4.0, tvos = 9.0, watchos = 1.0, visionos = 1.0)]
    pub fn cm_time_mapping_value(&self) -> cm::TimeMapping;
}
