use crate::{core_motion as cm, define_obj_type, objc};

define_obj_type!(
    #[doc(alias = "CMAbsoluteAltitudeData")]
    pub AbsAltitudeData(cm::LogItem)
);

impl AbsAltitudeData {
    /// The absolute altitude of the device in meters relative to sea level; could be positive or negative.
    #[objc::msg_send(altitude)]
    pub fn altitude(&self) -> f64;

    /// The accuracy of the altitude estimate, in meters.
    #[objc::msg_send(accuracy)]
    pub fn accuracy(&self) -> f64;

    /// The precision of the altitude estimate, in meters.
    #[objc::msg_send(precision)]
    pub fn precision(&self) -> f64;
}
