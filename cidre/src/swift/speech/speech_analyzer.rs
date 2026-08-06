use crate::{api, arc, swift, swift::abi};

#[cfg(feature = "av")]
use crate::ns;

#[cfg(feature = "av")]
use crate::swift::concurrency::{
    self, swift_async_epilogue, swift_async_function_pointer, swift_async_load_parent,
    swift_async_load_resume, swift_async_prologue, swift_async_store_parent,
    swift_async_store_resume, swift_async_task_descriptor, swift_task_alloc, swift_task_dealloc,
    swift_task_switch,
};

#[cfg(feature = "av")]
use std::panic::{AssertUnwindSafe, catch_unwind};

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
        AnalyzerTask::start_analysis(self, provider, callback);
    }
}

#[cfg(feature = "av")]
struct AnalyzerTask {
    // Declaration order is drop order: the input sequence comes out of the
    // provider, so it must be destroyed before the provider is released.
    input: super::capture_input_sequence_provider::AnalyzerInputSequence,
    _provider: arc::R<CaptureInputSequenceProvider>,
    analyzer: arc::R<SpeechAnalyzer>,
    error: *mut (),
    callback: Option<Box<dyn FnOnce(Result<(), arc::R<ns::Error>>) + Send>>,
}

#[cfg(feature = "av")]
unsafe impl Send for AnalyzerTask {}

#[cfg(feature = "av")]
impl AnalyzerTask {
    fn start_analysis<F>(
        analyzer: &SpeechAnalyzer,
        provider: &CaptureInputSequenceProvider,
        callback: F,
    ) where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            let task = Box::new(Self {
                input: provider.analyzer_inputs(),
                _provider: arc::Retain::retained(provider),
                analyzer: arc::Retain::retained(analyzer),
                error: core::ptr::null_mut(),
                callback: Some(Box::new(callback)),
            });
            let context = Box::into_raw(task).cast();
            let (_task, _) = concurrency::task_create(
                concurrency::ENQUEUED_DISCARDING_TASK_FLAGS,
                core::ptr::null(),
                (&raw const cidre_speech_analyzer_task_descriptor).cast(),
                context,
            );
        }
    }
}

#[cfg(feature = "av")]
extern "C" fn cidre_speech_analyzer_object(task: *mut AnalyzerTask) -> *mut () {
    unsafe { (*task).analyzer.as_ptr().cast() }
}

#[cfg(feature = "av")]
extern "C" fn cidre_speech_analyzer_input(task: *mut AnalyzerTask) -> *mut () {
    unsafe { (*task).input.value.as_mut_ptr() }
}

#[cfg(feature = "av")]
extern "C" fn cidre_speech_analyzer_input_metadata(
    task: *mut AnalyzerTask,
) -> *const abi::TypeMetadata {
    unsafe { (*task).input.value.metadata() }
}

#[cfg(feature = "av")]
extern "C" fn cidre_speech_analyzer_input_witness(task: *mut AnalyzerTask) -> *const () {
    unsafe { (*task).input.witness }
}

#[cfg(feature = "av")]
extern "C" fn cidre_speech_analyzer_set_error(task: *mut AnalyzerTask, error: *mut ()) {
    unsafe { (*task).error = error }
}

#[cfg(feature = "av")]
extern "C" fn cidre_speech_analyzer_complete(task: *mut AnalyzerTask) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        let mut task = Box::from_raw(task);
        let callback = task.callback.take().expect("analyzer callback missing");
        if task.error.is_null() {
            callback(Ok(()));
        } else {
            // Bridging returns the error box itself, so the task's reference
            // becomes the `ns::Error`'s and must not be released again here.
            callback(Err(arc::R::from_raw(
                abi::error_as_ns_error(task.error).cast(),
            )));
        }
    }));
}

// The whole invocation is gated, not just the items it expands to: without
// `av` the macro itself is not imported.
#[cfg(feature = "av")]
swift_async_task_descriptor!(
    cidre_speech_analyzer_task_descriptor,
    entry: analyzer_task_entry,
    context_size: "96",
);

/// Fills the async context from the Rust task, then tail-calls
/// `SpeechAnalyzer.analyzeSequence(_:)`.
#[cfg(feature = "av")]
#[unsafe(naked)]
unsafe extern "C" fn analyzer_task_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "bl {object}",
        "str x0, [x22, #48]",
        "ldr x0, [x22, #24]",
        "bl {input}",
        "str x0, [x22, #56]",
        "ldr x0, [x22, #24]",
        "bl {input_metadata}",
        "str x0, [x22, #64]",
        "ldr x0, [x22, #24]",
        "bl {input_witness}",
        "str x0, [x22, #72]",
        // Word 1 of the async function pointer is the callee's context size.
        "adrp x8, {analyze_async}@GOTPAGE",
        "ldr x8, [x8, {analyze_async}@GOTPAGEOFF]",
        "ldr w0, [x8, #4]",
        "bl {task_alloc}",
        "mov x9, x0",
        "str x9, [x22, #40]",
        swift_async_store_parent!(),
        swift_async_store_resume!("{resume}"),
        "ldr x0, [x22, #56]",
        "ldr x1, [x22, #64]",
        "ldr x2, [x22, #72]",
        "ldr x20, [x22, #48]",
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {analyze}",
        object = sym cidre_speech_analyzer_object,
        input = sym cidre_speech_analyzer_input,
        input_metadata = sym cidre_speech_analyzer_input_metadata,
        input_witness = sym cidre_speech_analyzer_input_witness,
        analyze_async = sym ANALYZE_SEQUENCE_ASYNC_FN,
        analyze = sym speech_analyzer_analyze_sequence,
        task_alloc = sym swift_task_alloc,
        resume = sym analyzer_task_resume,
    );
}

/// Resumed by Swift once the sequence is fully consumed or has thrown, with any
/// error in `x20`. Hops to a plain frame before handing it to Rust.
#[cfg(feature = "av")]
#[unsafe(naked)]
unsafe extern "C" fn analyzer_task_resume() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        swift_async_load_parent!(),
        "str x9, [sp]",
        "mov x0, x22",
        "bl {task_dealloc}",
        "ldr x9, [sp]",
        "ldr x0, [x9, #24]",
        "mov x1, x20",
        "bl {set_error}",
        "ldr x22, [sp]",
        swift_async_function_pointer!("{finish}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {task_switch}",
        task_dealloc = sym swift_task_dealloc,
        set_error = sym cidre_speech_analyzer_set_error,
        finish = sym analyzer_task_finish,
        task_switch = sym swift_task_switch,
    );
}

/// Runs the Rust completion, then returns to whoever created the task.
#[cfg(feature = "av")]
#[unsafe(naked)]
unsafe extern "C" fn analyzer_task_finish() {
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
        complete = sym cidre_speech_analyzer_complete,
    );
}
