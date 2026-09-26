//! AVFoundation error domain and codes.

use crate::ns;

impl ns::ErrorDomain {
    #[doc(alias = "AVFoundationErrorDomain")]
    pub fn av_foundation() -> &'static Self {
        unsafe { AVFoundationErrorDomain }
    }
}

#[doc(alias = "AVFoundationErrorDomain")]
pub fn domain() -> &'static ns::ErrorDomain {
    ns::ErrorDomain::av_foundation()
}

unsafe extern "C" {
    static AVFoundationErrorDomain: &'static ns::ErrorDomain;
}

/// `AVError` codes in [`domain()`](super::domain).
pub mod code {
    use crate::ns;

    #[doc(alias = "AVErrorUnknown")]
    pub const UNKNOWN: ns::Integer = -11800;

    #[doc(alias = "AVErrorOutOfMemory")]
    pub const OUT_OF_MEMORY: ns::Integer = -11801;

    #[doc(alias = "AVErrorSessionNotRunning")]
    pub const SESSION_NOT_RUNNING: ns::Integer = -11803;

    #[doc(alias = "AVErrorDeviceAlreadyUsedByAnotherSession")]
    pub const DEVICE_ALREADY_USED_BY_ANOTHER_SESSION: ns::Integer = -11804;

    #[doc(alias = "AVErrorNoDataCaptured")]
    pub const NO_DATA_CAPTURED: ns::Integer = -11805;

    #[doc(alias = "AVErrorSessionConfigurationChanged")]
    pub const SESSION_CFG_CHANGED: ns::Integer = -11806;

    #[doc(alias = "AVErrorDiskFull")]
    pub const DISK_FULL: ns::Integer = -11807;

    #[doc(alias = "AVErrorDeviceWasDisconnected")]
    pub const DEVICE_WAS_DISCONNECTED: ns::Integer = -11808;

    #[doc(alias = "AVErrorMediaChanged")]
    pub const MEDIA_CHANGED: ns::Integer = -11809;

    #[doc(alias = "AVErrorMaximumDurationReached")]
    pub const MAX_DURATION_REACHED: ns::Integer = -11810;

    #[doc(alias = "AVErrorMaximumFileSizeReached")]
    pub const MAX_FILE_SIZE_REACHED: ns::Integer = -11811;

    #[doc(alias = "AVErrorMediaDiscontinuity")]
    pub const MEDIA_DISCONTINUITY: ns::Integer = -11812;

    #[doc(alias = "AVErrorMaximumNumberOfSamplesForFileFormatReached")]
    pub const MAX_NUMBER_OF_SAMPLES_FOR_FILE_FORMAT_REACHED: ns::Integer = -11813;

    #[doc(alias = "AVErrorDeviceNotConnected")]
    pub const DEVICE_NOT_CONNECTED: ns::Integer = -11814;

    #[doc(alias = "AVErrorDeviceInUseByAnotherApplication")]
    pub const DEVICE_IN_USE_BY_ANOTHER_APPLICATION: ns::Integer = -11815;

    #[doc(alias = "AVErrorDeviceLockedForConfigurationByAnotherProcess")]
    pub const DEVICE_LOCKED_FOR_CFG_BY_ANOTHER_PROCESS: ns::Integer = -11817;

    #[doc(alias = "AVErrorSessionWasInterrupted")]
    pub const SESSION_WAS_INTERRUPTED: ns::Integer = -11818;

    #[doc(alias = "AVErrorMediaServicesWereReset")]
    pub const MEDIA_SERVICES_WERE_RESET: ns::Integer = -11819;

    #[doc(alias = "AVErrorExportFailed")]
    pub const EXPORT_FAILED: ns::Integer = -11820;

    /// User info may contain `AVErrorMediaTypeKey`, `AVErrorMediaSubTypeKey` & `AVErrorPresentationTimeStampKey`, if available.
    #[doc(alias = "AVErrorDecodeFailed")]
    pub const DECODE_FAILED: ns::Integer = -11821;

    #[doc(alias = "AVErrorInvalidSourceMedia")]
    pub const INVALID_SRC_MEDIA: ns::Integer = -11822;

    #[doc(alias = "AVErrorFileAlreadyExists")]
    pub const FILE_ALREADY_EXISTS: ns::Integer = -11823;

