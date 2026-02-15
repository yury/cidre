use crate::define_opts;

define_opts!(
    #[doc(alias = "ARPlaneDetection")]
    /// Plane types to detect while world tracking is active.
    pub PlaneDetection(usize)
);

impl PlaneDetection {
    /// No plane detection is run.
    pub const NONE: Self = Self(0);
    /// Detect horizontal planes.
    pub const HORIZONTAL: Self = Self(1 << 0);
    /// Detect vertical planes.
    pub const VERTICAL: Self = Self(1 << 1);
}
