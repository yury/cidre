use crate::{define_obj_type, ns};

define_obj_type!(Buffer(ns::Id));

/// A buffer of audio data, with a format.
impl Buffer {}

define_obj_type!(PCMBuffer(Buffer));

/// Provides a number of methods useful for manipulating buffers of
/// audio in PCM format.
impl PCMBuffer {}

define_obj_type!(CompressedBuffer(ns::Id));

/// Use with compressed audio formats.
impl CompressedBuffer {}
