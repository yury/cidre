#[cfg(feature = "av")]
use crate::{arc, av};
use crate::{cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "ARVideoFormat")]
    /// Video stream format used by an AR configuration.
    pub VideoFormat(ns::Id)
);

impl VideoFormat {
    /// Physical position of the capture device on the system.
    #[cfg(feature = "av")]
    #[objc::msg_send(captureDevicePosition)]
    #[objc::available(ios = 13.0)]
    pub fn capture_device_position(&self) -> av::CaptureDevicePos;

    /// Capture device type backing this AR video format.
    #[cfg(feature = "av")]
    #[objc::msg_send(captureDeviceType)]
    #[objc::available(ios = 14.5)]
    pub fn capture_device_type(&self) -> arc::R<av::CaptureDeviceType>;

    /// Image resolution in pixels.
    #[objc::msg_send(imageResolution)]
    pub fn image_resolution(&self) -> cg::Size;

    /// Streaming frame rate.
    #[objc::msg_send(framesPerSecond)]
    pub fn frames_per_second(&self) -> ns::Integer;

    /// Whether this format is recommended for high-resolution frame capture.
    #[objc::msg_send(isRecommendedForHighResolutionFrameCapturing)]
    #[objc::available(ios = 16.0)]
    pub fn is_recommended_for_high_resolution_frame_capturing(&self) -> bool;

    /// Whether HDR streaming is supported by this format.
    #[objc::msg_send(isVideoHDRSupported)]
    #[objc::available(ios = 16.0)]
    pub fn is_video_hdr_supported(&self) -> bool;

    /// Default capture color space used by ARKit for this format.
    #[cfg(feature = "av")]
    #[objc::msg_send(defaultColorSpace)]
    #[objc::available(ios = 26.0)]
    pub fn default_color_space(&self) -> av::CaptureColorSpace;

    /// Default photo settings used when capturing high-resolution frames.
    #[cfg(feature = "av")]
    #[objc::msg_send(defaultPhotoSettings)]
    #[objc::available(ios = 26.0)]
    pub fn default_photo_settings(&self) -> arc::R<av::capture::PhotoSettings>;
}
