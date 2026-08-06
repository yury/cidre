//! Speech.framework native Swift ABI bindings.
//!
//! These bindings target the Swift-native API introduced in Apple OS 26 and
//! call framework and Swift runtime symbols directly. No C or Objective-C
//! wrapper functions are used.

#[cfg(feature = "av")]
mod capture_input_sequence_provider;
mod dictation_transcriber;
mod speech_analyzer;
mod speech_detector;
mod speech_module;
mod speech_transcriber;

#[cfg(feature = "av")]
pub use capture_input_sequence_provider::CaptureInputSequenceProvider;
pub use dictation_transcriber::{DictationPreset, DictationTranscriber};
pub use speech_analyzer::SpeechAnalyzer;
pub use speech_detector::{SensitivityLevel, SpeechDetector};
pub use speech_module::SpeechModule;
pub use speech_transcriber::{SpeechTranscriber, TranscriberPreset};

use crate::swift::{
    ToSwift, abi, foundation,
    value::{Storage, call_with_owned_values},
};

#[link(name = "Speech", kind = "framework")]
unsafe extern "C" {}

/// Creates a transcriber through `init(locale:preset:)`.
///
/// `SpeechTranscriber` and `DictationTranscriber` both conform to
/// `LocaleDependentSpeechModule` and are both created by an `init(locale:preset:)`
/// taking two indirect `@owned` values. Only the symbols differ, so the
/// construction sequence lives here once. Both the locale and the preset are
/// consumed by the initializer.
///
/// # Safety
///
/// The symbols must belong to one type: `class_metadata_accessor` and `init`
/// must name a transcriber whose `Preset` is `P`.
unsafe fn transcriber_with_id_and_preset<P: ToSwift>(
    locale_id: &str,
    preset: P,
    class_metadata_accessor: *const (),
    init: *const (),
) -> *mut () {
    unsafe {
        let locale = foundation::Locale::with_id(locale_id);

        let mut preset_storage = Storage::<P>::new();
        preset.copy_to_swift(preset_storage.as_mut_ptr());
        let preset = preset_storage.assume_init();

        let class_metadata = abi::call::int_to_int(class_metadata_accessor, 0) as *const ();
        call_with_owned_values(locale.into_value(), preset, |locale, preset| {
            abi::call::static_values_to_object(init, class_metadata, locale, preset)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(unused_unsafe)]
    fn detector_and_analyzer_use_native_swift_abi() {
        unsafe {
            let detector = SpeechDetector::with_sensitivity(SensitivityLevel::High, true);
            let retained = detector.clone();
            drop(detector);

            let module = SpeechModule::from(retained.as_ref());
            let cloned_module = module.clone();
            drop(module);
            let analyzer = SpeechAnalyzer::with_modules(&[cloned_module]);
            drop(analyzer);
        }
    }

    #[test]
    #[allow(unused_unsafe)]
    fn transcriber_uses_foundation_locale_and_preset_values() {
        unsafe {
            let _ = SpeechTranscriber::is_available();

            let transcriber =
                SpeechTranscriber::with_locale_id("en-US", TranscriberPreset::Transcription);
            let module = SpeechModule::from(transcriber.as_ref());
            drop(transcriber);

            let _analyzer = SpeechAnalyzer::with_modules(&[module]);
        }
    }

    #[test]
    #[allow(unused_unsafe)]
    fn dictation_transcriber_uses_progressive_long_dictation() {
        unsafe {
            let transcriber =
                DictationTranscriber::with_locale_id("en_US", DictationPreset::default());
            let module = SpeechModule::from(transcriber.as_ref());
            drop(transcriber);

            let _analyzer = SpeechAnalyzer::with_modules(&[module]);
        }
    }
}
