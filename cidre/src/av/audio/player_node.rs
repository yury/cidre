use std::ffi::c_void;

use crate::{
    av::{self, audio},
    define_obj_type, msg_send,
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

define_obj_type!(PlayerNode(audio::Node));

impl PlayerNode {
    #[inline]
    pub unsafe fn schedule_buffer_completion_hander(
        &self,
        buffer: &av::AudioPCMBuffer,
        handler: *mut c_void,
    ) {
        msg_send!(
            "av",
            self,
            sel_scheduleBuffer_completionHandler,
            buffer,
            handler
        )
    }

    /// Schedule playing samples from an av::AudioPCMBuffer.
    ///
    /// Schedules the buffer to be played following any previously scheduled commands.
    #[inline]
    pub fn schedule_buffer(&self, buffer: &av::AudioPCMBuffer) {
        unsafe { self.schedule_buffer_completion_hander(buffer, std::ptr::null_mut()) }
    }

    #[inline]
    pub fn play(&self) {
        msg_send!("av", self, sel_play)
    }

    #[inline]
    pub fn stop(&self) {
        msg_send!("av", self, sel_stop)
    }

    #[inline]
    pub fn pause(&self) {
        msg_send!("av", self, sel_pause)
    }

    #[inline]
    pub fn is_playing(&self) -> bool {
        msg_send!("av", self, sel_isPlaying)
    }

    // - Mixing

    #[inline]
    pub fn volume(&self) -> f32 {
        msg_send!("av", self, sel_volume)
    }

    #[inline]
    pub fn set_volume(&self, value: f32) {
        msg_send!("av", self, sel_setVolume, value)
    }

    #[inline]
    pub fn pan(&self) -> f32 {
        msg_send!("av", self, sel_pan)
    }

    #[inline]
    pub fn set_pan(&self, value: f32) {
        msg_send!("av", self, sel_setPan, value)
    }
}
