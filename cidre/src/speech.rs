//! Speech.framework native Swift ABI bindings.
//!
//! These bindings target the Swift-native API introduced in Apple OS 26 and
//! call framework and Swift runtime symbols directly. No C or Objective-C
//! wrapper functions are used.

#[cfg(feature = "av")]
mod capture_input_sequence_provider;
mod dictation_transcriber;
mod locale;
mod speech_analyzer;
mod speech_detector;
mod speech_module;
mod speech_transcriber;
mod value;

#[cfg(feature = "av")]
pub use capture_input_sequence_provider::CaptureInputSequenceProvider;
pub use dictation_transcriber::{DictationPreset, DictationTranscriber};
pub use speech_analyzer::SpeechAnalyzer;
pub use speech_detector::{SensitivityLevel, SpeechDetector};
pub use speech_module::SpeechModule;
pub use speech_transcriber::{SpeechTranscriber, TranscriberPreset};

#[link(name = "Speech", kind = "framework")]
#[link(name = "swiftFoundation")]
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
