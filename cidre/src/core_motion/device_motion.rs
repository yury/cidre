use crate::{core_motion as cm, define_obj_type, objc};

#[doc(alias = "CMDeviceMotionSensorLocation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum SensorLocation {
    Default,
    HeadphoneLeft,
    HeadphoneRight,
}

#[doc(alias = "CMMagneticFieldCalibrationAccuracy")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum MagneticFieldCalibrationAccuracy {
    Uncalibrated = -1,
    Low,
    Medium,
    High,
}

#[repr(C)]
pub struct CalibratedMagneticField {
    field: cm::MagneticField,
    accuracy: cm::MagneticFieldCalibrationAccuracy,
}

define_obj_type!(
    #[doc(alias = "CMDeviceMotion")]
    pub DeviceMotion(cm::LogItem)
);

impl DeviceMotion {
    #[objc::msg_send(attitude)]
    pub fn attitude(&self) -> &cm::Attitude;

    #[objc::msg_send(rotationRate)]
    pub fn rotation_rate(&self) -> cm::RotationRate;

    #[objc::msg_send(gravity)]
    pub fn gravity(&self) -> cm::Acceleration;

    #[objc::msg_send(userAcceleration)]
    pub fn user_acceleration(&self) -> cm::Acceleration;

    #[objc::msg_send(magneticField)]
    pub fn magnetic_field(&self) -> cm::CalibratedMagneticField;

    #[objc::msg_send(heading)]
    pub fn heading(&self) -> f64;

    #[objc::msg_send(sensorLocation)]
    pub fn sensor_location(&self) -> cm::DeviceMotionSensorLocation;
}
