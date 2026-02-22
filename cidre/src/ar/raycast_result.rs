use crate::{ar, arc, define_obj_type, ns, objc, simd};

define_obj_type!(
    #[doc(alias = "ARRaycastResult")]
    /// Result of a raycast against a single target.
    pub RaycastResult(ns::Id)
);

impl RaycastResult {
    /// Result transform in world coordinates.
    #[doc(alias = "worldTransform")]
    #[cfg(target_arch = "aarch64")]
    pub fn world_transform(&self) -> simd::f32x4x4 {
        let q0: std::arch::aarch64::float32x4_t;
        let q1: std::arch::aarch64::float32x4_t;
        let q2: std::arch::aarch64::float32x4_t;
        let q3: std::arch::aarch64::float32x4_t;

        unsafe {
            core::arch::asm!(
                "bl _objc_msgSend$transform",
                in("x0") self as *const Self,
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
    #[objc::msg_send(worldTransform)]
    pub fn world_transform(&self) -> simd::f32x4x4 {
        unimplemented!()
    }

    /// Target type where the ray terminated.
    #[objc::msg_send(target)]
    pub fn target(&self) -> ar::RaycastTarget;

    /// Alignment of the hit target.
    #[objc::msg_send(targetAlignment)]
    pub fn target_alignment(&self) -> ar::RaycastTargetAlignment;

    /// Intersected anchor, when available.
    ///
    /// Existing-plane targets always provide an anchor.
    #[objc::msg_send(anchor)]
    pub fn anchor(&self) -> Option<arc::R<ar::Anchor>>;
}
