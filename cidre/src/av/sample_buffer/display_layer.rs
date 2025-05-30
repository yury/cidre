use crate::{arc, av, ca, define_obj_type, objc};

define_obj_type!(
    #[doc(alias = "AVSampleBufferDisplayLayer")]
    pub DisplayLayer(ca::Layer),
    AV_DISPLAY_LAYER
);

#[cfg(feature = "cm")]
use crate::cm;

#[cfg(feature = "cm")]
impl DisplayLayer {
    #[objc::msg_send(controlTimebase)]
    pub fn control_timebase(&self) -> Option<&cm::Timebase>;

    #[objc::msg_send(setControlTimebase:)]
    pub fn set_control_timebase(&self, value: Option<&cm::Timebase>);
}

impl DisplayLayer {
    #[objc::msg_send(videoGravity)]
    pub fn video_gravity(&self) -> arc::R<av::LayerVideoGravity>;

    #[objc::msg_send(setVideoGravity:)]
    pub fn set_video_gravity(&mut self, value: &av::LayerVideoGravity);

    /// Indicates that image data should be protected from capture.
    #[objc::msg_send(preventsCapture)]
    pub fn prevents_capture(&self) -> bool;

    #[objc::msg_send(setPreventsCapture:)]
    pub fn set_prevents_capture(&self, value: bool);

    #[objc::msg_send(preventsDisplaySleepDuringVideoPlayback)]
    pub fn prevents_display_sleep_during_video_playback(&self) -> bool;

    #[objc::msg_send(setPreventsDisplaySleepDuringVideoPlayback:)]
    pub fn set_prevents_display_sleep_during_video_playback(&self, value: bool);

    #[objc::msg_send(outputObscuredDueToInsufficientExternalProtection)]
    pub fn output_obscured_due_to_insufficient_external_protection(&self) -> bool;

    #[objc::msg_send(sampleBufferRenderer)]
    pub fn sample_buf_renderer(&self) -> arc::R<av::SampleBufVideoRenderer>;
}

#[link(name = "ca", kind = "static")]
unsafe extern "C" {
    static AV_DISPLAY_LAYER: &'static objc::Class<DisplayLayer>;
}
