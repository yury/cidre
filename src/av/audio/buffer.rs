use crate::{at::AudioBufferList, cf, define_obj_type, ns};

use super::Format;

define_obj_type!(Buffer(ns::Id));

/// A buffer of audio data, with a format.
impl Buffer {
    pub fn format(&self) -> cf::Retained<Format> {
        unsafe { rsel_format(self) }
    }

    pub fn audio_buffer_list(&self) -> &AudioBufferList<1, 1> {
        unsafe { rsel_audioBufferList(self) }
    }

    pub fn audio_buffer_list_mut(&self) -> &mut AudioBufferList<1, 1> {
        unsafe { rsel_mutableAudioBufferList(self) }
    }
}

define_obj_type!(PCMBuffer(Buffer));

/// Provides a number of methods useful for manipulating buffers of
/// audio in PCM format.
impl PCMBuffer {
    
}

define_obj_type!(CompressedBuffer(ns::Id));

/// Use with compressed audio formats.
impl CompressedBuffer {}

#[link(name = "av", kind = "static")]
extern "C" {
    fn rsel_format(id: &ns::Id) -> cf::Retained<Format>;
    fn rsel_audioBufferList(id: &ns::Id) -> &AudioBufferList<1, 1>;
    fn rsel_mutableAudioBufferList(id: &ns::Id) -> &mut AudioBufferList<1, 1>;
}
