use crate::cf;

pub mod code {
    use crate::cf;

    pub const USER_DECLINED: cf::Index = -3801; // The user chose not to authorize captur
    pub const FAILED_TO_START: cf::Index = -3802; // The stream failed to start
    pub const MISSING_ENTITLEMENTS: cf::Index = -3803; // The stream failed due to missing entitlements
    pub const FAILED_APPLICATION_CONNECTION_INVALID: cf::Index = -3804; // Failed during recording application connection invalid
    pub const FAILED_APPLICATION_CONNECTION_INTERRUPTED: cf::Index = -3805; // Failed during recording appliation connection interrupted
    pub const FAILED_NO_MATCHING_APPLICATION_CONTEXT: cf::Index = -3806; // Failed during recording context id does not match application
    pub const ATTEMPT_TO_START_STREAM_STATE: cf::Index = -3807; // Failed due to attempting to start a stream thats already in a recording state
    pub const ATTEMPT_TO_STOP_STREAM_STATE: cf::Index = -3808; // Failed due to attempting to stop a stream thats already in a recording state
    pub const ATTEMPT_TO_UPDATE_FILTER_STATE: cf::Index = -3809; // Failed due to attempting to update the filter on a stream
    pub const ATTEMPT_TO_CONFIG_STATE: cf::Index = -3810; // Failed due to attempting to update stream config on a stream
    pub const INTERNAL_ERROR: cf::Index = -3811; // Failed to start due to video/audio capture failure
    pub const INVALID_PARAMETER: cf::Index = -3812; // Failed due to invalid parameter
    pub const NO_WINDOW_LIST: cf::Index = -3813; // Failed due to no window list
    pub const NO_DISPLAY_LIST: cf::Index = -3814; // Failed due to no display list
    pub const NO_CAPTURE_SOURCE: cf::Index = -3815; // Failed due to no display or window list to capture
    pub const REMOVING_STREAM: cf::Index = -3816; // Failed to remove stream
}

pub fn domain() -> &'static cf::String {
    unsafe { SCStreamErrorDomain }
}

extern "C" {
    static SCStreamErrorDomain: &'static cf::String;
}
