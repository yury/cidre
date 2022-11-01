use crate::{define_obj_type, ns, vn};

#[repr(usize)]
pub enum Level {
    Accurate,
    Fast,
}

define_obj_type!(TrackingRequest(vn::ImageBasedRequest));

impl TrackingRequest {
    /// Tracking level allows tuning tracking algorithm to prefer speed (vn::RequestTrackingLevel::Fast)
    /// vs. tracking object location
    #[inline]
    pub fn tracking_level(&self) -> Level {
        unsafe { rsel_trackingLevel(self) }
    }

    #[inline]
    pub fn set_tracking_level(&mut self, value: Level) {
        unsafe { wsel_setTrackingLevel(self, value) }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {
    fn rsel_trackingLevel(id: &ns::Id) -> Level;
    fn wsel_setTrackingLevel(id: &mut ns::Id, value: Level);
}
