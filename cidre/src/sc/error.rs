use crate::cf;

pub mod code {
    use crate::cf;

    /// The user chose not to authorize capture
    #[doc(alias = "SCStreamErrorUserDeclined")]
    pub const USER_DECLINED: cf::Index = -3801;

    /// The stream failed to start
    #[doc(alias = "SCStreamErrorFailedToStart")]
    pub const FAILED_TO_START: cf::Index = -3802;

    /// The stream failed due to missing entitlements
    #[doc(alias = "SCStreamErrorMissingEntitlements")]
    pub const MISSING_ENTITLEMENTS: cf::Index = -3803;

    /// Failed during recording application connection invalid
    #[doc(alias = "SCStreamErrorFailedApplicationConnectionInvalid")]
    pub const FAILED_APPLICATION_CONNECTION_INVALID: cf::Index = -3804;

    /// Failed during recording appliation connection interrupted
    #[doc(alias = "SCStreamErrorFailedApplicationConnectionInterrupted")]
    pub const FAILED_APPLICATION_CONNECTION_INTERRUPTED: cf::Index = -3805;

    /// Failed during recording context id does not match application
    #[doc(alias = "SCStreamErrorFailedNoMatchingApplicationContext")]
    pub const FAILED_NO_MATCHING_APPLICATION_CONTEXT: cf::Index = -3806;

    /// Failed due to attempting to start a stream thats already in a recording state
    #[doc(alias = "SCStreamErrorAttemptToStartStreamState")]
    pub const ATTEMPT_TO_START_STREAM_STATE: cf::Index = -3807;

    /// Failed due to attempting to stop a stream thats already in a recording state
    #[doc(alias = "SCStreamErrorAttemptToStopStreamState")]
    pub const ATTEMPT_TO_STOP_STREAM_STATE: cf::Index = -3808;

    /// Failed due to attempting to update the filter on a stream
    #[doc(alias = "SCStreamErrorAttemptToUpdateFilterState")]
    pub const ATTEMPT_TO_UPDATE_FILTER_STATE: cf::Index = -3809;

    /// Failed due to attempting to update stream config on a stream
    #[doc(alias = "SCStreamErrorAttemptToConfigState")]
    pub const ATTEMPT_TO_CONFIG_STATE: cf::Index = -3810;

    /// Failed to start due to video/audio capture failure
    #[doc(alias = "SCStreamErrorInternalError")]
    pub const INTERNAL_ERROR: cf::Index = -3811;

    /// Failed due to invalid parameter
    #[doc(alias = "SCStreamErrorInvalidParameter")]
    pub const INVALID_PARAMETER: cf::Index = -3812;

    /// Failed due to no window list
    #[doc(alias = "SCStreamErrorNoWindowList")]
    pub const NO_WINDOW_LIST: cf::Index = -3813;

    /// Failed due to no display list
    #[doc(alias = "SCStreamErrorNoDisplayList")]
    pub const NO_DISPLAY_LIST: cf::Index = -3814;

    /// Failed due to no display or window list to capture
    #[doc(alias = "SCStreamErrorNoCaptureSource")]
    pub const NO_CAPTURE_SOURCE: cf::Index = -3815;

    /// Failed to remove stream
    #[doc(alias = "SCStreamErrorRemovingStream")]
    pub const REMOVING_STREAM: cf::Index = -3816;

    /// The stream was stopped by the user
    #[doc(alias = "SCStreamErrorUserStopped")]
    pub const USER_STOPPED: cf::Index = -3817;

    /// The stream failed to start audio
    #[doc(alias = "SCStreamErrorFailedToStartAudioCapture")]
    pub const FAILED_TO_START_AUDIO_CAPTURE: cf::Index = -3818;

    /// The stream failed to stop audio
    #[doc(alias = "SCStreamErrorFailedToStopAudioCapture")]
    pub const FAILED_TO_STOP_AUDIO_CAPTURE: cf::Index = -3819;

    /// The stream failed to start microphone
    #[doc(alias = "SCStreamErrorFailedToStartMicrophoneCapture")]
    pub const FAILED_TO_START_MIC_CAPTURE: cf::Index = -3820;

    /// The stream was stopped by the system
    #[doc(alias = "SCStreamErrorSystemStoppedStream")]
    pub const SYSTEM_STOPPED_STREAM: cf::Index = -3821;
}

pub fn domain() -> &'static cf::String {
    unsafe { SCStreamErrorDomain }
}

extern "C" {
    static SCStreamErrorDomain: &'static cf::String;
}
