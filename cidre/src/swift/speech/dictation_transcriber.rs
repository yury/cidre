use crate::{api, arc, ns, swift::concurrency::swift_opaque_iterator_typeref};

use super::{locale, speech_transcriber::ResultsTask};

crate::define_swift_class!(pub DictationTranscriber = accessor dictation_transcriber_metadata);

/// Declares `DictationTranscriber.Preset` cases together with the Swift static
/// getter each one reads, so a case can never drift from its symbol.
///
/// Symbol names are suffixes of the `Preset` type's mangling.
macro_rules! dictation_presets {
    ($( $(#[$meta:meta])* $variant:ident => $getter:ident @ $suffix:literal ),* $(,)?) => {
        /// `DictationTranscriber.Preset`.
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
        #[non_exhaustive]
        pub enum DictationPreset {
            $( $(#[$meta])* $variant, )*
        }

        unsafe extern "C" {
            $(
                $(#[$meta])*
                #[link_name = concat!("$s6Speech20DictationTranscriberC6PresetV", $suffix)]
                fn $getter();
            )*
        }

        impl DictationPreset {
            fn getter(self) -> *const () {
                match self {
                    $( $(#[$meta])* Self::$variant => $getter as _, )*
                }
            }

            #[cfg(test)]
            fn all() -> Vec<Self> {
                let mut presets = Vec::new();
                $(
                    $(#[$meta])*
                    presets.push(Self::$variant);
                )*
                presets
            }
        }
    };
}

dictation_presets!(
    LongDictation => long_dictation_preset @ "04longB0AEvgZ",
    Phrase => phrase_preset @ "6phraseAEvgZ",
    ProgressiveLongDictation => progressive_long_dictation_preset @ "015progressiveLongB0AEvgZ",
    ProgressiveShortDictation => progressive_short_dictation_preset @ "016progressiveShortB0AEvgZ",
    ShortDictation => short_dictation_preset @ "05shortB0AEvgZ",
    TimeIndexedLongDictation => time_indexed_long_dictation_preset @ "015timeIndexedLongB0AEvgZ",
    // Presets Speech ships but does not declare in its public interface.
    #[cfg(feature = "private")]
    Assistant => assistant_preset @ "9assistantAEvgZ",
    #[cfg(feature = "private")]
    AssistantDictation => assistant_dictation_preset @ "09assistantB0AEvgZ",
    #[cfg(feature = "private")]
    Captioning => captioning_preset @ "10captioningAEvgZ",
    #[cfg(feature = "private")]
    DictationCC => dictation_cc_preset @ "11dictationCCAEvgZ",
    #[cfg(feature = "private")]
    FoundInCalls => found_in_calls_preset @ "12foundInCallsAEvgZ",
    #[cfg(feature = "private")]
    KeyboardDictation => keyboard_dictation_preset @ "08keyboardB0AEvgZ",
    #[cfg(feature = "private")]
    MultisegmentAssistant => multisegment_assistant_preset @ "21multisegmentAssistantAEvgZ",
    #[cfg(feature = "private")]
    MultisegmentAssistantDictation => multisegment_assistant_dictation_preset @ "021multisegmentAssistantB0AEvgZ",
    #[cfg(feature = "private")]
    MultisegmentCaptioning => multisegment_captioning_preset @ "22multisegmentCaptioningAEvgZ",
    #[cfg(feature = "private")]
    MultisegmentDictationCC => multisegment_dictation_cc_preset @ "012multisegmentB2CCAEvgZ",
    #[cfg(feature = "private")]
    MultisegmentFoundInCalls => multisegment_found_in_calls_preset @ "24multisegmentFoundInCallsAEvgZ",
    #[cfg(feature = "private")]
    MultisegmentKeyboardDictation => multisegment_keyboard_dictation_preset @ "020multisegmentKeyboardB0AEvgZ",
    #[cfg(feature = "private")]
    MultisegmentSearch => multisegment_search_preset @ "18multisegmentSearchAEvgZ",
    #[cfg(feature = "private")]
    MultisegmentSpellCC => multisegment_spell_cc_preset @ "19multisegmentSpellCCAEvgZ",
    #[cfg(feature = "private")]
    MultisegmentSpelling => multisegment_spelling_preset @ "20multisegmentSpellingAEvgZ",
    #[cfg(feature = "private")]
    MultisegmentTshot => multisegment_tshot_preset @ "17multisegmentTshotAEvgZ",
    #[cfg(feature = "private")]
    MultisegmentVoicemail => multisegment_voicemail_preset @ "21multisegmentVoicemailAEvgZ",
    #[cfg(feature = "private")]
    Search => search_preset @ "6searchAEvgZ",
    #[cfg(feature = "private")]
    SpellCC => spell_cc_preset @ "7spellCCAEvgZ",
    #[cfg(feature = "private")]
    Spelling => spelling_preset @ "8spellingAEvgZ",
    #[cfg(feature = "private")]
    Tshot => tshot_preset @ "5tshotAEvgZ",
    #[cfg(feature = "private")]
    Voicemail => voicemail_preset @ "9voicemailAEvgZ",
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

    #[link_name = "$s6Speech20DictationTranscriberC6PresetVMa"]
    fn dictation_transcriber_preset_metadata();

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

crate::define_swift_marker!(DictationTranscriberPreset = accessor dictation_transcriber_preset_metadata);

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
                locale::transcriber_with_id_and_preset::<DictationTranscriberPreset>(
                    locale_id,
                    preset.getter(),
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
    use crate::swift::{SwiftMetadata, abi, value::Storage};

    /// Each case must reach its own Swift static, so a mistyped mangled name
    /// cannot silently alias another preset.
    #[test]
    fn every_preset_reads_a_distinct_swift_value() {
        let metadata = DictationTranscriberPreset::metadata();
        let size = unsafe { abi::value_layout(metadata) }.size;

        let all = DictationPreset::all();
        let values: Vec<Vec<u8>> = all
            .iter()
            .map(|preset| unsafe {
                let mut storage = Storage::<DictationTranscriberPreset>::new();
                abi::call0_value(preset.getter(), storage.as_mut_ptr());
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
