pub type ID = u32;

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ErrorCode(pub i64);

impl ErrorCode {
    /// Operation succeeded.
    pub const NONE: Self = Self(0);

    /// The app attempted to use the audio session during or after a Media Services failure.  App
    /// should wait for a AVAudioSessionMediaServicesWereResetNotification and then rebuild all
    /// its state.
    pub const MEDIA_SERVICES_FAILED: Self = Self(u32::from_be_bytes(*b"msrv") as _);

    /// The app attempted to set its audio session inactive or change its AVAudioSessionIOType,
    /// but it is still actively playing and/or recording.
    pub const IS_BUSY: Self = Self(u32::from_be_bytes(*b"!act") as _);

    /// The app tried to perform an operation on a session but its category does not support it.
    /// For instance, if the app calls setPreferredInputNumberOfChannels: while in a playback-only
    /// category.
    pub const INCOMPATIBLE_CATEGORY: Self = Self(u32::from_be_bytes(*b"!cat") as _);

    /// The app's audio session is non-mixable and trying to go active while in the background.
    /// This is allowed only when the app is the NowPlaying app.
    pub const CANNOT_INTERRUPT_OTHERS: Self = Self(u32::from_be_bytes(*b"!int") as _);

    /// The app does not have the required entitlements to perform an operation.
    pub const MISSING_ENTITLEMENT: Self = Self(u32::from_be_bytes(*b"ent?") as _);

    /// The app tried to do something with the audio session that is not allowed while Siri is
    /// recording.
    pub const SIRI_IS_RECORDING: Self = Self(u32::from_be_bytes(*b"siri") as _);

    /// The app is not allowed to start recording and/or playing, usually because of a lack of audio
    /// key in its Info.plist.  This could also happen if the app has this key but uses a category
    /// that can't record and/or play in the background (AVAudioSessionCategoryAmbient,
    /// AVAudioSessionCategorySoloAmbient, etc.).
    pub const CANNOT_START_PLAYING: Self = Self(u32::from_be_bytes(*b"!pla") as _);

    /// The app is not allowed to start recording, usually because it is starting a mixable
    /// recording from the background and is not an Inter-App Audio app.
    pub const CANNOT_START_RECORDING: Self = Self(u32::from_be_bytes(*b"!rec") as _);

    /// An illegal value was used for a property.
    pub const BAD_PARAM: Self = Self(-50);

    /// The app was not allowed to set the audio category because another app (Phone, etc.) is
    /// controlling it.
    pub const INSUFFICIENT_PRIORITY: Self = Self(u32::from_be_bytes(*b"!pri") as _);

    /// The operation failed because the device does not have sufficient hardware resources to
    /// complete the action. For example, the operation requires audio input hardware, but the
    /// device has no audio input available.
    pub const RESOURCE_NOT_AVAILABLE: Self = Self(u32::from_be_bytes(*b"!res") as _);

    /// An unspecified error has occurred.
    pub const UNSPECIFIED: Self = Self(u32::from_be_bytes(*b"what") as _);

    /// The operation failed because the associated session has been destroyed.
    pub const EXPIRED_SESSION: Self = Self(u32::from_be_bytes(*b"!ses") as _);

    /// The operation failed because the session is not active.
    pub const SESSION_NOT_ACTIVE: Self = Self(u32::from_be_bytes(*b"inac") as _);
}
