use crate::{
    api, arc, define_swift_getter_enum, ns, swift::concurrency::swift_opaque_iterator_typeref,
};

use super::{speech_transcriber::ResultsTask, transcriber_with_id_and_preset};

crate::define_swift_class!(pub DictationTranscriber = accessor dictation_transcriber_metadata);

define_swift_getter_enum!(
    /// `DictationTranscriber.Preset`.
    ///
    /// The `private` cases are presets Speech ships but does not declare in its
    /// public interface.
    pub DictationPreset in "Speech"
        = accessor "$s6Speech20DictationTranscriberC6PresetVMa"
    {
        LongDictation = "$s6Speech20DictationTranscriberC6PresetV04longB0AEvgZ",
        Phrase = "$s6Speech20DictationTranscriberC6PresetV6phraseAEvgZ",
        ProgressiveLongDictation = "$s6Speech20DictationTranscriberC6PresetV015progressiveLongB0AEvgZ",
        ProgressiveShortDictation = "$s6Speech20DictationTranscriberC6PresetV016progressiveShortB0AEvgZ",
        ShortDictation = "$s6Speech20DictationTranscriberC6PresetV05shortB0AEvgZ",
        TimeIndexedLongDictation = "$s6Speech20DictationTranscriberC6PresetV015timeIndexedLongB0AEvgZ",
        #[cfg(feature = "private")]
        Assistant = "$s6Speech20DictationTranscriberC6PresetV9assistantAEvgZ",
        #[cfg(feature = "private")]
        AssistantDictation = "$s6Speech20DictationTranscriberC6PresetV09assistantB0AEvgZ",
        #[cfg(feature = "private")]
        Captioning = "$s6Speech20DictationTranscriberC6PresetV10captioningAEvgZ",
        #[cfg(feature = "private")]
        DictationCC = "$s6Speech20DictationTranscriberC6PresetV11dictationCCAEvgZ",
        #[cfg(feature = "private")]
        FoundInCalls = "$s6Speech20DictationTranscriberC6PresetV12foundInCallsAEvgZ",
        #[cfg(feature = "private")]
        KeyboardDictation = "$s6Speech20DictationTranscriberC6PresetV08keyboardB0AEvgZ",
        #[cfg(feature = "private")]
        MultisegmentAssistant = "$s6Speech20DictationTranscriberC6PresetV21multisegmentAssistantAEvgZ",
        #[cfg(feature = "private")]
        MultisegmentAssistantDictation = "$s6Speech20DictationTranscriberC6PresetV021multisegmentAssistantB0AEvgZ",
        #[cfg(feature = "private")]
        MultisegmentCaptioning = "$s6Speech20DictationTranscriberC6PresetV22multisegmentCaptioningAEvgZ",
        #[cfg(feature = "private")]
        MultisegmentDictationCC = "$s6Speech20DictationTranscriberC6PresetV012multisegmentB2CCAEvgZ",
        #[cfg(feature = "private")]
        MultisegmentFoundInCalls = "$s6Speech20DictationTranscriberC6PresetV24multisegmentFoundInCallsAEvgZ",
        #[cfg(feature = "private")]
        MultisegmentKeyboardDictation = "$s6Speech20DictationTranscriberC6PresetV020multisegmentKeyboardB0AEvgZ",
        #[cfg(feature = "private")]
        MultisegmentSearch = "$s6Speech20DictationTranscriberC6PresetV18multisegmentSearchAEvgZ",
        #[cfg(feature = "private")]
        MultisegmentSpellCC = "$s6Speech20DictationTranscriberC6PresetV19multisegmentSpellCCAEvgZ",
        #[cfg(feature = "private")]
        MultisegmentSpelling = "$s6Speech20DictationTranscriberC6PresetV20multisegmentSpellingAEvgZ",
        #[cfg(feature = "private")]
        MultisegmentTshot = "$s6Speech20DictationTranscriberC6PresetV17multisegmentTshotAEvgZ",
        #[cfg(feature = "private")]
        MultisegmentVoicemail = "$s6Speech20DictationTranscriberC6PresetV21multisegmentVoicemailAEvgZ",
        #[cfg(feature = "private")]
        Search = "$s6Speech20DictationTranscriberC6PresetV6searchAEvgZ",
        #[cfg(feature = "private")]
        SpellCC = "$s6Speech20DictationTranscriberC6PresetV7spellCCAEvgZ",
        #[cfg(feature = "private")]
        Spelling = "$s6Speech20DictationTranscriberC6PresetV8spellingAEvgZ",
        #[cfg(feature = "private")]
        Tshot = "$s6Speech20DictationTranscriberC6PresetV5tshotAEvgZ",
        #[cfg(feature = "private")]
        Voicemail = "$s6Speech20DictationTranscriberC6PresetV9voicemailAEvgZ",
    }
);

