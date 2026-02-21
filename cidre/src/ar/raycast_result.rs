use crate::{ar, arc, define_obj_type, ns, objc, simd};

define_obj_type!(
    #[doc(alias = "ARRaycastResult")]
    /// Result of a raycast against a single target.
    pub RaycastResult(ns::Id)
);

impl RaycastResult {
    /// Result transform in world coordinates.
    #[cfg(target_arch = "aarch64")]
    pub fn world_transform(&self) -> simd::f32x4x4 {
        let mut out = std::mem::MaybeUninit::<simd::f32x4x4>::uninit();

        unsafe {
            let out_base = out.as_mut_ptr() as *mut simd::f32x4;
            let out_c0 = out_base;
            let out_c1 = out_base.add(1);
            let out_c2 = out_base.add(2);
            let out_c3 = out_base.add(3);

            core::arch::asm!(
                "bl _objc_msgSend$worldTransform",
                "str q0, [x23]",
                "str q1, [x24]",
                "str q2, [x25]",
                "str q3, [x26]",
                in("x0") self as *const RaycastResult,
                in("x23") out_c0,
                in("x24") out_c1,
                in("x25") out_c2,
                in("x26") out_c3,
                lateout("x23") _,
                lateout("x24") _,
                lateout("x25") _,
                lateout("x26") _,
                clobber_abi("C"),
            );

            out.assume_init()
        }
    }

    #[cfg(not(target_arch = "aarch64"))]
    #[objc::msg_send(worldTransform)]
    pub fn world_transform(&self) -> simd::f32x4x4;

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
