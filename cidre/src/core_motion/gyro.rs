use crate::{core_motion as cm, define_obj_type, objc};

#[doc(alias = "CMRotationRate")]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct RotationRate {
    x: f64,
    y: f64,
    z: f64,
}

define_obj_type!(
    #[doc(alias = "CMGyroData")]
    pub GyroData(cm::LogItem)
);

impl GyroData {
    #[objc::msg_send(rotationRate)]
    pub fn rotation_rate(&self) -> cm::RotationRate;
}
