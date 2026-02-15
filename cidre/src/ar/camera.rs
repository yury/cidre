use crate::{ar, define_obj_type, ns, objc, simd};

define_obj_type!(
    #[doc(alias = "ARCamera")]
    /// Camera model and parameters for an `ar::Frame`.
    pub Camera(ns::Id)
);

impl Camera {
    /// Camera transform (rotation + translation) in world coordinates.
    #[objc::msg_send(transform)]
    pub fn transform(&self) -> simd::f32x4x4;

    /// Camera orientation as Euler angles (pitch, yaw, roll in radians).
    #[objc::msg_send(eulerAngles)]
    pub fn euler_angles(&self) -> simd::f32x3;

    /// Current camera tracking state.
    #[objc::msg_send(trackingState)]
    pub fn tracking_state(&self) -> ar::TrackingState;

    /// Reason for limited tracking state.
    #[objc::msg_send(trackingStateReason)]
    pub fn tracking_state_reason(&self) -> ar::TrackingStateReason;

    /// Camera intrinsics matrix:
    ///
    /// ```text
    /// fx 0  px
    /// 0  fy py
    /// 0  0  1
    /// ```
    #[objc::msg_send(intrinsics)]
    pub fn intrinsics(&self) -> simd::f32x3x3;

    /// Exposure duration in seconds.
    #[objc::msg_send(exposureDuration)]
    #[objc::available(ios = 13.0)]
    pub fn exposure_duration(&self) -> ns::TimeInterval;

    /// Exposure offset in EV units.
    #[objc::msg_send(exposureOffset)]
    #[objc::available(ios = 13.0)]
    pub fn exposure_offset(&self) -> f32;

    /// Projection matrix for the camera (without far clipping limit).
    #[objc::msg_send(projectionMatrix)]
    pub fn projection_matrix(&self) -> simd::f32x4x4;
}
