use crate::{define_obj_type, ns, objc};

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
