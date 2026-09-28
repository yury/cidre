//! `CaptureInputSequenceProvider`, new in OS 27, and analyzing what it
//! captures. The module is compiled only for 27, so nothing here reaches a
//! symbol an older system lacks.

use crate::{
    arc, av, ns, swift,
    swift::{
        abi,
        concurrency::{self, AsyncCallArgs, TaskPriority},
        value::{Optional, Storage},
    },
};

use super::{SpeechAnalyzer, SpeechModule};

crate::define_swift!(
    #[swift::class("Speech.CaptureInputSequenceProvider")]
    pub CaptureInputSequenceProvider
);

struct AnalyzerInputSequence {
    value: crate::swift::value::AnyValue,
    witness: *const (),
}

unsafe impl Send for AnalyzerInputSequence {}

#[link(name = "Speech", kind = "framework")]
unsafe extern "C" {
    // `analyzerInputs` is an opaque `some AsyncSequence`, whose getter and
    // descriptor the mangler does not spell.
    #[link_name = "$s6Speech28CaptureInputSequenceProviderC14analyzerInputsQrvg"]
    fn capture_input_sequence_provider_analyzer_inputs();

    #[link_name = "$s6Speech28CaptureInputSequenceProviderC14analyzerInputsQrvpQOMQ"]
    static CAPTURE_INPUT_SEQUENCE_PROVIDER_ANALYZER_INPUTS_DESCRIPTOR: u8;

    // `SpeechAnalyzer.analyzeSequence(_:)` is generic over its input sequence,
    // with requirements the mangler does not spell.
    #[link_name = "$s6Speech0A8AnalyzerC15analyzeSequenceySo6CMTimeaSgxYaKs8SendableRzSciRzAA0B5InputV7ElementRtzlF"]
    fn speech_analyzer_analyze_sequence();

    #[link_name = "$s6Speech0A8AnalyzerC15analyzeSequenceySo6CMTimeaSgxYaKs8SendableRzSciRzAA0B5InputV7ElementRtzlFTu"]
    static ANALYZE_SEQUENCE_ASYNC_FN: u8;
}

crate::define_swift_marker!(
    AnalyzerInputs =
        opaque(&raw const CAPTURE_INPUT_SEQUENCE_PROVIDER_ANALYZER_INPUTS_DESCRIPTOR).cast(),
    0
);

impl CaptureInputSequenceProvider {
    /// `priority` is the call's `TaskPriority?`, which these bindings always
    /// leave to the runtime.
    #[doc(alias = "CaptureInputSequenceProvider.providerWithSession")]
    #[swift::call(
        "static Speech.CaptureInputSequenceProvider(class).providerWithSession(\
         from: __C.AVCaptureDevice(class), \
         compatibleWith: [any Speech.SpeechModule], \
         priority: TaskPriority?) async throws -> Speech.CaptureInputSequenceProvider(class)"
    )]
    fn provider_with_session(
        device: arc::R<av::CaptureDevice>,
        modules: swift::Array<SpeechModule>,
        priority: Storage<Optional<TaskPriority>>,
    ) -> Result<arc::R<Self>, arc::R<ns::Error>>;

    #[doc(alias = "CaptureInputSequenceProvider.providerWithSession")]
    pub fn with_session_handler<F>(
        device: &av::CaptureDevice,
        modules: &[SpeechModule],
        callback: F,
    ) where
        F: FnOnce(Result<arc::R<Self>, arc::R<ns::Error>>) + Send + 'static,
    {
        Self::provider_with_session_handler(
            device.retained(),
            swift::Array::from_slice(modules),
            Storage::none(),
            callback,
        );
    }

    #[doc(alias = "CaptureInputSequenceProvider.providerWithSession")]
    #[cfg(feature = "async")]
    pub fn with_session(
        device: &av::CaptureDevice,
        modules: &[SpeechModule],
    ) -> impl Future<Output = Result<arc::R<Self>, arc::R<ns::Error>>> {
        Self::provider_with_session(
            device.retained(),
            swift::Array::from_slice(modules),
            Storage::none(),
        )
    }

    #[doc(alias = "CaptureInputSequenceProvider.captureSession")]
    #[swift::call(
        "Speech.CaptureInputSequenceProvider(class).captureSession: \
         __C.AVCaptureSession(class) { get }"
    )]
    pub fn capture_session(&self) -> arc::R<av::CaptureSession>;

    fn analyzer_inputs(&self) -> AnalyzerInputSequence {
        unsafe {
            let descriptor =
                (&raw const CAPTURE_INPUT_SEQUENCE_PROVIDER_ANALYZER_INPUTS_DESCRIPTOR).cast();
            let mut storage = crate::swift::value::DynamicStorage::new(
                <AnalyzerInputs as crate::swift::SwiftMetadata>::metadata(),
            );
            abi::call::object_to_value(
                capture_input_sequence_provider_analyzer_inputs as *const (),
                (self as *const Self).cast(),
                storage.as_mut_ptr(),
            );
            let value = storage.assume_init();
            let witness = abi::opaque_type_conformance(descriptor, 1);
            AnalyzerInputSequence { value, witness }
        }
    }
}

impl SpeechAnalyzer {
    /// Starts consuming the provider's live analyzer-input sequence.
    #[doc(alias = "SpeechAnalyzer.analyzeSequence")]
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