    #[doc(alias = "AVErrorCompositionTrackSegmentsNotContiguous")]
    pub const COMPOSITION_TRACK_SEGMENTS_NOT_CONTIGUOUS: ns::Integer = -11824;

    #[doc(alias = "AVErrorInvalidCompositionTrackSegmentDuration")]
    pub const INVALID_COMPOSITION_TRACK_SEGMENT_DURATION: ns::Integer = -11825;

    #[doc(alias = "AVErrorInvalidCompositionTrackSegmentSourceStartTime")]
    pub const INVALID_COMPOSITION_TRACK_SEGMENT_SRC_START_TIME: ns::Integer = -11826;

    #[doc(alias = "AVErrorInvalidCompositionTrackSegmentSourceDuration")]
    pub const INVALID_COMPOSITION_TRACK_SEGMENT_SRC_DURATION: ns::Integer = -11827;

    #[doc(alias = "AVErrorFileFormatNotRecognized")]
    pub const FILE_FORMAT_NOT_RECOGNIZED: ns::Integer = -11828;

    #[doc(alias = "AVErrorFileFailedToParse")]
    pub const FILE_FAILED_TO_PARSE: ns::Integer = -11829;

    #[doc(alias = "AVErrorMaximumStillImageCaptureRequestsExceeded")]
    pub const MAX_STILL_IMAGE_CAPTURE_REQUESTS_EXCEEDED: ns::Integer = -11830;

    #[doc(alias = "AVErrorContentIsProtected")]
    pub const CONTENT_IS_PROTECTED: ns::Integer = -11831;

    #[doc(alias = "AVErrorNoImageAtTime")]
    pub const NO_IMAGE_AT_TIME: ns::Integer = -11832;

    /// User info may contain `AVErrorMediaTypeKey` & `AVErrorMediaSubTypeKey`, if available.
    #[doc(alias = "AVErrorDecoderNotFound")]
    pub const DECODER_NOT_FOUND: ns::Integer = -11833;

    /// User info may contain `AVErrorMediaTypeKey` & `AVErrorMediaSubTypeKey`, if available.
    #[doc(alias = "AVErrorEncoderNotFound")]
    pub const ENCODER_NOT_FOUND: ns::Integer = -11834;

    #[doc(alias = "AVErrorContentIsNotAuthorized")]
    pub const CONTENT_IS_NOT_AUTHORIZED: ns::Integer = -11835;

    #[doc(alias = "AVErrorApplicationIsNotAuthorized")]
    pub const APPLICATION_IS_NOT_AUTHORIZED: ns::Integer = -11836;

    /// Deprecated: no longer produced since iOS 9.
    #[doc(alias = "AVErrorDeviceIsNotAvailableInBackground")]
    pub const DEVICE_IS_NOT_AVAILABLE_IN_BACKGROUND: ns::Integer = -11837;

    #[doc(alias = "AVErrorOperationNotSupportedForAsset")]
    pub const OP_NOT_SUPPORTED_FOR_ASSET: ns::Integer = -11838;

    /// User info may contain `AVErrorMediaTypeKey` & `AVErrorMediaSubTypeKey`, if available.
    #[doc(alias = "AVErrorDecoderTemporarilyUnavailable")]
    pub const DECODER_TEMPORARILY_UNAVAILABLE: ns::Integer = -11839;

    /// User info may contain `AVErrorMediaTypeKey` & `AVErrorMediaSubTypeKey`, if available.
    #[doc(alias = "AVErrorEncoderTemporarilyUnavailable")]
    pub const ENCODER_TEMPORARILY_UNAVAILABLE: ns::Integer = -11840;

    #[doc(alias = "AVErrorInvalidVideoComposition")]
    pub const INVALID_VIDEO_COMPOSITION: ns::Integer = -11841;

    #[doc(alias = "AVErrorReferenceForbiddenByReferencePolicy")]
    pub const REFERENCE_FORBIDDEN_BY_REFERENCE_POLICY: ns::Integer = -11842;

    #[doc(alias = "AVErrorInvalidOutputURLPathExtension")]
    pub const INVALID_OUTPUT_URL_PATH_EXT: ns::Integer = -11843;

    #[doc(alias = "AVErrorScreenCaptureFailed")]
    pub const SCREEN_CAPTURE_FAILED: ns::Integer = -11844;

    #[doc(alias = "AVErrorDisplayWasDisabled")]
    pub const DISPLAY_WAS_DISABLED: ns::Integer = -11845;

