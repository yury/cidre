use crate::{cf, define_obj_type, objc::Id};

define_obj_type!(Device(Id));

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
    Front = 2
}
