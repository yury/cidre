#[cfg(feature = "blocks")]
use crate::blocks;
use crate::{
    arc,
    av::{self, AuthorizationStatus},
    define_obj_type, define_opts, ns,
    objc::{self, Obj},
};

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

define_opts!(
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
    sym AVSpeechSynthesisVoice
);

impl Voice {
    #[objc::msg_send(speechVoices)]
    pub fn voices() -> arc::R<ns::Array<Self>>;

    #[objc::msg_send(currentLanguageCode)]
    pub fn current_lang_code() -> arc::R<ns::String>;

    #[objc::msg_send(language)]
    pub fn lang(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(identifier)]
    pub fn id(&self) -> arc::R<ns::String>;

    #[objc::msg_send(name)]
    pub fn name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(quality)]
    pub fn quality(&self) -> VoiceQuality;

    #[objc::msg_send(gender)]
    pub fn gender(&self) -> VoiceGender;

    #[objc::msg_send(audioFileSettings)]
    pub fn audio_file_settings(&self) -> Option<arc::R<ns::Dictionary<ns::String, ns::Id>>>;

    #[objc::msg_send(voiceTraits)]
    pub fn traits(&self) -> VoiceTraits;

    #[objc::msg_send(voiceWithLanguage:)]
    pub fn with_lang(lang_code: Option<&ns::String>) -> Option<arc::R<Self>>;

    #[objc::msg_send(voiceWithIdentifier:)]
    pub fn with_id(identifier: &ns::String) -> Option<arc::R<Self>>;

    /// The identifier of the Alex voice.
    #[doc(alias = "AVSpeechSynthesisVoiceIdentifierAlex")]
    #[inline]
    pub fn id_alex() -> &'static ns::String {
        unsafe { AVSpeechSynthesisVoiceIdentifierAlex }
    }
}

impl ns::AttrStringKey {
    /// An attribute key with an IPA pronunciation string for the range it covers.
    #[doc(alias = "AVSpeechSynthesisIPANotationAttribute")]
    #[inline]
    pub fn speech_synthesis_ipa_notation() -> &'static Self {
        unsafe { AVSpeechSynthesisIPANotationAttribute }
    }
}

impl ns::NotificationName {
    /// Posted when the voices available change: one is downloaded, deleted, or a personal voice
    /// is made.
    #[doc(alias = "AVSpeechSynthesisAvailableVoicesDidChangeNotification")]
    #[inline]
    pub fn speech_synthesis_available_voices_did_change() -> &'static Self {
        unsafe { AVSpeechSynthesisAvailableVoicesDidChangeNotification }
    }
}

define_obj_type!(
    #[doc(alias = "AVSpeechUtterance")]
    pub Utterance(ns::Id),
    sym AVSpeechUtterance
);

impl Utterance {
    #[objc::init(initWithString:)]
    pub fn init_with_string(self, val: &ns::String) -> arc::R<Utterance>;

    #[objc::init(initWithAttributedString:)]
    pub fn init_with_attr_string(self, val: &ns::AttrString) -> arc::R<Utterance>;

    #[objc::init(initWithSSMLRepresentation:)]
    pub fn init_with_ssml(self, val: &ns::String) -> arc::R<Utterance>;

    pub fn min_speech_rate() -> f32 {
        unsafe { AVSpeechUtteranceMinimumSpeechRate }
    }

    pub fn max_speech_rate() -> f32 {
        unsafe { AVSpeechUtteranceMaximumSpeechRate }
    }

    pub fn default_speech_rate() -> f32 {
        unsafe { AVSpeechUtteranceDefaultSpeechRate }
    }

    #[inline]
    pub fn with_string(val: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_string(val)
    }

    #[inline]
    pub fn with_attr_string(val: &ns::AttrString) -> arc::R<Self> {
        Self::alloc().init_with_attr_string(val)
    }

    #[inline]
    pub fn with_ssml(val: &ns::String) -> arc::R<Self> {
        Self::alloc().init_with_ssml(val)
    }

    #[objc::msg_send(voice)]
    pub fn voice(&self) -> Option<arc::R<Voice>>;

    #[objc::msg_send(setVoice:)]
    pub fn set_voice(&mut self, val: Option<&Voice>);

    #[objc::msg_send(speechString)]
    pub fn speech_string(&self) -> arc::R<ns::String>;

    #[objc::msg_send(attributedSpeechString)]
    pub fn speech_attr_string(&self) -> Option<arc::R<ns::AttrString>>;

