#[cfg(feature = "ui")]
use crate::ui;
use crate::{ar, cg, define_obj_type, ns, objc, simd};

define_obj_type!(
    #[doc(alias = "ARCamera")]
    /// Camera model and parameters for an `ar::Frame`.
    pub Camera(ns::Id)
);

unsafe impl Send for Camera {}
unsafe impl Sync for Camera {}

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

    /// Camera image resolution in pixels.
    #[objc::msg_send(imageResolution)]
    pub fn image_resolution(&self) -> cg::Size;

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

    /// Projection matrix configured for orientation, viewport, and clipping planes.
    #[cfg(feature = "ui")]
    #[objc::msg_send(projectionMatrixForOrientation:viewportSize:zNear:zFar:)]
    pub fn projection_matrix_for_orientation(
        &self,
        orientation: ui::Orientation,
        viewport_size: cg::Size,
        z_near: cg::Float,
        z_far: cg::Float,
    ) -> simd::f32x4x4;

    /// Projects a 3D world point into 2D viewport coordinates.
    #[cfg(feature = "ui")]
    #[objc::msg_send(projectPoint:orientation:viewportSize:)]
    pub fn project_point(
        &self,
        point: simd::f32x3,
        orientation: ui::Orientation,
        viewport_size: cg::Size,
    ) -> cg::Point;

    /// Unprojects a 2D viewport point onto a 3D plane in world coordinates.
    #[cfg(feature = "ui")]
    #[objc::msg_send(unprojectPoint:ontoPlaneWithTransform:orientation:viewportSize:)]
    #[objc::available(ios = 12.0)]
    pub fn unproject_point_onto_plane_with_transform(
        &self,
        point: cg::Point,
        plane_transform: simd::f32x4x4,
        orientation: ui::Orientation,
        viewport_size: cg::Size,
    ) -> simd::f32x3;

    /// View matrix for rendering with a given interface orientation.
    #[cfg(feature = "ui")]
    #[objc::msg_send(viewMatrixForOrientation:)]
    pub fn view_matrix_for_orientation(&self, orientation: ui::Orientation) -> simd::f32x4x4;
}
