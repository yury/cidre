use crate::ns;

#[doc(alias = "ARErrorDomain")]
/// ARKit error domain.
#[inline]
pub fn domain() -> &'static ns::ErrorDomain {
    unsafe { ARErrorDomain }
}

impl ns::ErrorDomain {
    #[doc(alias = "ARErrorDomain")]
    #[inline]
    pub fn ar_kit() -> &'static Self {
        unsafe { ARErrorDomain }
    }
}

#[doc(alias = "ARErrorCode")]
/// ARKit error codes.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum Code {
    /// The provided configuration is unsupported.
    UnsupportedConfiguration = 100,
    /// A required sensor is unavailable.
    SensorUnavailable = 101,
    /// A required sensor failed.
    SensorFailed = 102,
    /// Camera authorization was denied.
    CameraUnauthorized = 103,
    /// Microphone authorization was denied.
    MicrophoneUnauthorized = 104,
    /// Location authorization was denied.
    LocationUnauthorized = 105,
    /// High-resolution frame capture is already in progress.
    HighResolutionFrameCaptureInProgress = 106,
    /// High-resolution frame capture failed.
    HighResolutionFrameCaptureFailed = 107,

    /// World tracking failed.
    WorldTrackingFailed = 200,
    /// Geo tracking is unavailable at the current location.
    GeoTrackingNotAvailableAtLocation = 201,
    /// Geo tracking failed.
    GeoTrackingFailed = 202,

    /// Provided reference image is invalid.
    InvalidReferenceImage = 300,
    /// Provided reference object is invalid.
    InvalidReferenceObject = 301,
    /// Provided world map is invalid.
    InvalidWorldMap = 302,
    /// Provided configuration is invalid.
    InvalidConfiguration = 303,
    /// Provided collaboration data is invalid.
    InvalidCollaborationData = 304,

    /// Not enough visual features were available.
    InsufficientFeatures = 400,
    /// Object merge operation failed.
    ObjectMergeFailed = 401,

    /// File I/O failed.
    FileIoFailed = 500,
    /// General request failure.
    RequestFailed = 501,
}

impl PartialEq<isize> for Code {
    #[inline]
    fn eq(&self, other: &isize) -> bool {
        *self as isize == *other
    }
}

#[link(name = "ARKit", kind = "framework")]
unsafe extern "C" {
    static ARErrorDomain: &'static ns::ErrorDomain;
}