    #[doc(alias = "AVErrorTorchLevelUnavailable")]
    pub const TORCH_LEVEL_UNAVAILABLE: ns::Integer = -11846;

    #[doc(alias = "AVErrorOperationInterrupted")]
    pub const OP_INTERRUPTED: ns::Integer = -11847;

    #[doc(alias = "AVErrorIncompatibleAsset")]
    pub const INCOMPATIBLE_ASSET: ns::Integer = -11848;

    #[doc(alias = "AVErrorFailedToLoadMediaData")]
    pub const FAILED_TO_LOAD_MEDIA_DATA: ns::Integer = -11849;

    #[doc(alias = "AVErrorServerIncorrectlyConfigured")]
    pub const SERVER_INCORRECTLY_CONFIGURED: ns::Integer = -11850;

    #[doc(alias = "AVErrorApplicationIsNotAuthorizedToUseDevice")]
    pub const APPLICATION_IS_NOT_AUTHORIZED_TO_USE_DEVICE: ns::Integer = -11852;

    #[doc(alias = "AVErrorFailedToParse")]
    pub const FAILED_TO_PARSE: ns::Integer = -11853;

    /// User info contains `AVErrorFileTypeKey`.
    #[doc(alias = "AVErrorFileTypeDoesNotSupportSampleReferences")]
    pub const FILE_TYPE_DOES_NOT_SUPPORT_SAMPLE_REFERENCES: ns::Integer = -11854;

    #[doc(alias = "AVErrorUndecodableMediaData")]
    pub const UNDECODABLE_MEDIA_DATA: ns::Integer = -11855;

    #[doc(alias = "AVErrorAirPlayControllerRequiresInternet")]
    pub const AIRPLAY_CONTROLLER_REQUIRES_INTERNET: ns::Integer = -11856;

    #[doc(alias = "AVErrorAirPlayReceiverRequiresInternet")]
    pub const AIRPLAY_RECEIVER_REQUIRES_INTERNET: ns::Integer = -11857;

    #[doc(alias = "AVErrorVideoCompositorFailed")]
    pub const VIDEO_COMPOSITOR_FAILED: ns::Integer = -11858;

    /// On iOS, `AVCaptureMovieFileOutput` only supports one recording at a time.
    #[doc(alias = "AVErrorRecordingAlreadyInProgress")]
    pub const RECORDING_ALREADY_IN_PROGRESS: ns::Integer = -11859;

    #[doc(alias = "AVErrorCreateContentKeyRequestFailed")]
    pub const CREATE_CONTENT_KEY_REQUEST_FAILED: ns::Integer = -11860;

    #[doc(alias = "AVErrorUnsupportedOutputSettings")]
    pub const UNSUPPORTED_OUTPUT_SETTINGS: ns::Integer = -11861;

    #[doc(alias = "AVErrorOperationNotAllowed")]
    pub const OP_NOT_ALLOWED: ns::Integer = -11862;

    #[doc(alias = "AVErrorContentIsUnavailable")]
    pub const CONTENT_IS_UNAVAILABLE: ns::Integer = -11863;

    #[doc(alias = "AVErrorFormatUnsupported")]
    pub const FORMAT_UNSUPPORTED: ns::Integer = -11864;

    #[doc(alias = "AVErrorMalformedDepth")]
    pub const MALFORMED_DEPTH: ns::Integer = -11865;

    #[doc(alias = "AVErrorContentNotUpdated")]
    pub const CONTENT_NOT_UPDATED: ns::Integer = -11866;

    #[doc(alias = "AVErrorNoLongerPlayable")]
    pub const NO_LONGER_PLAYABLE: ns::Integer = -11867;

    #[doc(alias = "AVErrorNoCompatibleAlternatesForExternalDisplay")]
    pub const NO_COMPATIBLE_ALTERNATES_FOR_EXTERNAL_DISPLAY: ns::Integer = -11868;

    #[doc(alias = "AVErrorNoSourceTrack")]
    pub const NO_SRC_TRACK: ns::Integer = -11869;

    #[doc(alias = "AVErrorExternalPlaybackNotSupportedForAsset")]
    pub const EXTERNAL_PLAYBACK_NOT_SUPPORTED_FOR_ASSET: ns::Integer = -11870;

    #[doc(alias = "AVErrorOperationNotSupportedForPreset")]
    pub const OP_NOT_SUPPORTED_FOR_PRESET: ns::Integer = -11871;

