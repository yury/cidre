//! Speech.framework native Swift ABI bindings.
//!
//! These bindings target the Swift-native API introduced in Apple OS 26 and
//! call framework and Swift runtime symbols directly. No C or Objective-C
//! wrapper functions are used.

// The module as a whole is compiled for OS 26; what is newer, or missing on a
// platform, is gated again here.

/// New in OS 27.
#[cfg(all(
    feature = "av",
    any(
        all(target_os = "macos", feature = "macos_27_0"),
        all(target_os = "ios", not(target_abi = "macabi"), feature = "ios_27_0"),
        all(target_os = "ios", target_abi = "macabi", feature = "maccatalyst_27_0"),
        all(target_os = "tvos", feature = "tvos_27_0"),
        all(target_os = "visionos", feature = "visionos_27_0"),
    )
))]
mod capture_input_sequence_provider;
/// Not on tvOS.
#[cfg(not(target_os = "tvos"))]
mod dictation_transcriber;
mod speech_analyzer;
mod speech_detector;
mod speech_module;
mod speech_transcriber;

#[cfg(all(
    feature = "av",
    any(
        all(target_os = "macos", feature = "macos_27_0"),
        all(target_os = "ios", not(target_abi = "macabi"), feature = "ios_27_0"),
        all(target_os = "ios", target_abi = "macabi", feature = "maccatalyst_27_0"),
        all(target_os = "tvos", feature = "tvos_27_0"),
        all(target_os = "visionos", feature = "visionos_27_0"),
    )
))]
pub use capture_input_sequence_provider::CaptureInputSequenceProvider;
#[cfg(not(target_os = "tvos"))]
pub use dictation_transcriber::{DictationPreset, DictationTranscriber};
pub use speech_analyzer::SpeechAnalyzer;
pub use speech_detector::{SensitivityLevel, SpeechDetector};
pub use speech_module::SpeechModule;
pub use speech_transcriber::{SpeechTranscriber, TranscriberPreset};

#[link(name = "Speech", kind = "framework")]
unsafe extern "C" {}

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

    /// `init(locale:preset:)` consumes both values. Handing over clones of one
    /// locale many times would release it once too often per call if the
    /// generated call also dropped what Swift took, and leave the original
    /// dangling; it has to stay readable throughout.
    #[cfg(not(target_os = "tvos"))]
    #[test]
    fn a_consumed_locale_is_released_exactly_once() {
        let locale = crate::swift::foundation::Locale::with_id("en_US");
        for _ in 0..500 {
            let transcriber =
                SpeechTranscriber::with_locale(locale.clone(), TranscriberPreset::Transcription);
            drop(transcriber);
            let dictation =
                DictationTranscriber::with_locale(locale.clone(), DictationPreset::default());
            drop(dictation);
        }
        assert_eq!("en_US", locale.id().to_string());
    }

    #[cfg(not(target_os = "tvos"))]
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
