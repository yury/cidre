use crate::cf;


pub type CaptureSessionPreset = cf::String;

impl CaptureSessionPreset {

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
    static AVCaptureSessionPresetPhoto: &'static CaptureSessionPreset;
    static AVCaptureSessionPresetHigh: &'static CaptureSessionPreset;
    static AVCaptureSessionPresetMedium: &'static CaptureSessionPreset;
    static AVCaptureSessionPresetLow: &'static CaptureSessionPreset;
    static AVCaptureSessionPreset320x240: &'static CaptureSessionPreset;
    static AVCaptureSessionPreset352x288: &'static CaptureSessionPreset;
    static AVCaptureSessionPreset640x480: &'static CaptureSessionPreset;
    static AVCaptureSessionPreset960x540: &'static CaptureSessionPreset;
    static AVCaptureSessionPreset1280x720: &'static CaptureSessionPreset;
    static AVCaptureSessionPreset1920x1080: &'static CaptureSessionPreset;
    static AVCaptureSessionPreset3840x2160: &'static CaptureSessionPreset;
    static AVCaptureSessionPresetiFrame960x540: &'static CaptureSessionPreset;
    static AVCaptureSessionPresetiFrame1280x720: &'static CaptureSessionPreset;
    static AVCaptureSessionPresetInputPriority: &'static CaptureSessionPreset;
}