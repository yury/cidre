use std::ops::{Deref, DerefMut};

use crate::{
    arc,
    av::{self, MediaType},
    cf, cg, cm, define_cls, define_obj_type, ns, objc,
};

use super::SessionPreset;

define_obj_type!(Type(ns::String));

/// ```
/// use cidre::av;
///
/// let device_type = av::CaptureDeviceType::external();
/// let device_type = av::CaptureDeviceType::external_unknown();
/// let device_type = av::CaptureDeviceType::built_in_microphone();
/// let device_type = av::CaptureDeviceType::built_in_wide_angle_camera();
/// let device_type = av::CaptureDeviceType::built_in_telephoto_camera();
/// let device_type = av::CaptureDeviceType::built_in_ultra_wide_camera();
/// let device_type = av::CaptureDeviceType::built_in_dual_camera();
/// let device_type = av::CaptureDeviceType::built_in_dual_wide_camera();
/// let device_type = av::CaptureDeviceType::built_in_tripple_camera();
/// let device_type = av::CaptureDeviceType::built_in_true_depth_camera();
/// let device_type = av::CaptureDeviceType::built_in_lidar_depth_camera();
/// ```
impl Type {
    /// An external device type. On iPad, external devices are those that conform
    /// to the UVC (USB Video Class) specification.
    ///
    /// Starting in Mac Catalyst 17.0, apps may opt in for using
    /// 'av::CaptureDeviceType::external()' by adding the following key
    /// to their Info.plist:
    /// ```xml
    ///  <key>NSCameraUseExternalDeviceType</key>
    ///  <true/>
    /// ```
    /// Otherwise, external cameras on Mac Catalyst report that their device type is
    /// 'av::CaptureDeviceType::built_in_wide_angle_camera()'.
    #[doc(alias = "AVCaptureDeviceTypeExternal")]
    pub fn external() -> &'static Self {
        unsafe { AVCaptureDeviceTypeExternal }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInMicrophone")]
    pub fn built_in_microphone() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInMicrophone }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInWideAngleCamera")]
    pub fn built_in_wide_angle_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInWideAngleCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInTelephotoCamera")]
    pub fn built_in_telephoto_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTelephotoCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInUltraWideCamera")]
    pub fn built_in_ultra_wide_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInUltraWideCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInDualCamera")]
    pub fn built_in_dual_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInDualCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInDualWideCamera")]
    pub fn built_in_dual_wide_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInDualWideCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInTripleCamera")]
    pub fn built_in_tripple_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTripleCamera }
    }

    /// A device that consists of two cameras, one YUV and one Infrared.
    /// The infrared camera provides high quality depth information that is synchronized
    /// and perspective corrected to frames produced by the YUV camera. While the resolution
    /// of the depth data and YUV frames may differ, their field of view and aspect ratio
    /// always match. Note that devices of this type may only be discovered using an
    /// `av::CaptureDevice::default_device_with_device_type_media_type_position`.
    #[doc(alias = "AVCaptureDeviceTypeBuiltInTrueDepthCamera")]
    pub fn built_in_true_depth_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTrueDepthCamera }
    }
    /// A device that consists of two cameras, one YUV and one LiDAR.
    /// The LiDAR camera provides high quality, high accuracy depth information by measuring
    /// the round trip of an artificial light signal emitted by a laser. The depth
    /// is synchronized and perspective corrected to frames produced by the paired YUV camera.
    /// While the resolution of the depth data and YUV frames may differ, their field of view
    /// and aspect ratio always match. Note that devices of this type may only be discovered
    /// using an av::CaptureDeviceDiscoverySession or
    /// `av::CaptureDevice::default_device_with_device_type_media_type_position`.
    #[doc(alias = "AVCaptureDeviceTypeBuiltInLiDARDepthCamera")]
    pub fn built_in_lidar_depth_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInLiDARDepthCamera }
    }

    /// A continuity camera device. These devices are suitable for general purpose use.
    /// Note that devices of this type may only be discovered using an
    /// av::CaptureDeviceDiscoverySession or
    ///`av::CaptureDevice::default_device_with_device_type_media_type_position`.
    ///
    /// Starting in macOS 14.0 and Mac Catalyst 17.0, apps may opt in for using
    /// 'av::CaptureDeviceType::continuity_camera()' by adding the following key
    /// to their Info.plist:
    /// ```xml
    /// <key>NSCameraUseContinuityCameraDeviceType</key>
    /// <true/>
    /// ```
    #[doc(alias = "AVCaptureDeviceTypeContinuityCamera")]
    pub fn continuity_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeContinuityCamera }
    }

    /// A distortion corrected cut out from an ultra wide camera, made to approximate
    /// an overhead camera pointing at a desk.
    /// Supports multicam operation.
    #[doc(alias = "AVCaptureDeviceTypeDeskViewCamera")]
    #[cfg(target_os = "macos")]
    pub fn desk_view_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeDeskViewCamera }
    }
    // #[cfg(target_os = "macos")]
    // #[doc(alias = "AVCaptureDeviceTypeExternalUnknown")]
    // pub fn external_unknown() -> &'static Self {
    //     unsafe { AVCaptureDeviceTypeExternalUnknown }
    // }
}

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVCaptureDeviceTypeExternal: &'static Type;
    static AVCaptureDeviceTypeBuiltInMicrophone: &'static Type;
    static AVCaptureDeviceTypeBuiltInWideAngleCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTelephotoCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInUltraWideCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInDualCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInDualWideCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTripleCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTrueDepthCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInLiDARDepthCamera: &'static Type;
    static AVCaptureDeviceTypeContinuityCamera: &'static Type;
    #[cfg(target_os = "macos")]
    static AVCaptureDeviceTypeDeskViewCamera: &'static Type;
    // #[cfg(target_os = "macos")]
    // static AVCaptureDeviceTypeExternalUnknown: &'static Type;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(isize)]
