use crate::{define_obj_type, objc, vn};

#[repr(usize)]
pub enum Level {
    Accurate,
    Fast,
}

define_obj_type!(pub TrackingRequest(vn::ImageBasedRequest));

impl TrackingRequest {
    /// Tracking level allows tuning tracking algorithm to prefer speed (vn::RequestTrackingLevel::Fast)
    /// vs. tracking object location
    #[objc::msg_send(trackingLevel)]
    pub fn tracking_level(&self) -> Level;

    #[objc::msg_send(setTrackingLevel:)]
    pub fn set_tracking_level(&mut self, value: Level);
}
