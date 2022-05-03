use std::ops::{Deref, DerefMut};

use crate::{
    av::MediaType,
    cf::{self, Retained},
    define_obj_type,
    objc::Id,
};

use super::SessionPreset;

define_obj_type!(Format(Id));

pub type Type = cf::String;

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

define_obj_type!(Device(Id));

impl Device {
    /// ```
    /// use cidre::{av::{self, capture::device::{self, Device} }};
    ///
    /// let device_type = device::Type::built_in_wide_angle_camera();
    /// let media_type = av::MediaType::video();
    /// let position = device::Position::Front;
    /// let mut device = Device::with_device_type_media_and_position(device_type, Some(media_type), position).expect("device");
    /// device.unique_id().show();
    /// assert!(device.formats().len() > 0);
    /// assert!(device.supports_preset(av::CaptureSessionPreset::photo()));
    /// let mut lock = device.configuration_lock().expect("locked");
    ///
    /// ```
    pub fn with_device_type_media_and_position<'a>(
        device_type: &Type,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> Option<Retained<'a, Self>> {
        unsafe {
            AVCaptureDevice_defaultDeviceWithDeviceType_mediaType_position(
                device_type,
                media_type,
                position,
            )
        }
    }

    pub fn unique_id(&self) -> &cf::String {
        unsafe { rsel_uniqueID(self) }
    }

    pub fn formats(&self) -> &cf::ArrayOf<Format> {
        unsafe { rsel_formats(self) }
    }

    pub fn supports_preset(&self, preset: &SessionPreset) -> bool {
        unsafe { rsel_supportsAVCaptureSessionPreset(self, preset) }
    }

    pub fn configuration_lock(&mut self) -> Result<ConfigurationLockGuard, Retained<cf::Error>> {
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

    pub unsafe fn lock_for_configuration<'a>(
        &mut self,
        error: &mut Option<Retained<'a, cf::Error>>,
    ) -> bool {
        rsel_lockForConfiguration(self, error)
    }

    pub unsafe fn unlock_for_configuration(&mut self) {
        wsel_unlockForConfiguration(self)
    }
}

pub struct ConfigurationLockGuard<'a> {
    device: &'a mut Device,
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
    fn AVCaptureDevice_defaultDeviceWithDeviceType_mediaType_position<'a>(
        device_type: &Type,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> Option<Retained<'a, Device>>;

    fn rsel_lockForConfiguration<'a>(
        device: &mut Device,
        error: &mut Option<Retained<'a, cf::Error>>,
    ) -> bool;
    fn wsel_unlockForConfiguration(device: &mut Device);

    fn rsel_uniqueID(device: &Device) -> &cf::String;

    fn rsel_supportsAVCaptureSessionPreset(device: &Device, preset: &SessionPreset) -> bool;
    fn rsel_formats(device: &Device) -> &cf::ArrayOf<Format>;
}
