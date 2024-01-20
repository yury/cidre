use crate::{core_motion as cm, define_obj_type, objc};

#[doc(alias = "CMRotationRate")]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct RotationRate {
    /// X-axis rotation rate in radians/second. The sign follows the right hand
    /// rule (i.e. if the right hand is wrapped around the X axis such that the
    /// tip of the thumb points toward positive X, a positive rotation is one
    /// toward the tips of the other 4 fingers).
    pub x: f64,

    /// Y-axis rotation rate in radians/second. The sign follows the right hand
    /// rule (i.e. if the right hand is wrapped around the Y axis such that the
    /// tip of the thumb points toward positive Y, a positive rotation is one
    /// toward the tips of the other 4 fingers).
    pub y: f64,

    /// Z-axis rotation rate in radians/second. The sign follows the right hand
    /// rule (i.e. if the right hand is wrapped around the Z axis such that the
    /// tip of the thumb points toward positive Z, a positive rotation is one
    /// toward the tips of the other 4 fingers).
    pub z: f64,
}

define_obj_type!(
    /// Contains a single gyro measurement.
    #[doc(alias = "CMGyroData")]
    pub GyroData(cm::LogItem)
);

impl GyroData {
    /// The rotation rate as measured by the gyro.
    #[objc::msg_send(rotationRate)]
    pub fn rotation_rate(&self) -> cm::RotationRate;
}