impl Default for DictationPreset {
    #[inline]
    fn default() -> Self {
        Self::ProgressiveLongDictation
    }
}

#[link(name = "Speech", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s6Speech20DictationTranscriberCMa"]
    fn dictation_transcriber_metadata();

    #[link_name = "$s6Speech20DictationTranscriberC6locale6presetAC10Foundation6LocaleV_AC6PresetVtcfC"]
    fn dictation_transcriber_init();

    #[link_name = "$s6Speech20DictationTranscriberC7resultsQrvg"]
    fn dictation_transcriber_results();

    #[link_name = "$s6Speech20DictationTranscriberC7resultsQrvpQOMQ"]
    static DICTATION_TRANSCRIBER_RESULTS_DESCRIPTOR: u8;

    #[link_name = "$s6Speech20DictationTranscriberC6ResultVMa"]
    fn dictation_transcriber_result_metadata();

    #[link_name = "$s6Speech20DictationTranscriberC6ResultV4text10Foundation16AttributedStringVvg"]
    fn dictation_transcriber_result_text();
}

impl DictationTranscriber {
    /// Creates a transcriber using `Foundation.Locale(identifier:)` and one of
    /// Speech's standard dictation presets.
    #[doc(alias = "DictationTranscriber.init(locale:preset:)")]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    pub fn with_locale_id(locale_id: &str, preset: DictationPreset) -> arc::R<Self> {
        unsafe {
            arc::R::from_raw(
                transcriber_with_id_and_preset(
                    locale_id,
                    preset,
                    dictation_transcriber_metadata as _,
                    dictation_transcriber_init as _,
                )
                .cast(),
            )
        }
    }

    /// Iterates `DictationTranscriber.results` on a Swift concurrency task.
    #[doc(alias = "DictationTranscriber.results")]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    pub fn results<F>(&self, callback: F)
    where
        F: FnMut(Result<Option<std::string::String>, arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            ResultsTask::start(
                (self as *const Self).cast(),
                dictation_transcriber_results as *const (),
                (&raw const DICTATION_TRANSCRIBER_RESULTS_DESCRIPTOR).cast(),
                &raw const cidre_dictation_transcriber_results_iterator_type_start,
                &raw const cidre_dictation_transcriber_results_iterator_type_end,
                dictation_transcriber_result_metadata as *const (),
                "6Speech20DictationTranscriberC6ResultVSg",
                dictation_transcriber_result_text as *const (),
                callback,
            );
        }
    }
}

swift_opaque_iterator_typeref!(
    cidre_dictation_transcriber_results_iterator_type_start
        ..= cidre_dictation_transcriber_results_iterator_type_end,
    descriptor: DICTATION_TRANSCRIBER_RESULTS_DESCRIPTOR,
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::swift::{SwiftMetadata, ToSwift, abi, value::Storage};

    /// Each case must reach its own Swift static, so a mistyped mangled name
    /// cannot silently alias another preset.
    #[test]
    fn every_preset_reads_a_distinct_swift_value() {
        let metadata = DictationPreset::metadata();
        let size = unsafe { abi::value_layout(metadata) }.size;

        let all = DictationPreset::all();
        let values: Vec<Vec<u8>> = all
            .iter()
            .map(|preset| unsafe {
                let mut storage = Storage::<DictationPreset>::new();
                preset.copy_to_swift(storage.as_mut_ptr());
                let value = storage.assume_init();
                core::slice::from_raw_parts(value.as_ptr().cast::<u8>(), size).to_vec()
            })
            .collect();

        for (index, value) in values.iter().enumerate() {
            for (other_index, other) in values.iter().enumerate().skip(index + 1) {
                assert_ne!(
                    value, other,
                    "{:?} and {:?} read the same value",
                    all[index], all[other_index]
                );
            }
        }
    }

    #[test]
    #[allow(unused_unsafe)]
    fn every_preset_constructs_a_transcriber() {
        for preset in DictationPreset::all() {
            unsafe {
                let transcriber = DictationTranscriber::with_locale_id("en_US", preset);
                let _module = crate::swift::speech::SpeechModule::from(transcriber.as_ref());
            }
        }
    }
}
