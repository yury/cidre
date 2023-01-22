use std::ops::{Deref, DerefMut};

use crate::{arc, av::MediaType, cf, cm, define_cls, define_obj_type, ns, objc};

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
    #[cfg(target_os = "macos")]
    pub fn external_unknown() -> &'static Self {
        unsafe { AVCaptureDeviceTypeExternalUnknown }
    }

    pub fn built_in_microphone() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInMicrophone }
    }

    pub fn built_in_wide_angle_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInWideAngleCamera }
    }

    pub fn built_in_telephoto_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTelephotoCamera }
    }

    pub fn built_in_ultra_wide_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInUltraWideCamera }
    }

    pub fn built_in_dual_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInDualCamera }
    }

    pub fn built_in_dual_wide_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInDualWideCamera }
    }

    pub fn built_in_tripple_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTripleCamera }
    }

    pub fn built_in_true_depth_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTrueDepthCamera }
    }

    pub fn built_in_lidar_depth_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInLiDARDepthCamera }
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
}

#[repr(isize)]
pub enum Position {
    Unspecified = 0,
    Back = 1,
    Front = 2,
}

define_obj_type!(Device(ns::Id));

impl arc::A<Device> {}

impl Device {
    define_cls!(AV_CAPTURE_DEVICE);

    pub fn with_device_type_media_and_position(
        device_type: &Type,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> Option<arc::R<Self>> {
        unsafe {
            AVCaptureDevice_defaultDeviceWithDeviceType_mediaType_position(
                device_type,
                media_type,
                position,
            )
        }
    }

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
}

pub struct ConfigurationLockGuard<'a> {
    device: &'a mut Device,
}

impl<'a> ConfigurationLockGuard<'a> {
    pub fn set_active_format(&mut self, value: &Format) {
        unsafe { wsel_setActiveFormat(self.device, value) }
    }

    pub fn set_active_video_min_frame_duration(&mut self, value: cm::Time) {
        unsafe { wsel_setActiveVideoMinFrameDuration(self.device, value) }
    }

    pub fn set_active_video_max_frame_duration(&mut self, value: cm::Time) {
        unsafe { wsel_setActiveVideoMaxFrameDuration(self.device, value) }
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
    fn AVCaptureDevice_defaultDeviceWithDeviceType_mediaType_position(
        device_type: &Type,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> Option<arc::R<Device>>;

    fn wsel_setActiveFormat(device: &mut Device, value: &Format);

    fn wsel_setActiveVideoMinFrameDuration(device: &mut Device, value: cm::Time);

    fn wsel_setActiveVideoMaxFrameDuration(device: &mut Device, value: cm::Time);
}

#[repr(isize)]
pub enum TorchMode {
    Off = 0,
    On = 1,
    Auto = 2,
}

#[repr(isize)]
pub enum FocusMode {
    Locked = 0,
    AutoFocus = 1,
    ContinuousAutoFocus = 2,
}

#[derive(Debug, Clone, Copy)]
#[repr(isize)]
pub enum AutoFocusSystem {
    None = 0,
    ContrastDetection = 1,
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

#[repr(isize)]
pub enum CenterStageControlMode {
    User = 0,
    App = 1,
    Cooperative = 2,
}

#[repr(isize)]
pub enum MicrophoneMode {
    Standard = 0,
    WideSpectrum = 1,
    VoiceIsolation = 2,
}

define_obj_type!(Format(ns::Id));

impl Format {
    #[cfg(not(target_os = "macos"))]
    pub fn is_video_binned(&self) -> bool {
        unsafe { rsel_isVideoBinned(self) }
    }

    pub fn video_supported_frame_rate_ranges(&self) -> &cf::ArrayOf<FrameRateRange> {
        unsafe { rsel_videoSupportedFrameRateRanges(self) }
    }

    pub fn format_description(&self) -> &cm::FormatDescription {
        unsafe { rsel_formatDescription(self) }
    }

    pub fn auto_focus_system(&self) -> AutoFocusSystem {
        unsafe { rsel_autoFocusSystem(self) }
    }

    #[cfg(not(target_os = "macos"))]
    pub fn is_mutli_cam_supported(&self) -> bool {
        unsafe { rsel_isMultiCamSupported(self) }
    }

    pub fn is_center_sstage_supported(&self) -> bool {
        unsafe { rsel_isCenterStageSupported(self) }
    }

    pub fn video_frame_rate_range_for_center_stage(&self) -> Option<&FrameRateRange> {
        unsafe { rsel_videoFrameRateRangeForCenterStage(self) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    #[cfg(not(target_os = "macos"))]
    fn rsel_isVideoBinned(format: &Format) -> bool;
    #[cfg(not(target_os = "macos"))]
    fn rsel_isMultiCamSupported(format: &Format) -> bool;

    fn rsel_isCenterStageSupported(format: &Format) -> bool;

    fn rsel_videoSupportedFrameRateRanges(format: &Format) -> &cf::ArrayOf<FrameRateRange>;
    fn rsel_formatDescription(format: &Format) -> &cm::FormatDescription;
    fn rsel_autoFocusSystem(format: &Format) -> AutoFocusSystem;

    fn rsel_videoFrameRateRangeForCenterStage(format: &Format) -> Option<&FrameRateRange>;
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
    pub fn with_device_types_media_and_position(
        device_types: &cf::ArrayOf<Type>,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> arc::R<Self> {
        unsafe {
            AVCaptureDeviceDiscoverySession_discoverySessionWithDeviceTypes_mediaType_position(
                device_types,
                media_type,
                position,
            )
        }
    }

    pub fn devices(&self) -> &cf::ArrayOf<Device> {
        unsafe { rsel_devices(self) }
    }

    #[cfg(not(target_os = "macos"))]
    pub fn supported_multi_cam_device_sets(&self) -> &cf::ArrayOf<cf::SetOf<Device>> {
        unsafe { rsel_supportedMultiCamDeviceSets(self) }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVCaptureDeviceDiscoverySession_discoverySessionWithDeviceTypes_mediaType_position(
        device_types: &cf::Array,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> arc::R<DiscoverySession>;
    fn rsel_devices(id: &ns::Id) -> &cf::ArrayOf<Device>;
    #[cfg(not(target_os = "macos"))]
    fn rsel_supportedMultiCamDeviceSets(id: &ns::Id) -> &cf::ArrayOf<cf::SetOf<Device>>;
}

#[derive(Debug, Clone, Copy)]
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
