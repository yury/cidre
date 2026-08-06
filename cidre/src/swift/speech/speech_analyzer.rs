use crate::{api, arc, swift, swift::abi};

#[cfg(feature = "av")]
use crate::ns;

#[cfg(feature = "av")]
use crate::swift::concurrency::{self, AsyncCallArgs};

#[cfg(feature = "av")]
use super::CaptureInputSequenceProvider;

use super::SpeechModule;
use crate::swift::value::{Optional, Value, call_with_owned_value};

crate::define_swift_class!(pub SpeechAnalyzer = accessor speech_analyzer_metadata);

#[link(name = "Speech", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s6Speech0A8AnalyzerCMa"]
    fn speech_analyzer_metadata();

    #[link_name = "$s6Speech0A8AnalyzerC7modules7optionsACSayAA0A6Module_pG_AC7OptionsVSgtcfC"]
    fn speech_analyzer_init();

    #[link_name = "$s6Speech0A8AnalyzerC7OptionsVMa"]
    fn speech_analyzer_options_metadata();

    #[cfg(feature = "av")]
    #[link_name = "$s6Speech0A8AnalyzerC15analyzeSequenceySo6CMTimeaSgxYaKs8SendableRzSciRzAA0B5InputV7ElementRtzlF"]
    fn speech_analyzer_analyze_sequence();

    #[cfg(feature = "av")]
    #[link_name = "$s6Speech0A8AnalyzerC15analyzeSequenceySo6CMTimeaSgxYaKs8SendableRzSciRzAA0B5InputV7ElementRtzlFTu"]
    static ANALYZE_SEQUENCE_ASYNC_FN: u8;
}

crate::define_swift_marker!(AnalyzerOptions = accessor speech_analyzer_options_metadata);

impl SpeechAnalyzer {
    /// Creates an analyzer with `options: nil`.
    #[doc(alias = "SpeechAnalyzer.init(modules:options:)")]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    pub fn with_modules(modules: &[SpeechModule]) -> arc::R<Self> {
        unsafe {
            let modules = swift::Array::from_slice(modules);
            let options = Value::<Optional<AnalyzerOptions>>::none();
            let analyzer_metadata =
                <SpeechAnalyzer as crate::swift::SwiftMetadata>::metadata().cast();
            let object = call_with_owned_value(options, |options| {
                abi::call_static_array_value_to_object(
                    speech_analyzer_init as *const (),
                    analyzer_metadata,
                    modules.into_raw(),
                    options,
                )
            });
            arc::R::from_raw(object.cast())
        }
    }

    /// Starts consuming the provider's live analyzer-input sequence.
    #[cfg(feature = "av")]
    #[doc(alias = "SpeechAnalyzer.analyzeSequence")]
    #[api::available(
        macos = 27.0,
        ios = 27.0,
        maccatalyst = 27.0,
        tvos = 27.0,
        visionos = 27.0
    )]
    pub fn analyze_capture<F>(&self, provider: &CaptureInputSequenceProvider, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            concurrency::call_async_result(
                speech_analyzer_analyze_sequence as *const (),
                &raw const ANALYZE_SEQUENCE_ASYNC_FN,
                // Declaration order is drop order: the input sequence comes out
                // of the provider, so it is destroyed before the provider is
                // released.
                (
                    provider.analyzer_inputs(),
                    arc::Retain::retained(provider),
                    arc::Retain::retained(self),
                ),
                |(input, _provider, analyzer)| {
                    // The generic call carries the input sequence's type and
                    // its `AsyncSequence` conformance alongside the value.
                    AsyncCallArgs::new()
                        .swift_self(analyzer.as_ptr().cast())
                        .arg(0, input.value.as_mut_ptr())
                        .arg(1, input.value.metadata().cast_mut().cast())
                        .arg(2, input.witness.cast_mut())
                },
                |_, _| (),
                callback,
            );
        }
    }
}