    #[objc::msg_send(rate)]
    pub fn rate(&self) -> f32;

    #[objc::msg_send(setRate:)]
    pub fn set_rate(&mut self, val: f32);

    #[objc::msg_send(pitchMultiplier)]
    pub fn pitch_multiplier(&self) -> f32;

    #[objc::msg_send(setPitchMultiplier:)]
    pub fn set_pitch_multiplier(&mut self, val: f32);

    #[objc::msg_send(volume)]
    pub fn volume(&self) -> f32;

    #[objc::msg_send(setVolume:)]
    pub fn set_volume(&mut self, val: f32);

    #[objc::msg_send(prefersAssistiveTechnologySettings)]
    pub fn prefers_assistive_technology_settings(&self) -> bool;

    #[objc::msg_send(setPrefersAssistiveTechnologySettings:)]
    pub fn set_prefers_assistive_technology_settings(&mut self, val: bool);

    #[objc::msg_send(preUtteranceDelay)]
    pub fn pre_utterance_delay(&self) -> ns::TimeInterval;

    #[objc::msg_send(setPreUtteranceDelay:)]
    pub fn set_pre_utterance_delay(&mut self, val: ns::TimeInterval);

    #[objc::msg_send(postUtteranceDelay)]
    pub fn post_utterance_delay(&self) -> ns::TimeInterval;

    #[objc::msg_send(setPostUtteranceDelay:)]
    pub fn set_post_utterance_delay(&mut self, val: ns::TimeInterval);
}

#[objc::protocol(AVSpeechSynthesizerDelegate)]
pub trait Delegate: Obj {
    #[objc::optional]
    #[objc::msg_send(speechSynthesizer:didStartSpeechUtterance:)]
    fn speech_synthesizer_did_start_speech_utterance(
        &mut self,
        synthesizer: &Synthesizer,
        utterance: &Utterance,
    );

    #[objc::optional]
    #[objc::msg_send(speechSynthesizer:didFinishSpeechUtterance:)]
    fn speech_synthesizer_did_finish_speech_utterance(
        &mut self,
        synthesizer: &Synthesizer,
        utterance: &Utterance,
    );

    #[objc::optional]
    #[objc::msg_send(speechSynthesizer:didPauseSpeechUtterance:)]
    fn speech_synthesizer_did_pause_speech_utterance(
        &mut self,
        synthesizer: &Synthesizer,
        utterance: &Utterance,
    );

    #[objc::optional]
    #[objc::msg_send(speechSynthesizer:didContinueSpeechUtterance:)]
    fn speech_synthesizer_did_continue_speech_utterance(
        &mut self,
        synthesizer: &Synthesizer,
        utterance: &Utterance,
    );

    #[objc::optional]
    #[objc::msg_send(speechSynthesizer:didCancelSpeechUtterance:)]
    fn speech_synthesizer_did_cancel_speech_utterance(
        &mut self,
        synthesizer: &Synthesizer,
        utterance: &Utterance,
    );

    #[objc::optional]
    #[objc::msg_send(speechSynthesizer:willSpeakRangeOfSpeechString:utterance:)]
    fn speech_synthesizer_will_speak_range_of_speech_string(
        &mut self,
        synthesizer: &Synthesizer,
        characters_range: ns::Range,
        utterance: &Utterance,
    );

    #[objc::optional]
    #[objc::msg_send(speechSynthesizer:willSpeakMarker:utterance:)]
    fn speech_synthesizer_will_speak_marker(
        &mut self,
        synthesizer: &Synthesizer,
        marker: &Marker,
        utterance: &Utterance,
    );
}

define_obj_type!(
    #[doc(alias = "AVSpeechSynthesizer")]
    pub Synthesizer(ns::Id),
    sym AVSpeechSynthesizer
);

