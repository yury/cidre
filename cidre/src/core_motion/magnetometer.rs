use crate::{core_motion as cm, define_obj_type, objc};

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct MagneticField {
    ///  X-axis magnetic field in microteslas.
    pub x: f64,

    /// Y-axis magnetic field in microteslas.
    pub y: f64,

    /// Z-axis magnetic field in microteslas.
    pub z: f64,
}

define_obj_type!(
    #[doc(alias = "CMMagnetometerData")]
    pub MagnetometerData(cm::LogItem)
);

impl MagnetometerData {
    #[objc::msg_send(magneticField)]
    pub fn magnetic_field(&self) -> cm::MagneticField;
}
