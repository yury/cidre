use crate::{core_motion as cm, define_obj_type, objc};

#[doc(alias = "CMAcceleration")]
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Acceleration {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

define_obj_type!(
    #[doc(alias = "CMAccelerometerData")]
    pub AccelerometerData(cm::LogItem)
);

impl AccelerometerData {
    #[objc::msg_send(acceleration)]
    pub fn acceleration(&self) -> cm::Acceleration;
}
