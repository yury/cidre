#[doc(alias = "ARTrackingState")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum TrackingState {
    /// Tracking is not available.
    NotAvailable,
    /// Tracking is limited; inspect `TrackingStateReason`.
    Limited,
    /// Tracking is normal.
    Normal,
}

#[doc(alias = "ARTrackingStateReason")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(isize)]
pub enum TrackingStateReason {
    /// Tracking is not limited.
    None,
    /// Tracking is limited while initializing.
    Initializing,
    /// Tracking is limited due to excessive camera motion.
    ExcessiveMotion,
    /// Tracking is limited due to insufficient visible features.
    InsufficientFeatures,
    /// Tracking is limited while relocalizing.
    Relocalizing,
}
