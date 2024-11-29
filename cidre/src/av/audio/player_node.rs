use crate::{
    arc,
    av::{self, audio},
    blocks, define_obj_type, objc,
};

/// Options controlling buffer scheduling.
#[doc(alias = "AVAudioPlayerNodeBufferOptions")]
#[repr(transparent)]
pub struct BufOpts(pub usize);

impl BufOpts {
    /// The buffer loops indefinitely.  
    pub const LOOPS: Self = Self(1usize << 0);
    /// The buffer interrupts any buffer already playing.
    pub const INTERRUPTS: Self = Self(1usize << 1);
    /// The buffer interrupts any buffer already playing, at its loop point.
    pub const INTERRUPTS_AT_LOOP: Self = Self(1usize << 2);
}

#[doc(alias = "AVAudioPlayerNodeCompletionCallbackType")]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum CompletionCbType {
    #[doc(alias = "AVAudioPlayerNodeCompletionDataConsumed")]
    DataConsumed = 0,
    #[doc(alias = "AVAudioPlayerNodeCompletionDataRendered")]
    DataRendered = 1,
    #[doc(alias = "AVAudioPlayerNodeCompletionDataPlayedBack")]
    DataPlayedBack = 2,
}

define_obj_type!(
    #[doc(alias = "AVAudioPlayerNode")]
    pub PlayerNode(audio::Node),
    AV_AUDIO_PLAYER_NODE
);

impl PlayerNode {
    /// Schedule playing samples from an [`av::AudioPcmBuf`].
    ///
    /// Schedules the buffer to be played following any previously scheduled commands.
    #[objc::msg_send(scheduleBuffer:completionHandler:)]
    pub fn schedule_buf_ch_block(
        &mut self,
        buffer: &av::AudioPcmBuf,
        handler: Option<&mut audio::NodeCh<blocks::Esc>>,
    );

    pub fn schedule_buf_ch(&mut self, buffer: &av::AudioPcmBuf, handler: impl FnMut() + 'static) {
        let mut block = blocks::EscBlock::new0(handler);
        self.schedule_buf_ch_block(buffer, Some(&mut block));
    }

    pub async fn schedule_buf(&mut self, buffer: &av::AudioPcmBuf) {
        let (future, mut block) = blocks::comp0();
        self.schedule_buf_ch_block(buffer, Some(&mut block));
        future.await
    }

    #[objc::msg_send(play)]
    pub fn play(&self);

    #[objc::msg_send(stop)]
    pub fn stop(&self);

    #[objc::msg_send(pause)]
    pub fn pause(&self);

    #[objc::msg_send(isPlaying)]
    pub fn is_playing(&self) -> bool;
}

/// Mixing
impl PlayerNode {
    #[objc::msg_send(volume)]
    pub fn volume(&self) -> f32;

    #[objc::msg_send(setVolume:)]
    pub fn set_volume(&mut self, val: f32);

    #[objc::msg_send(pan)]
    pub fn pan(&self) -> f32;

    #[objc::msg_send(setPan:)]
    pub fn set_pan(&mut self, val: f32);
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_PLAYER_NODE: &'static objc::Class<PlayerNode>;
}
