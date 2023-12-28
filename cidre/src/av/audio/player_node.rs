use std::ffi::c_void;

use crate::{
    arc,
    av::{self, audio},
    define_obj_type, objc,
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum CompletionCbType {
    DataConsumed = 0,
    DataRendered = 1,
    DataPlayedBack = 2,
}

define_obj_type!(pub PlayerNode(audio::Node), AV_AUDIO_PLAYER_NODE);

impl PlayerNode {
    #[objc::msg_send(scheduleBuffer:completionHandler:)]
    pub unsafe fn _schedule_buf_ch(&self, buffer: &av::AudioPcmBuf, handler: *mut c_void);

    /// Schedule playing samples from an [`av::AudioPCMBuf`].
    ///
    /// Schedules the buffer to be played following any previously scheduled commands.
    #[inline]
    pub fn schedule_buf_no_ch(&self, buffer: &av::AudioPcmBuf) {
        unsafe { self._schedule_buf_ch(buffer, std::ptr::null_mut()) }
    }

    #[objc::msg_send(play)]
    pub fn play(&self);

    #[objc::msg_send(stop)]
    pub fn stop(&self);

    #[objc::msg_send(pause)]
    pub fn pause(&self);

    #[objc::msg_send(isPlaying)]
    pub fn is_playing(&self) -> bool;

    // - Mixing

    #[objc::msg_send(volume)]
    pub fn volume(&self) -> f32;

    #[objc::msg_send(setVolume:)]
    pub fn set_volume(&self, val: f32);

    #[objc::msg_send(pan)]
    pub fn pan(&self) -> f32;

    #[objc::msg_send(setPan:)]
    pub fn set_pan(&self, val: f32);
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_PLAYER_NODE: &'static objc::Class<PlayerNode>;
}
