use crate::ns;

pub mod code {
    use crate::ns;

    /// The user chose not to authorize capture
    #[doc(alias = "SCStreamErrorUserDeclined")]
    pub const USER_DECLINED: ns::Integer = -3801;

    /// The stream failed to start
    #[doc(alias = "SCStreamErrorFailedToStart")]
    pub const FAILED_TO_START: ns::Integer = -3802;

    /// The stream failed due to missing entitlements
    #[doc(alias = "SCStreamErrorMissingEntitlements")]
    pub const MISSING_ENTITLEMENTS: ns::Integer = -3803;

    /// Failed during recording application connection invalid
    #[doc(alias = "SCStreamErrorFailedApplicationConnectionInvalid")]
    pub const FAILED_APPLICATION_CONNECTION_INVALID: ns::Integer = -3804;

    /// Failed during recording appliation connection interrupted
    #[doc(alias = "SCStreamErrorFailedApplicationConnectionInterrupted")]
    pub const FAILED_APPLICATION_CONNECTION_INTERRUPTED: ns::Integer = -3805;

    /// Failed during recording context id does not match application
    #[doc(alias = "SCStreamErrorFailedNoMatchingApplicationContext")]
    pub const FAILED_NO_MATCHING_APPLICATION_CONTEXT: ns::Integer = -3806;

    /// Failed due to attempting to start a stream thats already in a recording state
    #[doc(alias = "SCStreamErrorAttemptToStartStreamState")]
    pub const ATTEMPT_TO_START_STREAM_STATE: ns::Integer = -3807;

    /// Failed due to attempting to stop a stream thats already in a recording state
    #[doc(alias = "SCStreamErrorAttemptToStopStreamState")]
    pub const ATTEMPT_TO_STOP_STREAM_STATE: ns::Integer = -3808;

    /// Failed due to attempting to update the filter on a stream
    #[doc(alias = "SCStreamErrorAttemptToUpdateFilterState")]
    pub const ATTEMPT_TO_UPDATE_FILTER_STATE: ns::Integer = -3809;

    /// Failed due to attempting to update stream config on a stream
    #[doc(alias = "SCStreamErrorAttemptToConfigState")]
    pub const ATTEMPT_TO_CONFIG_STATE: ns::Integer = -3810;

    /// Failed to start due to video/audio capture failure
    #[doc(alias = "SCStreamErrorInternalError")]
    pub const INTERNAL_ERR: ns::Integer = -3811;

    /// Failed due to invalid parameter
    #[doc(alias = "SCStreamErrorInvalidParameter")]
    pub const INVALID_PARAM: ns::Integer = -3812;

    /// Failed due to no window list
    #[doc(alias = "SCStreamErrorNoWindowList")]
    pub const NO_WINDOW_LIST: ns::Integer = -3813;

    /// Failed due to no display list
    #[doc(alias = "SCStreamErrorNoDisplayList")]
    pub const NO_DISPLAY_LIST: ns::Integer = -3814;

    /// Failed due to no display or window list to capture
    #[doc(alias = "SCStreamErrorNoCaptureSource")]
    pub const NO_CAPTURE_SOURCE: ns::Integer = -3815;

    /// Failed to remove stream
    #[doc(alias = "SCStreamErrorRemovingStream")]
    pub const REMOVING_STREAM: ns::Integer = -3816;

    /// The stream was stopped by the user
    #[doc(alias = "SCStreamErrorUserStopped")]
    pub const USER_STOPPED: ns::Integer = -3817;

    /// The stream failed to start audio
    #[doc(alias = "SCStreamErrorFailedToStartAudioCapture")]
    pub const FAILED_TO_START_AUDIO_CAPTURE: ns::Integer = -3818;

    /// The stream failed to stop audio
    #[doc(alias = "SCStreamErrorFailedToStopAudioCapture")]
    pub const FAILED_TO_STOP_AUDIO_CAPTURE: ns::Integer = -3819;

    /// The stream failed to start microphone
    #[doc(alias = "SCStreamErrorFailedToStartMicrophoneCapture")]
    pub const FAILED_TO_START_MIC_CAPTURE: ns::Integer = -3820;

    /// The stream was stopped by the system
    #[doc(alias = "SCStreamErrorSystemStoppedStream")]
    pub const SYSTEM_STOPPED_STREAM: ns::Integer = -3821;
}

#[doc(alias = "SCStreamErrorDomain")]
pub fn domain() -> &'static ns::ErrorDomain {
    unsafe { SCStreamErrorDomain }
}

unsafe extern "C" {
    static SCStreamErrorDomain: &'static ns::ErrorDomain;
}

impl ns::ErrorDomain {
    #[doc(alias = "SCStreamErrorDomain")]
    pub fn screen_capture() -> &'static Self {
        unsafe { SCStreamErrorDomain }
    }
}
