use crate::{
    api, arc, ns, swift,
    swift::{SwiftMetadata, abi, async_task::swift_opaque_iterator_typeref},
};

use super::{
    speech_transcriber::ResultsTask,
    value::{Storage, call_with_owned_values},
};

crate::define_swift_class!(pub DictationTranscriber);

#[link(name = "Speech", kind = "framework")]
#[link(name = "swiftFoundation")]
unsafe extern "C" {
    #[link_name = "$s10Foundation6LocaleVMa"]
    fn foundation_locale_metadata();

    #[link_name = "$s10Foundation6LocaleV10identifierACSS_tcfC"]
    fn foundation_locale_init();

    #[link_name = "$s6Speech20DictationTranscriberCMa"]
    fn dictation_transcriber_metadata();

    #[link_name = "$s6Speech20DictationTranscriberC6PresetVMa"]
    fn dictation_transcriber_preset_metadata();

    #[link_name = "$s6Speech20DictationTranscriberC6PresetV015progressiveLongB0AEvgZ"]
    fn progressive_long_dictation_preset();

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

struct FoundationLocale;

unsafe impl SwiftMetadata for FoundationLocale {
    fn metadata() -> *const abi::TypeMetadata {
        unsafe {
            abi::call_int_to_int(foundation_locale_metadata as *const (), 0)
                as *const abi::TypeMetadata
        }
    }
}

struct DictationTranscriberPreset;

unsafe impl SwiftMetadata for DictationTranscriberPreset {
    fn metadata() -> *const abi::TypeMetadata {
        unsafe {
            abi::call_int_to_int(dictation_transcriber_preset_metadata as *const (), 0)
                as *const abi::TypeMetadata
        }
    }
}

impl DictationTranscriber {
    /// Creates a transcriber with
    /// `DictationTranscriber.Preset.progressiveLongDictation`.
    #[doc(alias = "DictationTranscriber.init(locale:preset:)")]
    #[doc(alias = "DictationTranscriber.Preset.progressiveLongDictation")]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    pub fn with_progressive_long_dictation_locale_id(locale_id: &str) -> arc::R<Self> {
        unsafe {
            let mut locale_storage = Storage::<FoundationLocale>::new();
            let locale_id = swift::String::from(locale_id).into_raw();
            abi::call_string_to_value(
                foundation_locale_init as *const (),
                locale_id,
                locale_storage.as_mut_ptr(),
            );
            let locale = locale_storage.assume_init();

            let mut preset_storage = Storage::<DictationTranscriberPreset>::new();
            abi::call0_value(
                progressive_long_dictation_preset as *const (),
                preset_storage.as_mut_ptr(),
            );
            let preset = preset_storage.assume_init();

            let transcriber_metadata =
                abi::call_int_to_int(dictation_transcriber_metadata as *const (), 0) as *const ();
            let object = call_with_owned_values(locale, preset, |locale, preset| {
                abi::call_static_values_to_object(
                    dictation_transcriber_init as *const (),
                    transcriber_metadata,
                    locale,
                    preset,
                )
            });
            arc::R::from_raw(object.cast())
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