impl Synthesizer {
    #[objc::msg_send(setDelegate:)]
    pub fn set_delegate<D: Delegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(isSpeaking)]
    pub fn is_speaking(&self) -> bool;

    #[objc::msg_send(isPaused)]
    pub fn is_paused(&self) -> bool;

    /// Enqueing the same [`av::SpeechUtterance`] that is already enqueued or is speaking will
    /// raise an exception
    #[objc::msg_send(speakUtterance:)]
    pub unsafe fn speak_utterance_throws(&mut self, utterance: &Utterance);

    pub fn speak_utterance<'ear>(&mut self, utterance: &Utterance) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.speak_utterance_throws(utterance) })
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(writeUtterance:toBufferCallback:)]
    pub unsafe fn write_utterance_block_throws(
        &mut self,
        utterance: &Utterance,
        buffer_callback: &mut BufCallback,
    );

    /// Synthesizes `utterance` into audio buffers instead of playing it. `buffer_callback` gets
    /// the buffers (`av::AudioPcmBuf`s in the voice's format, 22.05 kHz mono f32 for the system
    /// voices) on the main queue whichever thread writes, and a last one with no frames when the
    /// utterance is done. They come faster than real time.
    ///
    /// Writing the same [`av::SpeechUtterance`] that is already enqueued or is speaking raises
    /// an exception.
    #[cfg(feature = "blocks")]
    pub fn write_utterance_block<'ear>(
        &mut self,
        utterance: &Utterance,
        buffer_callback: &mut BufCallback,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe { self.write_utterance_block_throws(utterance, buffer_callback) })
    }

    /// See [`Self::write_utterance_block`].
    #[cfg(feature = "blocks")]
    pub fn write_utterance<'ear>(
        &mut self,
        utterance: &Utterance,
        buffer_callback: impl FnMut(&av::AudioBuf) + Send + 'static,
    ) -> ns::ExResult<'ear> {
        let mut buffer_callback = BufCallback::new1(buffer_callback);
        self.write_utterance_block(utterance, &mut buffer_callback)
    }

    #[cfg(feature = "blocks")]
    #[objc::msg_send(writeUtterance:toBufferCallback:toMarkerCallback:)]
    pub unsafe fn write_utterance_with_markers_block_throws(
        &mut self,
        utterance: &Utterance,
        buffer_callback: &mut BufCallback,
        marker_callback: &mut MarkerCallback,
    );

    /// As [`Self::write_utterance_block`], with the utterance's markers (words, sentences,
    /// bookmarks) given to `marker_callback` as they are synthesized.
    #[cfg(feature = "blocks")]
    pub fn write_utterance_with_markers_block<'ear>(
        &mut self,
        utterance: &Utterance,
        buffer_callback: &mut BufCallback,
        marker_callback: &mut MarkerCallback,
    ) -> ns::ExResult<'ear> {
        ns::try_catch(|| unsafe {
            self.write_utterance_with_markers_block_throws(
                utterance,
                buffer_callback,
                marker_callback,
            )
        })
    }

    /// See [`Self::write_utterance_with_markers_block`].
    #[cfg(feature = "blocks")]
    pub fn write_utterance_with_markers<'ear>(
        &mut self,
        utterance: &Utterance,
        buffer_callback: impl FnMut(&av::AudioBuf) + Send + 'static,
        marker_callback: impl FnMut(&ns::Array<Marker>) + Send + 'static,
    ) -> ns::ExResult<'ear> {
        let mut buffer_callback = BufCallback::new1(buffer_callback);
        let mut marker_callback = MarkerCallback::new1(marker_callback);
        self.write_utterance_with_markers_block(
            utterance,
            &mut buffer_callback,
            &mut marker_callback,
        )
    }

    #[objc::msg_send(stopSpeakingAtBoundary:)]
    pub fn _stop_speaking_at_boundary(&mut self, boundery: SpeechBoundery) -> bool;

    pub fn stop_speaking_at_boundary(&mut self, boundery: SpeechBoundery) -> Result<(), ()> {
        if self._stop_speaking_at_boundary(boundery) {
            Ok(())
        } else {
            Err(())
        }
    }

    #[objc::msg_send(pauseSpeakingAtBoundary:)]
    pub fn _pause_speaking_at_boundary(&mut self, boundery: SpeechBoundery) -> bool;

    pub fn pause_speaking_at_boundary(&mut self, boundery: SpeechBoundery) -> Result<(), ()> {
        if self._pause_speaking_at_boundary(boundery) {
            Ok(())
        } else {
            Err(())
        }
    }

    #[objc::msg_send(continueSpeaking)]
    pub fn _continue_speaking(&mut self) -> bool;

    pub fn continue_speaking(&mut self) -> Result<(), ()> {
        if self._continue_speaking() {
            Ok(())
        } else {
            Err(())
        }
    }
    #[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
    #[objc::msg_send(outputChannels)]
    pub fn output_channels(&self) -> Option<arc::R<ns::Array<av::audio::session::ChannelDesc>>>;

    #[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
    #[objc::msg_send(setOutputChannels:)]
    pub fn set_output_channels(&mut self, val: Option<&ns::Array<av::audio::session::ChannelDesc>>);

    #[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
    #[objc::msg_send(usesApplicationAudioSession)]
    pub fn uses_app_audio_session(&self) -> bool;

    #[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
    #[objc::msg_send(setUsesApplicationAudioSession:)]
    pub fn set_uses_app_audio_session(&mut self, val: bool);

    #[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
    #[objc::msg_send(mixToTelephonyUplink)]
    pub fn mix_to_telephony_uplink(&self) -> bool;

    #[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
    #[objc::msg_send(setMixToTelephonyUplink:)]
    pub fn set_mix_to_telephony_uplink(&mut self, val: bool);

    #[objc::msg_send(personalVoiceAuthorizationStatus)]
    pub fn personal_voice_authorization_status() -> AuthorizationStatus;

    #[cfg(feature = "blocks")]
    #[objc::msg_send(requestPersonalVoiceAuthorizationWithCompletionHandler:)]
    pub fn request_personal_voice_authorization_ch_block(
        block: &mut blocks::SendBlock<fn(AuthorizationStatus)>,
    );

    /// Asks the user to let the app use their personal voices; `handler` gets the answer on a
    /// queue of the synthesizer's.
    #[cfg(feature = "blocks")]
    pub fn request_personal_voice_authorization_ch(
        handler: impl FnMut(AuthorizationStatus) + Send + 'static,
    ) {
        let mut block = blocks::SendBlock::new1(handler);
        Self::request_personal_voice_authorization_ch_block(&mut block);
    }

    #[cfg(all(feature = "async", feature = "blocks"))]
    pub async fn request_personal_voice_authorization() -> AuthorizationStatus {
        let (future, mut block) = blocks::comp1();
        Self::request_personal_voice_authorization_ch_block(&mut block);
        future.await
    }
}