pub enum Position {
    Unspecified = 0,
    Back = 1,
    Front = 2,
}

define_obj_type!(Device(ns::Id));

impl Device {
    define_cls!(AV_CAPTURE_DEVICE);

    #[objc::cls_msg_send(defaultDeviceWithDeviceType:mediaType:position:)]
    pub fn with_device_type_media_and_position_ar(
        device_type: &Type,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_device_type_media_and_position(
        device_type: &Type,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(deviceWithUniqueID:)]
    pub fn with_unique_id_ar(unique_id: ns::String) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_unique_id(unique_id: ns::String) -> Option<arc::R<Self>>;

    #[objc::msg_send(uniqueID)]
    pub fn unique_id(&self) -> &ns::String;

    #[objc::msg_send(formats)]
    pub fn formats(&self) -> &ns::Array<Format>;

    #[objc::msg_send(supportsAVCaptureSessionPreset:)]
    pub fn supports_preset(&self, preset: &SessionPreset) -> bool;

    #[objc::msg_send(activeFormat)]
    pub fn active_format(&self) -> &Format;

    #[objc::msg_send(activeVideoMinFrameDuration)]
    pub fn active_video_min_frame_duration(&self) -> cm::Time;

    #[objc::msg_send(activeVideoMaxFrameDuration)]
    pub fn active_video_max_frame_duration(&self) -> cm::Time;

    #[objc::msg_send(hasTorch)]
    pub fn has_torch(&self) -> bool;

    #[objc::msg_send(isRampingVideoZoom)]
    pub fn is_ramping_video_zoom(&self) -> bool;

    pub fn configuration_lock(&mut self) -> Result<ConfigurationLockGuard, arc::R<cf::Error>> {
        let mut error = None;
        unsafe {
            let result = self.lock_for_configuration(&mut error);
            if let Some(error) = error.take() {
                return Err(error);
            }

            debug_assert!(result);

            Ok(ConfigurationLockGuard { device: self })
        }
    }

    #[objc::msg_send(lockForConfiguration:)]
    pub unsafe fn lock_for_configuration(&mut self, error: &mut Option<arc::R<cf::Error>>) -> bool;

    #[objc::msg_send(unlockForConfiguration)]
    pub unsafe fn unlock_for_configuration(&mut self);

    #[objc::msg_send(setActiveFormat:)]
    pub unsafe fn set_active_format(&mut self, value: &Format);

    #[objc::msg_send(setActiveVideoMinFrameDuration:)]
    pub unsafe fn set_active_video_min_frame_duration(&mut self, value: cm::Time);

    #[objc::msg_send(setActiveVideoMaxFrameDuration:)]
    pub unsafe fn set_active_video_max_frame_duration(&mut self, value: cm::Time);

    #[objc::msg_send(rampToVideoZoomFactor:withRate:)]
    pub unsafe fn ramp_to_video_zoom_factor_throws(&mut self, factor: cg::Float, rate: f32);

    #[objc::msg_send(cancelVideoZoomRamp)]
    pub unsafe fn cancel_video_zoom_ramp_throws(&mut self);
}

/// AVCaptureDeviceReactionEffects
impl Device {
    #[objc::cls_msg_send(reactionEffectsEnabled)]
    pub fn reaction_effects_enabled() -> bool;

