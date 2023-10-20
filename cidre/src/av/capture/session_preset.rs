use crate::{define_obj_type, ns};

define_obj_type!(SessionPreset(ns::String));

impl SessionPreset {
    /// preset suitable for high resolution photo quality output.
    #[inline]
    pub fn photo() -> &'static Self {
        unsafe { AVCaptureSessionPresetPhoto }
    }

    /// preset suitable for high quality video and audio output.
    #[inline]
    pub fn high() -> &'static Self {
        unsafe { AVCaptureSessionPresetHigh }
    }

    /// Clients may set an av::CaptureSession instance's sessionPreset to
    /// `av::CaptureSessionPreset::medium()` to achieve output video and audio
    /// bitrates suitable for sharing over WiFi.
    #[inline]
    pub fn medium() -> &'static Self {
        unsafe { AVCaptureSessionPresetMedium }
    }

    /// An `av::CaptureSession` preset suitable for low quality output.
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

    /// An 'av::CaptureSession' preset suitable for 3840x2160 (UHD 4K) video output.
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

    /// By calling `set_session_preset()`, clients can easily configure an 'av::CaptureSession'
    /// to produce a desired quality of service level. The session configures its inputs and outputs
    /// optimally to produce the QoS level indicated. Clients who need to ensure a particular
    /// input format is chosen can use 'av::CaptureDevice''s 'set_active_format()' method.
    /// When a client sets the active format on a device, the associated session's 'session_preset'
    /// property automatically changes to 'av::CaptureSessionPreset::input_priority()'.
    /// This change indicates that the input format selected by the client now dictates the quality
    /// of service level provided at the outputs. When a client sets the session preset to anything
    /// other than 'av::CaptureSessionPreset::input_priority()', the session resumes responsibility
    /// for configuring inputs and outputs, and is free to change its inputs' 'active_format' as needed.
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
