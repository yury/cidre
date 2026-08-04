use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::{api, arc, av, ns, swift, swift::abi};

use crate::swift::concurrency::{
    self, TaskPriority, swift_async_epilogue, swift_async_function_pointer,
    swift_async_load_parent, swift_async_load_resume, swift_async_prologue,
    swift_async_store_parent, swift_async_store_resume, swift_async_task_descriptor,
    swift_task_alloc, swift_task_dealloc, swift_task_switch,
};

use super::SpeechModule;
use crate::swift::value::{Optional, Storage, Value};

crate::define_swift_class!(pub CaptureInputSequenceProvider);

pub(super) struct AnalyzerInputSequence {
    pub(super) value: Value<AnalyzerInputs>,
    pub(super) witness: *const (),
}

unsafe impl Send for AnalyzerInputSequence {}

struct ProviderTask {
    device: arc::R<av::CaptureDevice>,
    modules: swift::Array<SpeechModule>,
    priority: Value<Optional<TaskPriority>>,
    result: *mut (),
    error: *mut (),
    callback: Option<
        Box<dyn FnOnce(Result<arc::R<CaptureInputSequenceProvider>, arc::R<ns::Error>>) + Send>,
    >,
}

unsafe impl Send for ProviderTask {}

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
        ProviderTask::start(device, modules, callback);
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
        let shared = crate::blocks::Shared::new();
        let comp = crate::blocks::Completion(shared.clone());
        ProviderTask::start(device, modules, move |res| shared.lock().ready(res));
        comp
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
                abi::call_value_to_object(
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
            let mut storage = Storage::<AnalyzerInputs>::new();
            abi::call_object_to_value(
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

impl ProviderTask {
    fn start<F>(device: &av::CaptureDevice, modules: &[SpeechModule], callback: F)
    where
        F: FnOnce(Result<arc::R<CaptureInputSequenceProvider>, arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            let task = Box::new(Self {
                device: device.retained(),
                modules: swift::Array::from_slice(modules),
                priority: Value::<Optional<TaskPriority>>::none(),
                result: core::ptr::null_mut(),
                error: core::ptr::null_mut(),
                callback: Some(Box::new(callback)),
            });
            let context = Box::into_raw(task).cast();
            let (_task, _) = concurrency::task_create(
                concurrency::ENQUEUED_DISCARDING_TASK_FLAGS,
                core::ptr::null(),
                (&raw const cidre_speech_capture_provider_task_descriptor).cast(),
                context,
            );
        }
    }
}

extern "C" fn cidre_speech_capture_provider_device(task: *mut ProviderTask) -> *mut () {
    unsafe { (*task).device.as_ptr().cast() }
}

extern "C" fn cidre_speech_capture_provider_modules(task: *mut ProviderTask) -> *mut () {
    unsafe { (*task).modules.as_raw() }
}

extern "C" fn cidre_speech_capture_provider_priority(task: *mut ProviderTask) -> *mut () {
    unsafe { (*task).priority.as_mut_ptr() }
}

extern "C" fn cidre_speech_capture_provider_metadata(_: *mut ProviderTask) -> *const () {
    unsafe {
        abi::call_int_to_int(capture_input_sequence_provider_metadata as *const (), 0) as *const ()
    }
}

extern "C" fn cidre_speech_capture_provider_set_result(
    task: *mut ProviderTask,
    result: *mut (),
    error: *mut (),
) {
    unsafe {
        (*task).result = result;
        (*task).error = error;
    }
}

extern "C" fn cidre_speech_capture_provider_complete(task: *mut ProviderTask) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        let mut task = Box::from_raw(task);
        let callback = task.callback.take().expect("provider callback missing");
        if task.error.is_null() {
            callback(Ok(arc::R::from_raw(task.result.cast())));
        } else {
            // Bridging returns the error box itself, so the task's reference
            // becomes the `ns::Error`'s and must not be released again here.
            callback(Err(arc::R::from_raw(
                abi::error_as_ns_error(task.error).cast(),
            )));
        }
    }));
}

swift_async_task_descriptor!(
    cidre_speech_capture_provider_task_descriptor,
    entry: provider_task_entry,
    context_size: "96",
);

/// Fills the async context from the Rust task, then tail-calls
/// `CaptureInputSequenceProvider.providerWithSession(from:compatibleWith:priority:)`.
#[unsafe(naked)]
unsafe extern "C" fn provider_task_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "bl {device}",
        "str x0, [x22, #48]",
        "ldr x0, [x22, #24]",
        "bl {modules}",
        "str x0, [x22, #56]",
        "ldr x0, [x22, #24]",
        "bl {priority}",
        "str x0, [x22, #64]",
        "ldr x0, [x22, #24]",
        "bl {metadata}",
        "str x0, [x22, #72]",
        // Word 1 of the async function pointer is the callee's context size.
        "adrp x8, {provider_async}@GOTPAGE",
        "ldr x8, [x8, {provider_async}@GOTPAGEOFF]",
        "ldr w0, [x8, #4]",
        "bl {task_alloc}",
        "mov x9, x0",
        "str x9, [x22, #40]",
        swift_async_store_parent!(),
        swift_async_store_resume!("{resume}"),
        "ldr x0, [x22, #48]",
        "ldr x1, [x22, #56]",
        "ldr x2, [x22, #64]",
        "ldr x20, [x22, #72]",
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {provider}",
        device = sym cidre_speech_capture_provider_device,
        modules = sym cidre_speech_capture_provider_modules,
        priority = sym cidre_speech_capture_provider_priority,
        metadata = sym cidre_speech_capture_provider_metadata,
        provider_async = sym PROVIDER_WITH_SESSION_ASYNC_FN,
        provider = sym provider_with_session,
        task_alloc = sym swift_task_alloc,
        resume = sym provider_task_resume,
    );
}

/// Resumed with the provider in `x0` or the thrown error in `x20`. Hops to a
/// plain frame before handing either to Rust.
#[unsafe(naked)]
unsafe extern "C" fn provider_task_resume() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "48", fp: "32", ctx: "24"),
        swift_async_load_parent!(),
        "str x9, [sp, #16]",
        "str x0, [sp, #8]",
        "str x20, [sp]",
        "mov x0, x22",
        "bl {task_dealloc}",
        "ldr x9, [sp, #16]",
        "ldr x0, [x9, #24]",
        "ldr x1, [sp, #8]",
        "ldr x2, [sp]",
        "bl {set_result}",
        "ldr x22, [sp, #16]",
        swift_async_function_pointer!("{finish}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "48", fp: "32"),
        "b {task_switch}",
        task_dealloc = sym swift_task_dealloc,
        set_result = sym cidre_speech_capture_provider_set_result,
        finish = sym provider_task_finish,
        task_switch = sym swift_task_switch,
    );
}

/// Runs the Rust completion, then returns to whoever created the task.
#[unsafe(naked)]
unsafe extern "C" fn provider_task_finish() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {complete}",
        "ldr x22, [sp, #8]",
        "ldr x9, [x22, #16]",
        swift_async_load_resume!(),
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        complete = sym cidre_speech_capture_provider_complete,
    );
}