    #[objc::cls_msg_send(reactionEffectGesturesEnabled)]
    pub fn reaction_effect_gestures_enabled() -> bool;

    #[objc::msg_send(canPerformReactionEffects)]
    pub fn can_perform_reaction_effects(&self) -> bool;

    /// Returns a list of reaction types which can be passed to perform_effect_for_reaction.
    ///
    /// The list may differ between devices, or be affected by changes to active format,
    /// and can be key-value observed.
    #[objc::msg_send(availableReactionTypes)]
    pub fn available_reaction_types(&self) -> &ns::Set<av::CaptureReactionType>;

    /// Triggers a specified reaction on the video stream.
    #[objc::msg_send(performEffectForReaction:)]
    pub fn perform_effect_for_reaction(&mut self, reaction_type: &av::CaptureReactionType);

    #[objc::msg_send(reactionEffectsInProgress)]
    pub fn reaction_effects_in_progress(&self) -> &ns::Array<av::CaptureReactionEffectState>;
}

/// AVCaptureDeviceContinuityCamera
impl Device {
    /// A property that reports YES if the receiver is a Continuity Camera.
    ///
    /// Access this property to discover if the receiver is
    /// a Continuity Camera (external iPhone webcam).
    #[objc::msg_send(isContinuityCamera)]
    pub fn is_continuity_camera(&self) -> bool;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum MicMode {
    Standard = 0,
    WideSpectrum = 1,
    VoiceIsolation = 2,
}

/// AVCaptureMicrophoneMode
impl Device {
    /// Indicates the microphone mode that has been selected by the user in Control Center.
    ///
    /// This readonly property returns the microphone mode selected by the user in Control Center. It is key-value observable.
    #[objc::cls_msg_send(preferredMicrophoneMode)]
    pub fn preferred_mic_mode() -> MicMode;

    /// Indicates the currently active microphone mode.
    ///
    /// This readonly property returns the currently active microphone mode,
    /// which may differ from the 'preferred_mic_mode()' if the application's
    /// active audio route does not support the preferred microphone mode.
    /// This property is key-value observable.
    #[objc::cls_msg_send(activeMicrophoneMode)]
    pub fn active_mic_mode() -> MicMode;
}

#[doc(alias = "AVCaptureCenterStageControlMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CenterStageControlMode {
    /// Indicates that the application is unaware of the Center Stage feature.
    /// Its enablement is entirely under user control in Control Center.
    User = 0,

