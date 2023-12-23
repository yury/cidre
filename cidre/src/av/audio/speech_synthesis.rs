use crate::{arc, define_obj_type, define_options, ns, objc};

#[doc(alias = "AVSpeechBoundary")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum SpeechBoundery {
    /// Indicates to pause or stop speech immediately.
    Immediate,

    /// Indicates to pause or stop speech after the synthesizer finishes speaking the current word.
    Word,
}

#[doc(alias = "AVSpeechSynthesisVoiceQuality")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum VoiceQuality {
    /// New voice return 0 as voice quality (see tests)
    Unknown = 0,
    /// The basic quality version of a voice that’s on the device by default.
    Default = 1,
    /// The enhanced quality version of a voice that the user must download.
    Enhanced,
    /// The enhanced quality version of a voice that the user must download.
    Premium,
}

#[doc(alias = "AVSpeechSynthesisVoiceGender")]
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
#[doc(alias = "AVSpeechSynthesisMarkerMark")]
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

define_options!(
    #[doc(alias = "AVSpeechSynthesisVoiceTraits")]
    pub VoiceTraits(usize)
);

impl VoiceTraits {
    pub const NONE: Self = Self(0);

    /// The voice is generally for novelty purposes, for example a character's voice in a game.
    pub const IS_NOVELTY_VOICE: Self = Self(1 << 0);

    pub const IS_PERSONAL_VOICE: Self = Self(1 << 1);
}

define_obj_type!(
    #[doc(alias = "AVSpeechSynthesisVoice")]
    pub Voice(ns::Id),
    AV_SPEECH_SYNTHESIS_VOICE
);

impl Voice {
    #[objc::cls_msg_send(speechVoices)]
    pub fn speech_voices_ar() -> arc::Rar<ns::Array<Self>>;

    #[objc::cls_rar_retain]
    pub fn speech_voices() -> arc::R<ns::Array<Self>>;

    #[objc::cls_msg_send(currentLanguageCode)]
    pub fn current_lang_code_ar() -> arc::Rar<ns::String>;

    #[objc::cls_rar_retain]
    pub fn current_lang_code() -> arc::R<ns::String>;

    #[objc::msg_send(language)]
    pub fn lang(&self) -> Option<&ns::String>;

    #[objc::msg_send(identifier)]
    pub fn id(&self) -> &ns::String;

    #[objc::msg_send(name)]
    pub fn name(&self) -> &ns::String;

    #[objc::msg_send(quality)]
    pub fn quality(&self) -> VoiceQuality;

    #[objc::msg_send(gender)]
    pub fn gender(&self) -> VoiceGender;

    #[objc::msg_send(audioFileSettings)]
    pub fn audio_file_settings(&self) -> Option<&ns::Dictionary<ns::String, ns::Id>>;
}

define_obj_type!(pub Utterance(ns::Id));

impl Utterance {
    pub fn min_speech_rate() -> f32 {
        unsafe { AVSpeechUtteranceMinimumSpeechRate }
    }

    pub fn max_speech_rate() -> f32 {
        unsafe { AVSpeechUtteranceMaximumSpeechRate }
    }

    pub fn default_speech_rate() -> f32 {
        unsafe { AVSpeechUtteranceDefaultSpeechRate }
    }
}

define_obj_type!(pub Synthesizer(ns::Id));

define_obj_type!(pub Marker(ns::Id));

#[link(name = "AVFAudio", kind = "framework")]
extern "C" {
    static AVSpeechUtteranceMinimumSpeechRate: f32;
    static AVSpeechUtteranceMaximumSpeechRate: f32;
    static AVSpeechUtteranceDefaultSpeechRate: f32;

    static AV_SPEECH_SYNTHESIS_VOICE: &'static objc::Class<Voice>;
}

#[cfg(test)]
mod tests {
    use crate::av;

    #[test]
    fn basics() {
        assert!(
            av::SpeechUtterance::min_speech_rate() < av::SpeechUtterance::default_speech_rate()
        );
        assert!(
            av::SpeechUtterance::default_speech_rate() < av::SpeechUtterance::max_speech_rate()
        );

        let voice = av::SpeechSynthesisVoice::new();
        voice.as_type_ref().show();
        assert!(voice.id().is_empty());
        assert!(voice.name().is_empty());
        assert!(voice.lang().is_none());
        assert_eq!(voice.quality(), av::SpeechSynthesisVoiceQuality::Unknown);
        assert!(av::SpeechSynthesisVoice::speech_voices().len() > 0);
        let settings = voice.audio_file_settings();
        assert!(settings.is_none());
    }
}
