use crate::{
    api, arc, define_swift_getter_enum, ns, swift,
    swift::{concurrency::swift_opaque_iterator_typeref, foundation, value::Storage},
};

use super::speech_transcriber::ResultsTask;

crate::define_swift!(#[swift::class("Speech.DictationTranscriber")] pub DictationTranscriber);

define_swift_getter_enum!(
    /// `DictationTranscriber.Preset`.
    ///
    /// The `private` cases are presets Speech ships but does not declare in its
    /// public interface.
    pub DictationPreset = swift "Speech.DictationTranscriber(class).Preset" {
        LongDictation = "longDictation",
        Phrase = "phrase",
        ProgressiveLongDictation = "progressiveLongDictation",
        ProgressiveShortDictation = "progressiveShortDictation",
        ShortDictation = "shortDictation",
        TimeIndexedLongDictation = "timeIndexedLongDictation",
        #[cfg(feature = "private")]
        Assistant = "assistant",
        #[cfg(feature = "private")]
        AssistantDictation = "assistantDictation",
        #[cfg(feature = "private")]
        Captioning = "captioning",
        #[cfg(feature = "private")]
        DictationCC = "dictationCC",
        #[cfg(feature = "private")]
        FoundInCalls = "foundInCalls",
        #[cfg(feature = "private")]
        KeyboardDictation = "keyboardDictation",
        #[cfg(feature = "private")]
        MultisegmentAssistant = "multisegmentAssistant",
        #[cfg(feature = "private")]
        MultisegmentAssistantDictation = "multisegmentAssistantDictation",
        #[cfg(feature = "private")]
        MultisegmentCaptioning = "multisegmentCaptioning",
        #[cfg(feature = "private")]
        MultisegmentDictationCC = "multisegmentDictationCC",
        #[cfg(feature = "private")]
        MultisegmentFoundInCalls = "multisegmentFoundInCalls",
        #[cfg(feature = "private")]
        MultisegmentKeyboardDictation = "multisegmentKeyboardDictation",
        #[cfg(feature = "private")]
        MultisegmentSearch = "multisegmentSearch",
        #[cfg(feature = "private")]
        MultisegmentSpellCC = "multisegmentSpellCC",
        #[cfg(feature = "private")]
        MultisegmentSpelling = "multisegmentSpelling",
        #[cfg(feature = "private")]
        MultisegmentTshot = "multisegmentTshot",
        #[cfg(feature = "private")]
        MultisegmentVoicemail = "multisegmentVoicemail",
        #[cfg(feature = "private")]
        Search = "search",
        #[cfg(feature = "private")]
        SpellCC = "spellCC",
        #[cfg(feature = "private")]
        Spelling = "spelling",
        #[cfg(feature = "private")]
        Tshot = "tshot",
        #[cfg(feature = "private")]
        Voicemail = "voicemail",
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
    #[link_name = "$s6Speech20DictationTranscriberC7resultsQrvg"]
    fn dictation_transcriber_results();

    #[link_name = "$s6Speech20DictationTranscriberC7resultsQrvpQOMQ"]
    static DICTATION_TRANSCRIBER_RESULTS_DESCRIPTOR: u8;
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
        Self::with_locale(foundation::Locale::with_id(locale_id), preset)
    }

    /// Creates a transcriber for `locale` with one of Speech's standard
    /// presets.
    #[doc(alias = "DictationTranscriber.init(locale:preset:)")]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    pub fn with_locale(locale: foundation::Locale, preset: DictationPreset) -> arc::R<Self> {
        Self::init_with_locale(locale, Storage::from_value(&preset))
    }

    /// The initializer takes both values at `+1`.
    #[swift::call(
        "Speech.DictationTranscriber(class).init(locale: Foundation.Locale(struct), \
         preset: Speech.DictationTranscriber(class).Preset(struct))"
    )]
    fn init_with_locale(
        locale: foundation::Locale,
        preset: Storage<DictationPreset>,
    ) -> arc::R<Self>;

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
                swift::metadata_accessor!(struct, "Speech.DictationTranscriber(class).Result"),
                "6Speech20DictationTranscriberC6ResultVSg",
                swift::symbol!(
                    "Speech.DictationTranscriber(class).Result(struct).text: \
                     Foundation.AttributedString(struct) { get }"
                ),
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
                let bytes =
                    core::slice::from_raw_parts(storage.as_ptr().cast::<u8>(), size).to_vec();
                storage.destroy();
                bytes
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
