use crate::{api, swift::abi};

// used by 27.0 api only
#[cfg(any(
    all(target_os = "macos", feature = "macos_27_0"),
    all(target_os = "ios", feature = "ios_27_0"),
    all(target_os = "tvos", feature = "tvos_27_0"),
    all(target_os = "visionos", feature = "visionos_27_0"),
    all(target_os = "ios", target_abi = "macabi", feature = "maccatalyst_27_0")
))]
use {
    super::SpeechModule,
    crate::{
        arc, av, ns, swift,
        swift::{
            concurrency::TaskPriority,
            value::{Optional, Storage},
        },
    },
};

// available(macos = 27.0, ios = 27.0): the class is new in 27. The type exists
// on every target, as the unavailable variants of its API name it, but only a
// 27 build reaches the framework.
crate::define_swift!(
    #[swift::class("Speech.CaptureInputSequenceProvider")]
    pub CaptureInputSequenceProvider
);

pub(super) struct AnalyzerInputSequence {
    pub(super) value: crate::swift::value::AnyValue,
    pub(super) witness: *const (),
}

unsafe impl Send for AnalyzerInputSequence {}

// `analyzerInputs` is an opaque `some AsyncSequence`, whose getter and
// descriptor the mangler does not spell.
// available(macos = 27.0, ios = 27.0)
#[link(name = "Speech", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s6Speech28CaptureInputSequenceProviderC14analyzerInputsQrvg"]
    fn capture_input_sequence_provider_analyzer_inputs();

    #[link_name = "$s6Speech28CaptureInputSequenceProviderC14analyzerInputsQrvpQOMQ"]
    static CAPTURE_INPUT_SEQUENCE_PROVIDER_ANALYZER_INPUTS_DESCRIPTOR: u8;

}

crate::define_swift_marker!(
    pub(super) AnalyzerInputs =
        opaque (&raw const CAPTURE_INPUT_SEQUENCE_PROVIDER_ANALYZER_INPUTS_DESCRIPTOR).cast(), 0
);

// The bodiless `swift::call` exists only when available, so its wrappers are
// gated the same way instead of getting an unavailable variant.
#[cfg(any(
    all(target_os = "macos", feature = "macos_27_0"),
    all(target_os = "ios", feature = "ios_27_0"),
    all(target_os = "tvos", feature = "tvos_27_0"),
    all(target_os = "visionos", feature = "visionos_27_0"),
    all(target_os = "ios", target_abi = "macabi", feature = "maccatalyst_27_0")
))]
impl CaptureInputSequenceProvider {
    #[doc(alias = "CaptureInputSequenceProvider.providerWithSession")]
    /// `priority` is the call's `TaskPriority?`, which these bindings always
    /// leave to the runtime.
    #[api::available(
        macos = 27.0,
        ios = 27.0,
        maccatalyst = 27.0,
        tvos = 27.0,
        visionos = 27.0
    )]
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
    #[api::available(
        macos = 27.0,
        ios = 27.0,
        maccatalyst = 27.0,
        tvos = 27.0,
        visionos = 27.0
    )]
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
    #[api::available(
        macos = 27.0,
        ios = 27.0,
        maccatalyst = 27.0,
        tvos = 27.0,
        visionos = 27.0
    )]
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
}

impl CaptureInputSequenceProvider {
    #[doc(alias = "CaptureInputSequenceProvider.captureSession")]
    #[api::available(
        macos = 27.0,
        ios = 27.0,
        maccatalyst = 27.0,
        tvos = 27.0,
        visionos = 27.0
    )]
    #[swift::call(
        "Speech.CaptureInputSequenceProvider(class).captureSession: \
         __C.AVCaptureSession(class) { get }"
    )]
    pub fn capture_session(&self) -> arc::R<av::CaptureSession>;

    pub(super) fn analyzer_inputs(&self) -> AnalyzerInputSequence {
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
