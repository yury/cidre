use crate::ns;

impl ns::ErrorDomain {
    pub fn av_kit() -> &'static Self {
        unsafe { AVKitErrorDomain }
    }
}

#[link(name = "AVKit", kind = "framework")]
extern "C" {
    static AVKitErrorDomain: &'static ns::ErrorDomain;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum Code {
    /// Unknown error.
    #[doc(alias = "AVKitErrorUnknown")]
    Unknown = -1000,

    /// Failed to start Picture in Picture.
    #[doc(alias = "AVKitErrorPictureInPictureStartFailed")]
    PipStartFailed = -1001,

    /// Media content rating missing or unrecognized.
    #[doc(alias = "AVKitErrorContentRatingUnknown")]
    ContentRatingUnknown = -1100,

    /// Restriction settings disallow access to this content, but the user can override by entering the passcode.
    #[doc(alias = "AVKitErrorContentDisallowedByPasscode")]
    ContentDisallowedByPasscode = -1101,

    /// An installed profile sets restriction settings that disallow access to this content;
    /// the user cannot override by entering the passcode (they may be able to override in Settings).
    #[doc(alias = "AVKitErrorContentDisallowedByProfile")]
    ContentDisallowedByProfile = -1102,

    /// The recording failed.
    #[doc(alias = "AVKitErrorRecordingFailed")]
    RecordingFailed = -1200,
}