    /// Indicates that the application controls the Center Stage feature,
    /// disallowing input from the user in Control Center.
    App = 1,

    /// Indicates that both the user and application cooperatively share
    /// control of the Center Stage feature.
    Cooperative = 2,
}

impl Device {
    /// Current mode of Center Stage control (user, app, or cooperative).
    ///
    /// This class property determines how the Center Stage feature is controlled.
    /// When set to the default value of 'av::CaptureCenterStageControlMode::User',
    /// 'set_center_stage_enabled' may not be set programmatically and throws an
    /// NSInvalidArgumentException. In User mode, the feature may only be set by
    /// the user in Control Center. If you wish to take Center Stage control away
    /// from the user and exclusively enable / disable it programmatically, set this property
    /// to 'av::CaptureCenterStageControlMode::App'. When under exclusive app control,
    /// Center Stage user control is disallowed (for instance, the toggle is grayed out
    /// in Control Center). If you wish to take control of Center Stage, but also cooperate
    /// with the user by listening for and appropriately reacting to their changes to the
    /// 'center_stage_enabled' property, set this property to
    /// 'av::CaptureCenterStageControlMode::Cooperative'. Note that in this mode,
    /// the onus is on you, the app developer, to honor user intent and conform your
    /// 'av::CaptureSession' configuration to make Center Stage active (see the 'av::CaptureDevice'
    /// instance property 'center_stage_active'). In cooperative mode, the 'center_stage_enabled'
    /// property may change at any time (such as when the user enables / disables the feature
    /// in Control Center).
    #[objc::cls_msg_send(centerStageControlMode)]
    pub fn center_stage_control_mode() -> CenterStageControlMode;

    #[objc::cls_msg_send(setCenterStageControlMode:)]
    pub fn set_center_stage_control_mode(value: CenterStageControlMode);

    /// Indicates whether Center Stage is currently active on a particular av::CaptureDevice.
    ///
    /// This readonly property returns 'true' when Center Stage is currently active on
    /// the receiver. When active, the camera automatically adjusts to keep people optimally
    /// framed within the field of view. The field of view may pan, tighten or widen as needed.
    /// Certain restrictions come into play when Center Stage is active:
    ///
    /// - The device's 'min_available_video_zoom_factor' and 'max_available_video_zoom_factor'
    ///   become restricted (see av::CaptureDeviceFormat's 'video_min_zoom_factor_for_center_stage'
    ///   and 'video_max_zoom_factor_for_center_stage').
    /// - The device's 'active_video_min_frame_duration' and 'active_video_max_frame_duration'
    ///   are limited (see av::CaptureDeviceFormat's 'video_frame_rate_range_for_center_stage').
    ///
    /// Center Stage may be enabled via user control or application control, depending on the
    /// current 'av::CaptureDevice::center_stage_control_mode()'.
    /// When 'av::CaptureDevice::is_center_stage_enabled()' is 'true', a particular
    /// 'av::CaptureDevice' instance may return 'true' for this property, depending whether it
    /// supports the feature in its current configuration. Some device features are mutually
    /// exclusive to Center Stage:
    ///
    /// - If depth data delivery is enabled on any output, such as 'av::CaptureDepthDataOutput',
    ///   or 'av::CapturePhotoOutput.depth_data_delivery_enabled', Center Stage is deactivated.
    /// - If 'geometric_distortion_correction_supported' is 'true',
    ///   'geometric_distortion_correction_enabled' must also be 'true', or Center Stage
    ///   is deactivated.
    ///
    /// This property is key-value observable.
    #[objc::msg_send(isCenterStageActive)]
    pub fn is_center_stage_active(&self) -> bool;

    #[objc::msg_send(centerStageRectOfInterest)]
    pub fn center_stage_rect_of_interest(&self) -> cg::Rect;

