//! `DockKit.DockKitError` codes.
//!
//! `DockKitError` is a native Swift enum rather than an `NSError` subclass, so
//! bridging gives an [`ns::Error`](crate::ns::Error) whose domain is the
//! fully-qualified Swift type name and whose code is the case's declaration
//! index. Unlike some bridged Swift enums it does carry a real localized
//! description, so these codes are for branching rather than for display.

use crate::ns;

impl ns::ErrorDomain {
    /// The domain Swift synthesizes when bridging `DockKit.DockKitError`.
    #[doc(alias = "DockKitError")]
    pub fn dock_kit() -> &'static Self {
        let domain = crate::nsstr!(c"DockKit.DockKitError");
        unsafe { std::mem::transmute(domain) }
    }
}

#[doc(alias = "DockKitError")]
pub fn domain() -> &'static ns::ErrorDomain {
    ns::ErrorDomain::dock_kit()
}

/// `DockKitError` codes, in the cases' declaration order.
pub mod code {
    use crate::ns;

    #[doc(alias = "DockKitError.notSupported")]
    pub const NOT_SUPPORTED: ns::Integer = 0;

    #[doc(alias = "DockKitError.notConnected")]
    pub const NOT_CONNECTED: ns::Integer = 1;

    #[doc(alias = "DockKitError.notSupportedByDevice")]
    pub const NOT_SUPPORTED_BY_DEVICE: ns::Integer = 2;

    #[doc(alias = "DockKitError.invalidParameter")]
    pub const INVALID_PARAMETER: ns::Integer = 3;

    #[doc(alias = "DockKitError.noSubjectFound")]
    pub const NO_SUBJECT_FOUND: ns::Integer = 4;

    #[doc(alias = "DockKitError.frameRateTooLow")]
    pub const FRAME_RATE_TOO_LOW: ns::Integer = 5;

    #[doc(alias = "DockKitError.cameraTccMissing")]
    pub const CAMERA_TCC_MISSING: ns::Integer = 6;

    #[doc(alias = "DockKitError.frameRateTooHigh")]
    pub const FRAME_RATE_TOO_HIGH: ns::Integer = 7;
}
