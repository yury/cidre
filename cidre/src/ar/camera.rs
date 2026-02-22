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
    #[cfg(target_arch = "aarch64")]
    pub fn transform(&self) -> simd::f32x4x4 {
        let q0: std::arch::aarch64::float32x4_t;
        let q1: std::arch::aarch64::float32x4_t;
        let q2: std::arch::aarch64::float32x4_t;
        let q3: std::arch::aarch64::float32x4_t;

        unsafe {
            core::arch::asm!(
                "bl _objc_msgSend$transform",
                in("x0") self as *const Camera,
                lateout("q0") q0,
                lateout("q1") q1,
                lateout("q2") q2,
                lateout("q3") q3,
                clobber_abi("C"),
            );
        }

        simd::f32x4x4(std::arch::aarch64::float32x4x4_t(q0, q1, q2, q3))
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn transform(&self) -> simd::f32x4x4 {
        unimplemented!()
    }

    /// Camera orientation as Euler angles (pitch, yaw, roll in radians).
    #[cfg(target_arch = "aarch64")]
    pub fn euler_angles(&self) -> simd::f32x4 {
        let q0: std::arch::aarch64::float32x4_t;

        unsafe {
            core::arch::asm!(
                "bl _objc_msgSend$eulerAngles",
                in("x0") self as *const Camera,
                lateout("q0") q0,
                clobber_abi("C"),
            );
        }

        simd::f32x4(q0)
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn euler_angles(&self) -> simd::f32x4 {
        unimplemented!()
    }

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
    #[cfg(target_arch = "aarch64")]
    pub fn projection_matrix(&self) -> simd::f32x4x4 {
        let q0: std::arch::aarch64::float32x4_t;
        let q1: std::arch::aarch64::float32x4_t;
        let q2: std::arch::aarch64::float32x4_t;
        let q3: std::arch::aarch64::float32x4_t;

        unsafe {
            core::arch::asm!(
                "bl _objc_msgSend$projectionMatrix",
                in("x0") self as *const Camera,
                lateout("q0") q0,
                lateout("q1") q1,
                lateout("q2") q2,
                lateout("q3") q3,

                clobber_abi("C"),
            );
        }

        simd::f32x4x4(std::arch::aarch64::float32x4x4_t(q0, q1, q2, q3))
    }

    #[cfg(not(target_arch = "aarch64"))]
    pub fn projection_matrix(&self) -> simd::f32x4x4 {
        unimplemented!()
    }

    /// Projection matrix configured for orientation, viewport, and clipping planes.
    #[cfg(all(feature = "ui", target_arch = "aarch64"))]
    pub fn projection_matrix_for_orientation(
        &self,
        orientation: ui::Orientation,
        viewport_size: cg::Size,
        z_near: cg::Float,
        z_far: cg::Float,
    ) -> simd::f32x4x4 {
        let q0: std::arch::aarch64::float32x4_t;
        let q1: std::arch::aarch64::float32x4_t;
        let q2: std::arch::aarch64::float32x4_t;
        let q3: std::arch::aarch64::float32x4_t;

        unsafe {
            core::arch::asm!(
                "bl \"_objc_msgSend$projectionMatrixForOrientation:viewportSize:zNear:zFar:\"",
                in("x0") self as *const Camera,
                in("x2") orientation as isize,
                in("d0") viewport_size.width,
                in("d1") viewport_size.height,
                in("d2") z_near,
                in("d3") z_far,
                lateout("q0") q0,
                lateout("q1") q1,
                lateout("q2") q2,
                lateout("q3") q3,
                clobber_abi("C"),
            );
        }

        simd::f32x4x4(std::arch::aarch64::float32x4x4_t(q0, q1, q2, q3))
    }

    #[cfg(all(feature = "ui", not(target_arch = "aarch64")))]
    pub fn projection_matrix_for_orientation(
        &self,
        orientation: ui::Orientation,
        viewport_size: cg::Size,
        z_near: cg::Float,
        z_far: cg::Float,
    ) -> simd::f32x4x4 {
        let _ = (orientation, viewport_size, z_near, z_far);
        unimplemented!()
    }

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
    #[cfg(all(feature = "ui", target_arch = "aarch64"))]
    pub fn view_matrix_for_orientation(&self, orientation: ui::Orientation) -> simd::f32x4x4 {
        let q0: std::arch::aarch64::float32x4_t;
        let q1: std::arch::aarch64::float32x4_t;
        let q2: std::arch::aarch64::float32x4_t;
        let q3: std::arch::aarch64::float32x4_t;

        unsafe {
            core::arch::asm!(
                "bl \"_objc_msgSend$viewMatrixForOrientation:\"",
                in("x0") self as *const Camera,
                in("x2") orientation as isize,
                lateout("q0") q0,
                lateout("q1") q1,
                lateout("q2") q2,
                lateout("q3") q3,
                clobber_abi("C"),
            );
        }
        simd::f32x4x4(std::arch::aarch64::float32x4x4_t(q0, q1, q2, q3))
    }

    #[cfg(all(feature = "ui", not(target_arch = "aarch64")))]
    pub fn view_matrix_for_orientation(&self, orientation: ui::Orientation) -> simd::f32x4x4 {
        let _ = orientation;
        unimplemented!()
    }
}
