use crate::{api, arc, av, ns, swift, swift::SwiftMetadata, swift::abi};

use crate::swift::concurrency::{self, AsyncCallArgs, TaskPriority};

use super::SpeechModule;
use crate::swift::value::{Optional, Storage};

crate::define_swift_class!(pub CaptureInputSequenceProvider = accessor capture_input_sequence_provider_metadata);

pub(super) struct AnalyzerInputSequence {
    pub(super) value: crate::swift::value::AnyValue,
    pub(super) witness: *const (),
}

unsafe impl Send for AnalyzerInputSequence {}

#[link(name = "Speech", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s6Speech28CaptureInputSequenceProviderCMa"]
    fn capture_input_sequence_provider_metadata();

    #[link_name = "$s6Speech28CaptureInputSequenceProviderC14captureSessionSo09AVCaptureG0Cvg"]
    fn capture_input_sequence_provider_capture_session();

    #[link_name = "$s6Speech28CaptureInputSequenceProviderC14analyzerInputsQrvg"]
    fn capture_input_sequence_provider_analyzer_inputs();

    #[link_name = "$s6Speech28CaptureInputSequenceProviderC14analyzerInputsQrvpQOMQ"]
    static CAPTURE_INPUT_SEQUENCE_PROVIDER_ANALYZER_INPUTS_DESCRIPTOR: u8;

    #[link_name = "$s6Speech28CaptureInputSequenceProviderC19providerWithSession4from010compatibleG08priorityACSo15AVCaptureDeviceC_SayAA0A6Module_pGScPSgtYaKFZ"]
    fn provider_with_session();

    #[link_name = "$s6Speech28CaptureInputSequenceProviderC19providerWithSession4from010compatibleG08priorityACSo15AVCaptureDeviceC_SayAA0A6Module_pGScPSgtYaKFZTu"]
    static PROVIDER_WITH_SESSION_ASYNC_FN: u8;
}

crate::define_swift_marker!(
    pub(super) AnalyzerInputs =
        opaque (&raw const CAPTURE_INPUT_SEQUENCE_PROVIDER_ANALYZER_INPUTS_DESCRIPTOR).cast(), 0
);

impl CaptureInputSequenceProvider {
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
        unsafe {
            concurrency::call_async_result(
                provider_with_session as *const (),
                &raw const PROVIDER_WITH_SESSION_ASYNC_FN,
                (
                    device.retained(),
                    swift::Array::from_slice(modules),
                    // The call takes a `TaskPriority?`, which these bindings
                    // always leave to the runtime.
                    Storage::<Optional<TaskPriority>>::none(),
                ),
                |(device, modules, priority)| {
                    AsyncCallArgs::new()
                        // A static method's `self` is the class metadata.
                        .swift_self(<Self as SwiftMetadata>::metadata().cast_mut().cast())
                        .arg(0, device.as_ptr().cast())
                        .arg(1, modules.as_raw())
                        .arg(2, priority.as_mut_ptr())
                },
                |_, provider| arc::R::from_raw(provider.cast()),
                callback,
            );
        }
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
        concurrency::future_from(|callback| Self::with_session_handler(device, modules, callback))
    }

    #[doc(alias = "CaptureInputSequenceProvider.captureSession")]
    #[api::available(
        macos = 27.0,
        ios = 27.0,
        maccatalyst = 27.0,
        tvos = 27.0,
        visionos = 27.0
    )]
    pub fn capture_session(&self) -> arc::R<av::CaptureSession> {
        unsafe {
            arc::R::from_raw(
                abi::call::value_to_object(
                    capture_input_sequence_provider_capture_session as *const (),
                    (self as *const Self).cast(),
                )
                .cast(),
            )
        }
    }

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