    /// Specifies the effective region within the output pixel buffer that will be used
    /// to perform Center Stage framing.
    ///
    /// Applications that wish to apply additional processing (such as cropping) on top of
    /// Center Stage's output can use this property to guide Center Stage's framing.
    ///
    /// The rectangle's origin is top left and is relative to the coordinate space of the
    /// output pixel buffer. The default value of this property is the value
    /// 'cg::Rect::new(0.0, 0.0, 1.0, 1.0)', where {0.0,0.0} represents the top left of the
    /// picture area, and {1.0,1.0} represents the bottom right on an unrotated picture.
    /// This rectangle of interest is applied prior to rotation, mirroring or scaling.
    ///
    /// Pixels outside of this rectangle of interest will be blackened out.
    ///
    /// Setting this property has no impact on objects specified in the metadata output.
    ///
    /// 'set_center_stage_rect_of_interest_throws': throws an 'ns::ExceptionName::generic()'
    /// if called without first obtaining exclusive access to the receiver using
    /// 'lock_for_configuration' 'set_center_stage_rect_of_interest_throws'
    /// throws an 'ns::ExceptionName::invalid_argument()' if none of the av::CaptureDeviceFormats
    /// supported by the receiver support CenterStage.
    /// 'set_center_stage_rect_of_interest_throws' throws an 'ns::ExceptionName::invalid_argument()'
    /// '::center_stage_enabled' is 'false' on the av::CaptureDevice class.
    /// 'set_center_stage_rect_of_interest_throws' throws an 'ns::ExceptionName::invalid_argument()'
    /// if the provided rect of interest goes outside the normalized (0-1) coordinate space.
    #[objc::msg_send(setCenterStageRectOfInterest:)]
    pub unsafe fn set_center_stage_rect_of_interest_throws(&mut self, value: cg::Rect);
}

#[doc(alias = "AVCaptureSystemUserInterface")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum SystemUi {
    VideoEffects = 1,
    MicModes = 2,
}

/// AVCaptureSystemUserInterface
impl Device {
    #[objc::cls_msg_send(showSystemUserInterface:)]
    pub fn show_system_ui(system_ui: SystemUi);
}

pub struct ConfigurationLockGuard<'a> {
    device: &'a mut Device,
}

impl<'a> ConfigurationLockGuard<'a> {
    pub fn set_active_format(&mut self, value: &Format) {
        unsafe { self.device.set_active_format(value) }
    }

    pub fn set_active_video_min_frame_duration(&mut self, value: cm::Time) {
        unsafe { self.device.set_active_video_min_frame_duration(value) }
    }

    pub fn set_active_video_max_frame_duration(&mut self, value: cm::Time) {
        unsafe { self.device.set_active_video_max_frame_duration(value) }
    }

    pub fn ramp_to_video_zoom_factor(&mut self, factor: cg::Float, rate: f32) {
        unsafe { self.device.ramp_to_video_zoom_factor_throws(factor, rate) }
    }

    pub fn cancel_video_zoom_ramp(&mut self) {
        unsafe { self.device.cancel_video_zoom_ramp_throws() }
    }

    pub fn set_center_stage_rect_of_interest<'ar>(
        &mut self,
        value: cg::Rect,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.device.set_center_stage_rect_of_interest_throws(value) })
    }
}

impl<'a> Drop for ConfigurationLockGuard<'a> {
    fn drop(&mut self) {
        unsafe { self.device.unlock_for_configuration() }
    }
}

impl<'a> Deref for ConfigurationLockGuard<'a> {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        self.device
    }
}

impl<'a> DerefMut for ConfigurationLockGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.device
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_DEVICE: &'static objc::Class<Device>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum TorchMode {
    Off = 0,
    On = 1,
    Auto = 2,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum FocusMode {
    Locked = 0,
    AutoFocus = 1,
    ContinuousAutoFocus = 2,
}