    #[doc(alias = "AVErrorSessionHardwareCostOverage")]
    pub const SESSION_HARDWARE_COST_OVERAGE: ns::Integer = -11872;

    #[doc(alias = "AVErrorUnsupportedDeviceActiveFormat")]
    pub const UNSUPPORTED_DEVICE_ACTIVE_FORMAT: ns::Integer = -11873;

    #[doc(alias = "AVErrorIncorrectlyConfigured")]
    pub const INCORRECTLY_CONFIGURED: ns::Integer = -11875;

    #[doc(alias = "AVErrorSegmentStartedWithNonSyncSample")]
    pub const SEGMENT_STARTED_WITH_NON_SYNC_SAMPLE: ns::Integer = -11876;

    #[doc(alias = "AVErrorRosettaNotInstalled")]
    pub const ROSETTA_NOT_INSTALLED: ns::Integer = -11877;

    #[doc(alias = "AVErrorOperationCancelled")]
    pub const OP_CANCELLED: ns::Integer = -11878;

    #[doc(alias = "AVErrorContentKeyRequestCancelled")]
    pub const CONTENT_KEY_REQUEST_CANCELLED: ns::Integer = -11879;

    #[doc(alias = "AVErrorInvalidSampleCursor")]
    pub const INVALID_SAMPLE_CURSOR: ns::Integer = -11880;

    #[doc(alias = "AVErrorFailedToLoadSampleData")]
    pub const FAILED_TO_LOAD_SAMPLE_DATA: ns::Integer = -11881;

    #[doc(alias = "AVErrorAirPlayReceiverTemporarilyUnavailable")]
    pub const AIRPLAY_RECEIVER_TEMPORARILY_UNAVAILABLE: ns::Integer = -11882;

    #[doc(alias = "AVErrorEncodeFailed")]
    pub const ENCODE_FAILED: ns::Integer = -11883;

    #[doc(alias = "AVErrorSandboxExtensionDenied")]
    pub const SANDBOX_EXT_DENIED: ns::Integer = -11884;

    #[doc(alias = "AVErrorToneMappingFailed")]
    pub const TONE_MAPPING_FAILED: ns::Integer = -11885;

    #[doc(alias = "AVErrorMediaExtensionDisabled")]
    pub const MEDIA_EXT_DISABLED: ns::Integer = -11886;

    #[doc(alias = "AVErrorMediaExtensionConflict")]
    pub const MEDIA_EXT_CONFLICT: ns::Integer = -11887;

    #[doc(alias = "AVErrorNoSmartFramingsEnabled")]
    pub const NO_SMART_FRAMINGS_ENABLED: ns::Integer = -11890;

    #[doc(alias = "AVErrorAutoWhiteBalanceNotLocked")]
    pub const AUTO_WHITE_BALANCE_NOT_LOCKED: ns::Integer = -11891;

    #[doc(alias = "AVErrorFollowExternalSyncDeviceTimedOut")]
    pub const FOLLOW_EXTERNAL_SYNC_DEVICE_TIMED_OUT: ns::Integer = -11892;

    #[doc(alias = "AVErrorFollowExternalSyncFailed")]
    pub const FOLLOW_EXTERNAL_SYNC_FAILED: ns::Integer = -11894;

    #[doc(alias = "AVErrorExternalSyncDeviceFrequencyHigherThanSpecified")]
    pub const EXTERNAL_SYNC_DEVICE_FREQUENCY_HIGHER_THAN_SPECIFIED: ns::Integer = -11895;

    #[doc(alias = "AVErrorExternalSyncDeviceFrequencyLowerThanSpecified")]
    pub const EXTERNAL_SYNC_DEVICE_FREQUENCY_LOWER_THAN_SPECIFIED: ns::Integer = -11896;

    #[doc(alias = "AVErrorNotEnoughSpaceForProVideoStorageReplenishment")]
    pub const NOT_ENOUGH_SPACE_FOR_PRO_VIDEO_STORAGE_REPLENISHMENT: ns::Integer = -11897;
}

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        assert_eq!(av::error::domain().to_string(), "AVFoundationErrorDomain");
        assert_eq!(
            av::error::code::NOT_ENOUGH_SPACE_FOR_PRO_VIDEO_STORAGE_REPLENISHMENT,
            -11897
        );
    }
}
