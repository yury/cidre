use crate::{ar, arc, define_cls, define_obj_type, ns, objc, simd};

#[doc(alias = "ARRaycastTarget")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum Target {
    /// Intersect already detected planes using their estimated geometry.
    ExistingPlaneGeometry,
    /// Intersect already detected planes as infinite planes.
    ExistingPlaneInfinite,
    /// Intersect estimated planes around ray feature points.
    EstimatedPlane,
}

#[doc(alias = "ARRaycastTargetAlignment")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum TargetAlignment {
    /// Horizontal target.
    Horizontal,
    /// Vertical target.
    Vertical,
    /// Any target alignment.
    Any,
}

define_obj_type!(
    #[doc(alias = "ARRaycastQuery")]
    /// Ray representation and target constraints used for raycasting.
    pub RaycastQuery(ns::Id)
);

impl arc::A<RaycastQuery> {
    /// Creates a query from ray origin/direction, allowed target, and alignment.
    #[objc::msg_send(initWithOrigin:direction:allowingTarget:alignment:)]
    pub fn init_with_origin_direction_allowing_target_alignment(
        self,
        origin: simd::f32x3,
        direction: simd::f32x3,
        target: Target,
        alignment: TargetAlignment,
    ) -> arc::R<RaycastQuery>;
}

impl RaycastQuery {
    define_cls!(AR_RAYCAST_QUERY);

    /// Creates a query from ray origin/direction, allowed target, and alignment.
    #[inline]
    pub fn with_origin_direction_target_alignment(
        origin: simd::f32x3,
        direction: simd::f32x3,
        target: Target,
        alignment: TargetAlignment,
    ) -> arc::R<Self> {
        Self::alloc().init_with_origin_direction_allowing_target_alignment(
            origin, direction, target, alignment,
        )
    }

    /// Origin of the ray.
    #[objc::msg_send(origin)]
    pub fn origin(&self) -> simd::f32x3;

    /// Direction of the ray.
    #[objc::msg_send(direction)]
    pub fn direction(&self) -> simd::f32x3;

    /// Type of target where the ray should terminate.
    #[objc::msg_send(target)]
    pub fn target(&self) -> Target;

    /// Alignment considered during raycasting.
    #[objc::msg_send(targetAlignment)]
    pub fn target_alignment(&self) -> TargetAlignment;
}

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
    pub fn target(&self) -> Target;

    /// Alignment of the hit target.
    #[objc::msg_send(targetAlignment)]
    pub fn target_alignment(&self) -> TargetAlignment;

    /// Intersected anchor, when available.
    ///
    /// Existing-plane targets always provide an anchor.
    #[objc::msg_send(anchor)]
    pub fn anchor(&self) -> Option<arc::R<ar::Anchor>>;
}

define_obj_type!(
    #[doc(alias = "ARTrackedRaycast")]
    /// Continuously updated raycast that can be stopped.
    pub TrackedRaycast(ns::Id)
);

impl TrackedRaycast {
    /// Stops tracked raycasting updates.
    #[objc::msg_send(stopTracking)]
    pub fn stop_tracking(&mut self);
}

#[link(name = "ar", kind = "static")]
unsafe extern "C" {
    static AR_RAYCAST_QUERY: &'static objc::Class<RaycastQuery>;
}
