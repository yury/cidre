use crate::{ar, arc, define_obj_type, ns, objc, simd};

define_obj_type!(
    #[doc(alias = "ARRaycastResult")]
    /// Result of a raycast against a single target.
    pub RaycastResult(ns::Id)
);

impl RaycastResult {
    /// Result transform in world coordinates.
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