#[doc(alias = "AVCaptureAutoFocusSystem")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum AutoFocusSystem {
    /// Indicates that autofocus is not available.
    None = 0,

    /// Indicates that autofocus is achieved by contrast detection.
    /// Contrast detection performs a focus scan to find the optimal position.
    ContrastDetection = 1,

    /// Indicates that autofocus is achieved by phase detection.
    /// Phase detection has the ability to achieve focus in many cases without a focus scan.
    /// Phase detection autofocus is typically less visually intrusive than contrast
    // detection autofocus.
    PhaseDetection = 2,
}

define_obj_type!(FrameRateRange(ns::Id));

impl FrameRateRange {
    #[objc::msg_send(minFrameRate)]
    pub fn min_frame_rate(&self) -> f64;

    #[objc::msg_send(maxFrameRate)]
    pub fn max_frame_rate(&self) -> f64;

    #[objc::msg_send(maxFrameDuration)]
    pub fn max_frame_duration(&self) -> cm::Time;

    #[objc::msg_send(minFrameDuration)]
    pub fn min_frame_duration(&self) -> cm::Time;
}

define_obj_type!(Format(ns::Id));
impl Format {
    #[objc::msg_send(mediaType)]
    pub fn media_type(&self) -> &av::MediaType;

    #[objc::msg_send(formatDescription)]
    pub fn format_description(&self) -> &cm::FormatDescription;

    #[objc::msg_send(videoSupportedFrameRateRanges)]
    pub fn video_supported_frame_rate_ranges(&self) -> &ns::Array<FrameRateRange>;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(videoFieldOfView)]
    pub fn video_fov(&self) -> f32;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isVideoBinned)]
    pub fn is_video_binned(&self) -> bool;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isVideoStabilizationModeSupported:)]
    pub fn is_video_stabilization_mode_supported(&self, mode: VideoStabilizationMode) -> bool;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(videoMaxZoomFactor)]
    pub fn video_max_zoom_factor(&self) -> cg::Float;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(videoZoomFactorUpscaleThreshold)]
    pub fn video_zoom_factor_upscale_threshold(&self) -> cg::Float;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(minExposureDuration)]
    pub fn min_exposure_duration(&self) -> cm::Time;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(maxExposureDuration)]
    pub fn max_exposure_duration(&self) -> cm::Time;

    #[objc::msg_send(autoFocusSystem)]
    pub fn auto_focus_system(&self) -> AutoFocusSystem;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isMultiCamSupported)]
    pub fn is_mutli_cam_supported(&self) -> bool;

    #[objc::msg_send(geometricDistortionCorrectedVideoFieldOfView)]
    pub fn geometric_distortion_corrected_video_field_of_view(&self) -> f32;
}

/// Center Stage
impl Format {
    #[objc::msg_send(isCenterStageSupported)]
    pub fn is_center_stage_supported(&self) -> bool;

    #[objc::msg_send(videoMinZoomFactorForCenterStage)]
    pub fn video_min_zoom_factor_for_center_stage(&self) -> cg::Float;

    #[objc::msg_send(videoFrameRateRangeForCenterStage)]
    pub fn video_frame_rate_range_for_center_stage(&self) -> Option<&FrameRateRange>;
}

/// Portrait Effect
impl Format {
    /// Indicates whether the format supports the Portrait Effect feature.
    #[objc::msg_send(isPortraitEffectSupported)]
    pub fn is_portrait_effect_supported(&self) -> bool;
}

pub mod notifications {
    use crate::cf;

