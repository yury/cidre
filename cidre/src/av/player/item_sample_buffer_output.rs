use crate::{api, arc, av, cm, define_obj_type, dispatch, ns, objc};

#[objc::protocol(AVPlayerItemSampleBufferOutputDelegate)]
pub trait ItemSampleBufOutputDelegate: objc::Obj {
    #[objc::optional]
    #[objc::msg_send(outputMediaDataAvailable:)]
    fn output_media_data_available(&mut self, output: &mut ItemSampleBufOutput);

    /// Note that delivery of this message may race with calls to `next_buf`
    #[objc::optional]
    #[objc::msg_send(outputSequenceWasRestarted:)]
    fn output_sequence_was_restarted(&mut self, output: &mut ItemSampleBufOutput);
}

define_obj_type!(
    pub AnyItemSampleBufOutputDelegate(ns::Id)
);

impl ItemSampleBufOutputDelegate for AnyItemSampleBufOutputDelegate {}

define_obj_type!(
    #[doc(alias = "AVPlayerItemSampleBufferOutputConfiguration")]
    pub ItemSampleBufOutputCfg(ns::Id),
    AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT_CONFIGURATION,
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
);

define_obj_type!(
    #[doc(alias = "AVPlayerItemSampleBufferOutputAudioConfiguration")]
    pub ItemSampleBufOutputAudioCfg(ItemSampleBufOutputCfg),
    AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT_AUDIO_CONFIGURATION,
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
);

impl ItemSampleBufOutputAudioCfg {
    #[objc::msg_send(requestedAudioFormat)]
    pub fn requested_audio_format(&self) -> Option<&cm::FormatDesc>;

    /// Must be a PCM format.
    ///
    /// Specifying a PCM format is currently required.  In the future it may be optional.
    #[objc::msg_send(setRequestedAudioFormat:)]
    pub fn set_requested_audio_format(&mut self, val: Option<&cm::FormatDesc>);
}

define_obj_type!(
    #[doc(alias = "AVPlayerItemSampleBufferOutput")]
    pub ItemSampleBufOutput(av::player::ItemOutput),
    AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT,
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
);

unsafe impl Send for ItemSampleBufOutput {}
unsafe impl Sync for ItemSampleBufOutput {}

impl ItemSampleBufOutput {
    #[objc::init(initWithConfiguration:)]
    pub fn init_with_cfg(self, cfg: Option<&ItemSampleBufOutputCfg>)
    -> arc::R<ItemSampleBufOutput>;

    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn with_cfg(cfg: Option<&ItemSampleBufOutputCfg>) -> arc::R<Self> {
        Self::alloc().init_with_cfg(cfg)
    }

    #[objc::msg_send(copyNextSampleBuffer)]
    pub fn next_buf(&self) -> Option<arc::R<cm::SampleBuf>>;

    #[objc::msg_send(setDelegate:queue:)]
    pub fn set_delegate<D: ItemSampleBufOutputDelegate>(
        &mut self,
        val: Option<&D>,
        queue: Option<&dispatch::Queue>,
    );

    #[objc::msg_send(delegate)]
    pub fn delegate(&self) -> Option<arc::R<AnyItemSampleBufOutputDelegate>>;

    #[objc::msg_send(delegateQueue)]
    pub fn delegate_queue(&self) -> Option<arc::R<dispatch::Queue>>;
}

unsafe extern "C" {
    static AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT: &'static objc::Class<ItemSampleBufOutput>;
    static AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT_CONFIGURATION:
        &'static objc::Class<ItemSampleBufOutputCfg>;
    static AV_PLAYER_ITEM_SAMPLE_BUFFER_OUTPUT_AUDIO_CONFIGURATION:
        &'static objc::Class<ItemSampleBufOutputAudioCfg>;

}