#[doc(alias = "AVSpeechSynthesizerBufferCallback")]
#[cfg(feature = "blocks")]
pub type BufCallback = blocks::SendBlock<fn(&av::AudioBuf)>;

#[doc(alias = "AVSpeechSynthesizerMarkerCallback")]
#[cfg(feature = "blocks")]
pub type MarkerCallback = blocks::SendBlock<fn(&ns::Array<Marker>)>;

define_obj_type!(
    #[doc(alias = "AVSpeechSynthesisMarker")]
    pub Marker(ns::Id),
    sym AVSpeechSynthesisMarker
);

impl arc::A<Marker> {
    #[objc::msg_send(initWithMarkerType:forTextRange:atByteSampleOffset:)]
    pub fn init_with_mark(
        self,
        mark: MarkerMark,
        text_range: ns::Range,
        byte_sample_offset: usize,
    ) -> arc::R<Marker>;

    #[objc::msg_send(initWithWordRange:atByteSampleOffset:)]
    pub fn init_with_word_range(
        self,
        range: ns::Range,
        byte_sample_offset: isize,
    ) -> arc::R<Marker>;

    #[objc::msg_send(initWithSentenceRange:atByteSampleOffset:)]
    pub fn init_with_sentence_range(
        self,
        range: ns::Range,
        byte_sample_offset: isize,
    ) -> arc::R<Marker>;

    #[objc::msg_send(initWithParagraphRange:atByteSampleOffset:)]
    pub fn init_with_paragraph_range(
        self,
        range: ns::Range,
        byte_sample_offset: isize,
    ) -> arc::R<Marker>;

    #[objc::msg_send(initWithPhonemeString:atByteSampleOffset:)]
    pub fn init_with_phoneme(
        self,
        phoneme: &ns::String,
        byte_sample_offset: isize,
    ) -> arc::R<Marker>;

    #[objc::msg_send(initWithBookmarkName:atByteSampleOffset:)]
    pub fn init_with_bookmark_name(
        self,
        name: &ns::String,
        byte_sample_offset: isize,
    ) -> arc::R<Marker>;
}

impl Marker {
    #[inline]
    pub fn with_mark(
        mark: MarkerMark,
        text_range: ns::Range,
        byte_sample_offset: usize,
    ) -> arc::R<Self> {
        Self::alloc().init_with_mark(mark, text_range, byte_sample_offset)
    }

    #[objc::msg_send(mark)]
    pub fn mark(&self) -> MarkerMark;

    #[objc::msg_send(setMark:)]
    pub fn set_mark(&mut self, val: MarkerMark);

