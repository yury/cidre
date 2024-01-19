use crate::{define_obj_type, define_opts, ns, objc};

/// Type represents a rotation matrix.
#[doc(alias = "CMRotationMatrix")]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct RotationMatrix {
    pub m11: f64,
    pub m12: f64,
    pub m13: f64,
    pub m21: f64,
    pub m22: f64,
    pub m23: f64,
    pub m31: f64,
    pub m32: f64,
    pub m33: f64,
}

#[doc(alias = "CMQuaternion")]
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

define_opts!(
    #[doc(alias = "CMAttitudeReferenceFrame")]
    pub AttitudeRefFrame(usize)
);

impl AttitudeRefFrame {
    pub const X_ARBITRARY_Z_VERTICAL: Self = Self(1 << 0);
    pub const X_ARBITRARY_CORRECTED_Z_VERTICAL: Self = Self(1 << 1);
    pub const X_MAGNETIC_NORTH_Z_VERTICAL: Self = Self(1 << 2);
    pub const X_TRUE_NORTH_Z_VERTICAL: Self = Self(1 << 3);
}

define_obj_type!(
    #[doc(alias = "CMAttitude")]
    pub Attitude(ns::Id)
);

impl Attitude {
    /// Returns the roll of the device in radians.
    #[objc::msg_send(roll)]
    pub fn roll(&self) -> f64;

    /// Returns the pitch of the device in radians.
    #[objc::msg_send(pitch)]
    pub fn pitch(&self) -> f64;

    /// Returns the yaw of the device in radians.
    #[objc::msg_send(yaw)]
    pub fn yaw(&self) -> f64;

    #[objc::msg_send(rotationMatrix)]
    pub fn rotation_matrix(&self) -> RotationMatrix;

    #[objc::msg_send(quaternion)]
    pub fn quaternion(&self) -> Quaternion;

    #[objc::msg_send(multiplyByInverseOfAttitude:)]
    pub fn multiply_by_inverse_of_attitude(&self, attitude: &Attitude);
}