    /// Posted when a device becomes available on the system.
    pub fn was_connected() -> &'static cf::NotificationName {
        unsafe { AVCaptureDeviceWasConnectedNotification }
    }

    /// Posted when a device becomes unavailable on the system.
    pub fn was_disconnected() -> &'static cf::NotificationName {
        unsafe { AVCaptureDeviceWasDisconnectedNotification }
    }

    /// Posted when the instance of av::CaptureDevice has detected a substantial
    /// change to the video subject area.
    #[cfg(not(target_os = "macos"))]
    pub fn subject_area_did_change() -> &'static cf::NotificationName {
        unsafe { AVCaptureDeviceSubjectAreaDidChangeNotification }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    extern "C" {
        static AVCaptureDeviceWasConnectedNotification: &'static cf::NotificationName;
        static AVCaptureDeviceWasDisconnectedNotification: &'static cf::NotificationName;
        #[cfg(not(target_os = "macos"))]
        static AVCaptureDeviceSubjectAreaDidChangeNotification: &'static cf::NotificationName;
    }
}

define_obj_type!(CaptureAudioChannel(ns::Id));

define_obj_type!(DiscoverySession(ns::Id));
impl DiscoverySession {
    define_cls!(AV_CAPTURE_DEVICE_DISCOVERY_SESSION);

    #[objc::cls_msg_send(discoverySessionWithDeviceTypes:mediaType:position:)]
    pub fn with_device_types_media_and_position_ar(
        device_types: &ns::Array<Type>,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_device_types_media_and_position(
        device_types: &ns::Array<Type>,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> arc::R<Self>;

    /// The list of devices that comply to the search criteria specified
    /// on the discovery session.
    ///
    /// The returned array contains only devices that are available at the time the method
    /// is called. Applications can key-value observe this property to be notified when
    /// the list of available devices has changed. For apps linked against iOS 10,
    /// the devices returned are unsorted. For apps linked against iOS 11 or later,
    /// the devices are sorted by 'av::CaptureDeviceType', matching the order specified
    /// in the deviceTypes parameter of 'av::CaptureDeviceDiscoverySession::with_device_types_media_position`.
    /// If a position of 'av::CaptureDevicePosition::unspecified' is specified,
    /// the results are further ordered by position in the 'av::CaptureDevicePosition' enum.
    /// Starting in Mac Catalyst 14.0, clients can key value observe the value of this
    /// property to be notified when the devices change.
    #[objc::msg_send(devices)]
    pub fn devices(&self) -> &ns::Array<Device>;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(supportedMultiCamDeviceSets)]
    pub fn supported_multi_cam_device_sets(&self) -> &ns::Array<ns::Set<Device>>;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_DEVICE_DISCOVERY_SESSION: &'static objc::Class<DiscoverySession>;
}

#[doc(alias = "AVCaptureVideoStabilizationMode")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum VideoStabilizationMode {
    Off = 0,
    Standard = 1,
    Cinematic = 2,
    CinematicExtended,
    Auto = -1,
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::{
        av::{
            self, capture,
            capture::device::{self, Device},
        },
        cm::io,
        ns,
    };

    #[test]
    fn basics() {
        let device_type = device::Type::built_in_wide_angle_camera();
        let media_type = av::MediaType::video();
        let position = device::Position::Front;
        let mut device =
            Device::with_device_type_media_and_position(device_type, Some(media_type), position)
                .expect("device");
        //device.unique_id().show();
        assert!(!device.has_torch());
        assert!(device.formats().len() > 0);
        assert!(device.supports_preset(av::CaptureSessionPreset::photo()));
        let mut _lock = device.configuration_lock().expect("locked");
    }

    #[test]
    fn session() {
        io::Object::SYSTEM
            .set_allow_screen_capture_devices(true)
            .unwrap();
        io::Object::SYSTEM
            .set_allow_wireless_screen_capture_devices(true)
            .unwrap();

        io::Object::SYSTEM.show();

        let list = ns::Array::from_slice(&[av::CaptureDeviceType::external()]);
        let session = capture::DiscoverySession::with_device_types_media_and_position(
            list.as_ref(),
            Some(av::MediaType::muxed()),
            capture::DevicePosition::Unspecified,
        );

        let devices = session.devices();
        devices.as_type_ref().show();
    }
}