    /// The byte offset into the audio buffer.
    #[objc::msg_send(byteSampleOffset)]
    pub fn byte_sample_offset(&self) -> usize;

    #[objc::msg_send(setByteSampleOffset:)]
    pub fn set_byte_sample_offset(&mut self, val: usize);

    /// The location and length of the request's text.
    #[objc::msg_send(textRange)]
    pub fn text_range(&self) -> ns::Range;

    #[objc::msg_send(setTextRange:)]
    pub fn set_text_range(&mut self, val: ns::Range);

    #[objc::msg_send(bookmarkName)]
    pub fn bookmark_name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setBookmarkName:)]
    pub fn set_bookmark_name(&mut self, val: &ns::String);

    #[objc::msg_send(phoneme)]
    pub fn phoneme(&self) -> arc::R<ns::String>;

    #[objc::msg_send(setPhoneme:)]
    pub fn set_phoneme(&mut self, val: &ns::String);
}

unsafe extern "C" {
    static AVSpeechUtteranceMinimumSpeechRate: f32;
    static AVSpeechUtteranceMaximumSpeechRate: f32;
    static AVSpeechUtteranceDefaultSpeechRate: f32;
    static AVSpeechSynthesisVoiceIdentifierAlex: &'static ns::String;
    static AVSpeechSynthesisIPANotationAttribute: &'static ns::AttrStringKey;
    static AVSpeechSynthesisAvailableVoicesDidChangeNotification: &'static ns::NotificationName;
}

#[cfg(test)]
mod tests {
    use crate::{av, ns};

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
        let settings = voice.audio_file_settings();
        assert!(settings.is_none());

        assert!(av::SpeechSynthesisVoice::voices().len() > 0);

        let v1 = &av::SpeechSynthesisVoice::voices().get(0).unwrap();

        let _v = av::SpeechSynthesisVoice::with_id(&v1.id()).unwrap();
        let _v = av::SpeechSynthesisVoice::with_lang(Some(&v1.lang().unwrap())).unwrap();
    }

    #[tokio::test]
    async fn authorization() {
        let res = av::SpeechSynthesizer::request_personal_voice_authorization().await;
        assert_eq!(res, av::AuthorizationStatus::Restricted);
        assert_eq!(
            res,
            av::SpeechSynthesizer::personal_voice_authorization_status()
        );
    }

    #[test]
    fn write_utterance() {
        let voice = av::SpeechSynthesisVoice::with_lang(Some(ns::str!(c"en-US"))).unwrap();
        assert!(
            !voice
                .traits()
                .contains(av::SpeechSynthesisVoiceTraits::IS_PERSONAL_VOICE)
        );
        let mut ut = av::SpeechUtterance::with_string(ns::str!(c"Hello, world"));
        ut.set_voice(Some(&voice));
        ut.set_post_utterance_delay(0.0);

        // The buffers come on the main queue, which a test's thread never runs.
        let mut synth = av::SpeechSynthesizer::new();
        synth
            .write_utterance_with_markers(&ut, |_buf| {}, |_markers| {})
            .unwrap();
    }

    #[test]
    fn marker() {
        let range = ns::Range::new(0, 5);
        let mut m =
            av::SpeechSynthesisMarker::with_mark(av::SpeechSynthesisMarkerMark::Word, range, 128);
        assert_eq!(m.mark(), av::SpeechSynthesisMarkerMark::Word);
        assert_eq!(m.text_range(), range);
        assert_eq!(m.byte_sample_offset(), 128);
        m.set_mark(av::SpeechSynthesisMarkerMark::Sentence);
        m.set_byte_sample_offset(256);
        assert_eq!(m.mark(), av::SpeechSynthesisMarkerMark::Sentence);
        assert_eq!(m.byte_sample_offset(), 256);

        let name = ns::str!(c"intro");
        let m = av::SpeechSynthesisMarker::alloc().init_with_bookmark_name(name, 64);
        assert_eq!(&m.bookmark_name(), name);
    }

    #[test]
    fn utterance() {
        let str = ns::str!(c"Hello");

        let ut = av::SpeechUtterance::with_string(str);
        assert_eq!(&ut.speech_string(), str);
        assert!(ut.speech_attr_string().is_none());

        let attr_str = ns::AttrString::with_string(&str);
        let ut = av::SpeechUtterance::with_attr_string(&attr_str);
        assert_eq!(&ut.speech_string(), str);
        assert_eq!(&ut.speech_attr_string().unwrap().string(), str);
    }
}
