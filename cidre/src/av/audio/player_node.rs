use std::ffi::c_void;

use crate::{
    arc,
    av::{self, audio},
    define_obj_type, objc,
};

/// Options controlling buffer scheduling.
#[repr(transparent)]
pub struct BufferOptions(pub usize);

impl BufferOptions {
    /// The buffer loops indefinitely.  
    pub const LOOPS: Self = Self(1usize << 0);
    /// The buffer interrupts any buffer already playing.
    pub const INTERRUPTS: Self = Self(1usize << 1);
    /// he buffer interrupts any buffer already playing, at its loop point.
    pub const INTERRUPTS_AT_LOOP: Self = Self(1usize << 2);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum CompletionCallbackType {
    DataConsumed = 0,
    DataRendered = 1,
    DataPlayedBack = 2,
}

define_obj_type!(PlayerNode(audio::Node), AV_AUDIO_PLAYER_NODE);

impl PlayerNode {
    #[objc::msg_send(scheduleBuffer:completionHandler:)]
    pub unsafe fn schedule_buffer_completion_hander(
        &self,
        buffer: &av::AudioPCMBuffer,
        handler: *mut c_void,
    );

    /// Schedule playing samples from an av::AudioPCMBuffer.
    ///
    /// Schedules the buffer to be played following any previously scheduled commands.
    #[inline]
    pub fn schedule_buffer(&self, buffer: &av::AudioPCMBuffer) {
        unsafe { self.schedule_buffer_completion_hander(buffer, std::ptr::null_mut()) }
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
    pub fn set_volume(&self, value: f32);

    #[objc::msg_send(pan)]
    pub fn pan(&self) -> f32;

    #[objc::msg_send(setPan:)]
    pub fn set_pan(&self, value: f32);
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_AUDIO_PLAYER_NODE: &'static objc::Class<PlayerNode>;
}
