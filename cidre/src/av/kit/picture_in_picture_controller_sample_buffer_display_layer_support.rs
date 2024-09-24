use crate::{arc, av, blocks, cm, define_obj_type, ns, objc};

impl av::PipController {
    #[objc::msg_send(invalidatePlaybackState)]
    #[objc::available(macos = 12.0, ios = 15.0, tvos = 15.0, visionos = 1.0)]
    pub fn invalidate_playback_state(&mut self);
}

#[objc::protocol(AVPictureInPictureSampleBufferPlaybackDelegate)]
pub trait PipSampleBufPlayerDelegate: objc::Obj {
    #[objc::msg_send(pictureInPictureController:setPlaying:)]
    fn pip_controller_set_playing(&mut self, pip_controller: &mut av::PipController, val: bool);

    #[objc::msg_send(pictureInPictureControllerTimeRangeForPlayback:)]
    fn pip_controller_time_range_for_playback(
        &mut self,
        pip_controller: &mut av::PipController,
    ) -> cm::TimeRange;

    #[objc::msg_send(pictureInPictureControllerIsPlaybackPaused:)]
    fn pip_controller_is_playback_paused(&mut self, pip_controller: &mut av::PipController)
        -> bool;

    #[objc::msg_send(pictureInPictureController:didTransitionToRenderSize:)]
    fn pip_controller_did_transition_to_render_size(
        &mut self,
        pip_controller: &mut av::PipController,
        new_render_size: cm::VideoDimensions,
    );

    #[objc::msg_send(pictureInPictureController:skipByInterval:completionHandler:)]
    fn pip_controller_skip_by_interval_ch(
        &mut self,
        pip_controller: &mut av::PipController,
        skip_by_interval: cm::Time,
        ch: &mut blocks::EscBlock<fn()>,
    );

    #[objc::optional]
    #[objc::msg_send(pictureInPictureControllerShouldProhibitBackgroundAudioPlayback)]
    fn pip_controller_should_prohibit_bg_audio_playback(&mut self) -> bool;
}

define_obj_type!(
    pub AnyPipSampleBufPlayerDelegate(ns::Id)
);

impl arc::A<av::PipControllerContentSrc> {
    #[objc::msg_send(initWithSampleBufferDisplayLayer:playbackDelegate:)]
    pub fn init_with_sample_buf_display_layer<D: PipSampleBufPlayerDelegate>(
        self,
        sbuf_display_layer: &av::SampleBufDisplayLayer,
        playback_delegate: &D,
    ) -> arc::R<av::PipControllerContentSrc>;
}

/// AVSampleBufferDisplayLayerSupport
impl av::PipControllerContentSrc {
    pub fn with_sample_buf_display_layer<D: PipSampleBufPlayerDelegate>(
        sbuf_display_layer: &av::SampleBufDisplayLayer,
        playback_delegate: &D,
    ) -> arc::R<Self> {
        Self::alloc().init_with_sample_buf_display_layer(sbuf_display_layer, playback_delegate)
    }

    /// The receiver's sample buffer display layer.
    #[objc::msg_send(sampleBufferDisplayLayer)]
    pub fn sample_buf_display_layer(&self) -> Option<arc::R<av::SampleBufDisplayLayer>>;

    /// The receiver's sample buffer playback delegate.
    #[objc::msg_send(sampleBufferPlaybackDelegate)]
    pub fn sample_buf_playback_delegate(&self) -> Option<arc::R<AnyPipSampleBufPlayerDelegate>>;
}
