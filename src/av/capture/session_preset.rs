use crate::{cf, define_cf_type};

define_cf_type!(SessionPreset(cf::String));

impl SessionPreset {
    #[inline]
    pub fn photo() -> &'static Self {
        unsafe { AVCaptureSessionPresetPhoto }
    }

    #[inline]
    pub fn high() -> &'static Self {
        unsafe { AVCaptureSessionPresetHigh }
    }

    #[inline]
    pub fn medium() -> &'static Self {
        unsafe { AVCaptureSessionPresetMedium }
    }

    #[inline]
    pub fn low() -> &'static Self {
        unsafe { AVCaptureSessionPresetLow }
    }

    #[inline]
    pub fn _320_240() -> &'static Self {
        unsafe { AVCaptureSessionPreset320x240 }
    }

    #[inline]
    pub fn _352x288() -> &'static Self {
        unsafe { AVCaptureSessionPreset352x288 }
    }

    #[inline]
    pub fn _640x480() -> &'static Self {
        unsafe { AVCaptureSessionPreset640x480 }
    }

    #[inline]
    pub fn _960x540() -> &'static Self {
        unsafe { AVCaptureSessionPreset960x540 }
    }

    #[inline]
    pub fn _1280x720() -> &'static Self {
        unsafe { AVCaptureSessionPreset1280x720 }
    }

    #[inline]
    pub fn _1920x1080() -> &'static Self {
        unsafe { AVCaptureSessionPreset1920x1080 }
    }

    #[inline]
    pub fn _3840x2160() -> &'static Self {
        unsafe { AVCaptureSessionPreset3840x2160 }
    }

    #[inline]
    pub fn iframe_960x540() -> &'static Self {
        unsafe { AVCaptureSessionPresetiFrame960x540 }
    }

    #[inline]
    pub fn iframe_1280x720() -> &'static Self {
        unsafe { AVCaptureSessionPresetiFrame1280x720 }
    }

    #[inline]
    pub fn input_priority() -> &'static Self {
        unsafe { AVCaptureSessionPresetInputPriority }
    }
}

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVCaptureSessionPresetPhoto: &'static SessionPreset;
    static AVCaptureSessionPresetHigh: &'static SessionPreset;
    static AVCaptureSessionPresetMedium: &'static SessionPreset;
    static AVCaptureSessionPresetLow: &'static SessionPreset;
    static AVCaptureSessionPreset320x240: &'static SessionPreset;
    static AVCaptureSessionPreset352x288: &'static SessionPreset;
    static AVCaptureSessionPreset640x480: &'static SessionPreset;
    static AVCaptureSessionPreset960x540: &'static SessionPreset;
    static AVCaptureSessionPreset1280x720: &'static SessionPreset;
    static AVCaptureSessionPreset1920x1080: &'static SessionPreset;
    static AVCaptureSessionPreset3840x2160: &'static SessionPreset;
    static AVCaptureSessionPresetiFrame960x540: &'static SessionPreset;
    static AVCaptureSessionPresetiFrame1280x720: &'static SessionPreset;
    static AVCaptureSessionPresetInputPriority: &'static SessionPreset;
}
