use crate::{define_obj_type, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum SpeechBoundery {
    /// Indicates to pause or stop speech immediately.
    Immediate,

    /// Indicates to pause or stop speech after the synthesizer finishes speaking the current word.
    Word,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum VoiceQuality {
    /// The basic quality version of a voice that’s on the device by default.
    Default = 1,
    /// The enhanced quality version of a voice that the user must download.
    Enhanced,
    ///
    Premium,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum VoiceGender {
    /// The nonspecific gender option.
    Unspecified,
    /// The male voice option.
    Male,
    /// The female voice option.
    Female,
}

/// Markers used in the output event callback. Used for providing metadata on synthesized audio.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum MarkerMark {
    /// A type of text that represents a phoneme.
    Phoneme,
    /// A type of text that represents a word.
    Word,
    /// A type of text that represents a sentence.
    Sentence,
    /// A type of text that represents a paragraph.
    Paragraph,
}

define_obj_type!(Voice(objc::Id));

define_obj_type!(Utterance(objc::Id));

define_obj_type!(Synthesizer(objc::Id));

define_obj_type!(Marker(objc::Id));
