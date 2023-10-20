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
    #[doc(alias = "AVCaptureDeviceTypeExternalUnknown")]
    #[cfg(target_os = "macos")]
    pub fn external_unknown() -> &'static Self {
        unsafe { AVCaptureDeviceTypeExternalUnknown }
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

    /// A distortion corrected cut out from an ultra wide camera, made to approximate
    /// an overhead camera pointing at a desk.
    /// Supports multicam operation.
    #[doc(alias = "AVCaptureDeviceTypeDeskViewCamera")]
    #[cfg(target_os = "macos")]
    pub fn desk_view_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeDeskViewCamera }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {

    #[cfg(target_os = "macos")]
    static AVCaptureDeviceTypeExternalUnknown: &'static Type;
    static AVCaptureDeviceTypeBuiltInMicrophone: &'static Type;
    static AVCaptureDeviceTypeBuiltInWideAngleCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTelephotoCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInUltraWideCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInDualCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInDualWideCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTripleCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTrueDepthCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInLiDARDepthCamera: &'static Type;
    #[cfg(target_os = "macos")]
    static AVCaptureDeviceTypeDeskViewCamera: &'static Type;
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
    /// Phase detection autofocus is typically less visually intrusive than contrast detection autofocus.
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CenterStageControlMode {
    User = 0,
    App = 1,
    Cooperative = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum MicrophoneMode {
    Standard = 0,
    WideSpectrum = 1,
    VoiceIsolation = 2,
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

    pub fn was_connected() -> &'static cf::NotificationName {
        unsafe { AVCaptureDeviceWasConnectedNotification }
    }

    pub fn was_disconnected() -> &'static cf::NotificationName {
        unsafe { AVCaptureDeviceWasDisconnectedNotification }
    }

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
    use crate::av::{
        self,
        capture::device::{self, Device},
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
}
