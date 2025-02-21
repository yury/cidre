use crate::ns;

impl ns::ErrorDomain {
    pub fn multipeer_connectivity() -> &'static Self {
        unsafe { MCErrorDomain }
    }
}

#[link(name = "MultipeerConnectivity", kind = "framework")]
unsafe extern "C" {
    static MCErrorDomain: &'static ns::ErrorDomain;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum Code {
    /// An unknown error occurred.
    #[doc(alias = "MCErrorUnknown")]
    Unknown = 0,

    /// Your app attempted to send data to a peer that is not connected.
    #[doc(alias = "MCErrorNotConnected")]
    NotConnected = 1,

    /// Your app passed an invalid value as a parameter.
    #[doc(alias = "MCErrorInvalidParameter")]
    InvalidParameter = 2,

    /// The operation is unsupported.
    #[doc(alias = "MCErrorUnsupported")]
    Unsupported = 3,

    /// The connection attempt timed out.
    #[doc(alias = "MCErrorTimedOut")]
    TimedOut = 4,

    /// The operation was cancelled by the user.
    #[doc(alias = "MCErrorCancelled")]
    Cancelled = 5,

    /// Multipeer connectivity is currently unavailable.
    #[doc(alias = "MCErrorUnavailable")]
    Unavailable = 6,
}
