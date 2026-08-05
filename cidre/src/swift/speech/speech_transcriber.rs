use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::{
    api, arc, ns,
    swift::abi,
    swift::foundation::{self, AttrStringValue},
};

use crate::swift::concurrency::{
    self, ASYNC_ITERATOR_CONFORMANCE, ASYNC_ITERATOR_NEXT_ASYNC_FN,
    ASYNC_SEQUENCE_ASSOCIATED_TYPES, async_iterator_next, async_sequence_make_iterator,
    swift_async_epilogue, swift_async_function_pointer, swift_async_load_parent,
    swift_async_load_resume, swift_async_prologue, swift_async_task_descriptor,
    swift_opaque_iterator_typeref, swift_task_alloc, swift_task_dealloc, swift_task_switch,
};

use super::locale;
use crate::swift::value::{AnyValue, DynamicStorage, Storage};

crate::define_swift_class!(pub SpeechTranscriber = accessor speech_transcriber_metadata);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TranscriberPreset {
    #[default]
    Transcription,
    TranscriptionWithAlternatives,
    TimeIndexedTranscriptionWithAlternatives,
    ProgressiveTranscription,
    TimeIndexedProgressiveTranscription,
}

#[link(name = "Speech", kind = "framework")]
#[link(name = "swiftFoundation")]
unsafe extern "C" {
    #[link_name = "$s6Speech0A11TranscriberCMa"]
    fn speech_transcriber_metadata();

    #[link_name = "$s6Speech0A11TranscriberC11isAvailableSbvgZ"]
    fn speech_transcriber_is_available();

    #[link_name = "$s6Speech0A11TranscriberC6PresetVMa"]
    fn speech_transcriber_preset_metadata();

    #[link_name = "$s6Speech0A11TranscriberC6PresetV13transcriptionAEvgZ"]
    fn transcription_preset();

    #[link_name = "$s6Speech0A11TranscriberC6PresetV29transcriptionWithAlternativesAEvgZ"]
    fn transcription_with_alternatives_preset();

    #[link_name = "$s6Speech0A11TranscriberC6PresetV40timeIndexedTranscriptionWithAlternativesAEvgZ"]
    fn time_indexed_transcription_with_alternatives_preset();

    #[link_name = "$s6Speech0A11TranscriberC6PresetV24progressiveTranscriptionAEvgZ"]
    fn progressive_transcription_preset();

    #[link_name = "$s6Speech0A11TranscriberC6PresetV35timeIndexedProgressiveTranscriptionAEvgZ"]
    fn time_indexed_progressive_transcription_preset();

    #[link_name = "$s6Speech0A11TranscriberC6locale6presetAC10Foundation6LocaleV_AC6PresetVtcfC"]
    fn speech_transcriber_init();

    #[link_name = "$s6Speech0A11TranscriberC7resultsQrvg"]
    fn speech_transcriber_results();

    #[link_name = "$s6Speech0A11TranscriberC7resultsQrvpQOMQ"]
    static SPEECH_TRANSCRIBER_RESULTS_DESCRIPTOR: u8;

    #[link_name = "$s6Speech0A11TranscriberC6ResultVMa"]
    fn speech_transcriber_result_metadata();

    #[link_name = "$s6Speech0A11TranscriberC6ResultV4text10Foundation16AttributedStringVvg"]
    fn speech_transcriber_result_text();

}

crate::define_swift_marker!(SpeechTranscriberPreset = accessor speech_transcriber_preset_metadata);

crate::define_swift_marker!(SwiftError = mangled "s5Error_p");

impl SpeechTranscriber {
    #[doc(alias = "SpeechTranscriber.isAvailable")]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    pub fn is_available() -> bool {
        unsafe {
            let metadata = <SpeechTranscriber as crate::swift::SwiftMetadata>::metadata().cast();
            abi::call_static0_bool(speech_transcriber_is_available as *const (), metadata)
        }
    }

    /// Creates a transcriber using `Foundation.Locale(identifier:)` and one of
    /// Speech's standard presets.
    #[doc(alias = "SpeechTranscriber.init(locale:preset:)")]
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]
    pub fn with_locale_id(locale_id: &str, preset: TranscriberPreset) -> arc::R<Self> {
        let preset_getter = match preset {
            TranscriberPreset::Transcription => transcription_preset as _,
            TranscriberPreset::TranscriptionWithAlternatives => {
                transcription_with_alternatives_preset as _
            }
            TranscriberPreset::TimeIndexedTranscriptionWithAlternatives => {
                time_indexed_transcription_with_alternatives_preset as _
            }
            TranscriberPreset::ProgressiveTranscription => progressive_transcription_preset as _,
            TranscriberPreset::TimeIndexedProgressiveTranscription => {
                time_indexed_progressive_transcription_preset as _
            }
        };

        unsafe {
            arc::R::from_raw(
                locale::transcriber_with_id_and_preset::<SpeechTranscriberPreset>(
                    locale_id,
                    preset_getter,
                    speech_transcriber_metadata as _,
                    speech_transcriber_init as _,
                )
                .cast(),
            )
        }
    }

    /// Iterates `SpeechTranscriber.results` on a Swift concurrency task.
    ///
    /// The callback receives each transcription as a Rust `String`, `Ok(None)`
    /// when the sequence ends, or the thrown error when iteration fails.
    #[doc(alias = "SpeechTranscriber.results")]
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
                speech_transcriber_results as *const (),
                (&raw const SPEECH_TRANSCRIBER_RESULTS_DESCRIPTOR).cast(),
                &raw const cidre_speech_transcriber_results_iterator_type_start,
                &raw const cidre_speech_transcriber_results_iterator_type_end,
                speech_transcriber_result_metadata as *const (),
                "6Speech0A11TranscriberC6ResultVSg",
                speech_transcriber_result_text as *const (),
                callback,
            );
        }
    }
}

struct ResultsOwner(*const ());

unsafe impl Send for ResultsOwner {}

impl Drop for ResultsOwner {
    fn drop(&mut self) {
        unsafe { abi::object_release(self.0) }
    }
}

struct ResultsSequence {
    _value: AnyValue,
    _witness: *const (),
}

unsafe impl Send for ResultsSequence {}

pub(super) struct ResultsTask {
    // Declaration order is drop order: the iterator borrows the sequence, and
    // the sequence borrows the object, so each must be destroyed before the
    // thing that keeps it alive.
    iterator: AnyValue,
    _sequence: ResultsSequence,
    _owner: ResultsOwner,
    iterator_witness: *const (),
    result: DynamicStorage,
    payload_metadata: *const abi::TypeMetadata,
    text_getter: *const (),
    error: Storage<SwiftError>,
    failed: bool,
    callback: Box<dyn FnMut(Result<Option<std::string::String>, arc::R<ns::Error>>) + Send>,
}

unsafe impl Send for ResultsTask {}

impl ResultsTask {
    #[allow(clippy::too_many_arguments)]
    pub(super) unsafe fn start<F>(
        object: *const (),
        results_getter: *const (),
        descriptor: *const (),
        iterator_type_start: *const u8,
        iterator_type_end: *const u8,
        result_metadata_getter: *const (),
        optional_result_mangled_name: &str,
        text_getter: *const (),
        callback: F,
    ) where
        F: FnMut(Result<Option<std::string::String>, arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            let sequence_metadata = abi::opaque_type_metadata(descriptor, 0);
            let sequence_witness = abi::opaque_type_conformance(descriptor, 1);
            let mut sequence_storage = DynamicStorage::new(sequence_metadata);
            abi::call_object_to_value(results_getter, object, sequence_storage.as_mut_ptr());
            let mut sequence_value = sequence_storage.assume_init();

            let type_len = iterator_type_end.offset_from(iterator_type_start) as usize;
            let iterator_type = core::slice::from_raw_parts(iterator_type_start, type_len);
            let iterator_metadata = abi::type_by_mangled_name_bytes(iterator_type);
            assert!(
                !iterator_metadata.is_null(),
                "Speech results iterator metadata must exist"
            );

            let mut iterator_storage = DynamicStorage::new(iterator_metadata);
            concurrency::call_make_async_iterator(
                async_sequence_make_iterator as *const (),
                sequence_value.as_mut_ptr(),
                sequence_metadata,
                sequence_witness,
                iterator_storage.as_mut_ptr(),
            );
            let iterator = iterator_storage.assume_init();

            let iterator_witness = abi::associated_conformance_witness(
                sequence_witness,
                sequence_metadata,
                iterator_metadata,
                (&raw const ASYNC_SEQUENCE_ASSOCIATED_TYPES).cast(),
                (&raw const ASYNC_ITERATOR_CONFORMANCE).cast(),
            );
            let payload_metadata =
                abi::call_int_to_int(result_metadata_getter, 0) as *const abi::TypeMetadata;
            let result_metadata = abi::type_by_mangled_name(optional_result_mangled_name);

            let task = Box::new(Self {
                iterator,
                _sequence: ResultsSequence {
                    _value: sequence_value,
                    _witness: sequence_witness,
                },
                _owner: ResultsOwner(abi::object_retain(object)),
                iterator_witness,
                result: DynamicStorage::new(result_metadata),
                payload_metadata,
                text_getter,
                error: Storage::new(),
                failed: false,
                callback: Box::new(callback),
            });
            let context = Box::into_raw(task).cast();
            let (_task, _) = concurrency::task_create(
                concurrency::ENQUEUED_DISCARDING_TASK_FLAGS,
                core::ptr::null(),
                (&raw const cidre_speech_transcriber_results_task_descriptor).cast(),
                context,
            );
        }
    }
}

extern "C" fn cidre_speech_transcriber_results_result(task: *mut ResultsTask) -> *mut () {
    unsafe { (*task).result.as_mut_ptr() }
}

extern "C" fn cidre_speech_transcriber_results_error(task: *mut ResultsTask) -> *mut () {
    unsafe { (*task).error.as_mut_ptr() }
}

extern "C" fn cidre_speech_transcriber_results_iterator(task: *mut ResultsTask) -> *mut () {
    unsafe { (*task).iterator.as_mut_ptr() }
}

extern "C" fn cidre_speech_transcriber_results_iterator_metadata(
    task: *mut ResultsTask,
) -> *const abi::TypeMetadata {
    unsafe { (*task).iterator.metadata() }
}

extern "C" fn cidre_speech_transcriber_results_iterator_witness(
    task: *mut ResultsTask,
) -> *const () {
    unsafe { (*task).iterator_witness }
}

extern "C" fn cidre_speech_transcriber_results_set_error(task: *mut ResultsTask, error: *mut ()) {
    unsafe {
        let task = &mut *task;
        task.failed = !error.is_null();
    }
}

extern "C" fn cidre_speech_transcriber_results_process(task: *mut ResultsTask) -> bool {
    match catch_unwind(AssertUnwindSafe(|| unsafe {
        let task = &mut *task;
        if task.failed {
            // `next(isolation:)` writes its typed error indirectly into this
            // storage. An `any Error` existential is a single owned pointer to
            // the error box, so hand that reference to the `ns::Error` rather
            // than destroying it.
            let error = task.error.as_ptr().cast::<*mut ()>().read();
            (task.callback)(Err(arc::R::from_raw(abi::error_as_ns_error(error).cast())));
            return false;
        }

        let tag = abi::get_enum_tag_single_payload(task.result.as_ptr(), 1, task.payload_metadata);
        if tag == 1 {
            abi::destroy_value(task.result.as_mut_ptr(), task.result.metadata());
            (task.callback)(Ok(None));
            return false;
        }

        let text_getter = task.text_getter;
        let payload = OwnedPayload {
            value: task.result.as_mut_ptr(),
            metadata: task.payload_metadata,
        };
        let value = speech_result_text(payload.value.cast_const(), text_getter);
        drop(payload);

        (task.callback)(Ok(Some(value)));
        true
    })) {
        Ok(keep_going) => {
            if !keep_going {
                unsafe { drop(Box::from_raw(task)) };
            }
            keep_going
        }
        Err(_) => {
            unsafe { drop(Box::from_raw(task)) };
            false
        }
    }
}

/// Destroys a Swift value when it goes out of scope, including while the text
/// conversion below unwinds.
struct OwnedPayload {
    value: *mut (),
    metadata: *const abi::TypeMetadata,
}

impl Drop for OwnedPayload {
    fn drop(&mut self) {
        unsafe { abi::destroy_value(self.value, self.metadata) }
    }
}

unsafe fn speech_result_text(result: *const (), text_getter: *const ()) -> std::string::String {
    unsafe {
        let mut storage = Storage::<AttrStringValue>::new();
        abi::call_value_to_value(text_getter, result, storage.as_mut_ptr());
        foundation::AttrString::from_storage(storage).to_rust_string()
    }
}

swift_opaque_iterator_typeref!(
    cidre_speech_transcriber_results_iterator_type_start
        ..= cidre_speech_transcriber_results_iterator_type_end,
    descriptor: SPEECH_TRANSCRIBER_RESULTS_DESCRIPTOR,
);

swift_async_task_descriptor!(
    cidre_speech_transcriber_results_task_descriptor,
    entry: results_task_entry,
    context_size: "96",
);

/// Allocates the callee context, wires up the resume symbol, and tail-calls
/// `AsyncIteratorProtocol.next(isolation:)`.
///
/// Shared by the first iteration and by every subsequent one, which is why the
/// context slots are re-read rather than kept in registers.
macro_rules! results_task_next {
    () => {
        concat!(
            // Word 1 of the async function pointer is the callee's context size.
            "adrp x8, {next_async}@GOTPAGE\n",
            "ldr x8, [x8, {next_async}@GOTPAGEOFF]\n",
            "ldr w0, [x8, #4]\n",
            "bl {task_alloc}\n",
            "mov x9, x0\n",
            "str x9, [x22, #72]\n",
            $crate::swift::concurrency::swift_async_store_parent!(), "\n",
            $crate::swift::concurrency::swift_async_store_resume!("{resume}"), "\n",
            "ldr x0, [x22, #32]\n",
            "mov x1, #0\n",
            "mov x2, #0\n",
            "ldr x3, [x22, #40]\n",
            "ldr x4, [x22, #56]\n",
            "ldr x5, [x22, #64]\n",
            "ldr x20, [x22, #48]\n",
            "mov x22, x9\n",
            $crate::swift::concurrency::swift_async_epilogue!(frame: "32", fp: "16"), "\n",
            "b {next}",
        )
    };
}

/// Fills the async context from the Rust task, then starts iterating.
#[unsafe(naked)]
unsafe extern "C" fn results_task_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "bl {result}",
        "str x0, [x22, #32]",
        "ldr x0, [x22, #24]",
        "bl {error}",
        "str x0, [x22, #40]",
        "ldr x0, [x22, #24]",
        "bl {iterator}",
        "str x0, [x22, #48]",
        "ldr x0, [x22, #24]",
        "bl {iterator_metadata}",
        "str x0, [x22, #56]",
        "ldr x0, [x22, #24]",
        "bl {iterator_witness}",
        "str x0, [x22, #64]",
        results_task_next!(),
        result = sym cidre_speech_transcriber_results_result,
        error = sym cidre_speech_transcriber_results_error,
        iterator = sym cidre_speech_transcriber_results_iterator,
        iterator_metadata = sym cidre_speech_transcriber_results_iterator_metadata,
        iterator_witness = sym cidre_speech_transcriber_results_iterator_witness,
        next_async = sym ASYNC_ITERATOR_NEXT_ASYNC_FN,
        next = sym async_iterator_next,
        task_alloc = sym swift_task_alloc,
        resume = sym results_task_resume,
    );
}

/// Resumed once per element, with any thrown error in `x20`. Hops to a plain
/// frame before calling back into Rust.
#[unsafe(naked)]
unsafe extern "C" fn results_task_resume() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        swift_async_load_parent!(),
        "str x9, [sp]",
        "str x20, [sp, #8]",
        "mov x0, x22",
        "bl {task_dealloc}",
        "ldr x0, [sp]",
        "ldr x0, [x0, #24]",
        "ldr x1, [sp, #8]",
        "bl {set_error}",
        "ldr x22, [sp]",
        swift_async_function_pointer!("{process}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {task_switch}",
        task_dealloc = sym swift_task_dealloc,
        set_error = sym cidre_speech_transcriber_results_set_error,
        process = sym results_task_process,
        task_switch = sym swift_task_switch,
    );
}

/// Delivers one element to Rust, then either asks for the next one or returns
/// to whoever created the task.
#[unsafe(naked)]
unsafe extern "C" fn results_task_process() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {process_element}",
        "ldr x22, [sp, #8]",
        "cbnz x0, 0f",
        "ldr x9, [x22, #16]",
        swift_async_load_resume!(),
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        "0:",
        "str x22, [x22, #16]",
        results_task_next!(),
        process_element = sym cidre_speech_transcriber_results_process,
        next_async = sym ASYNC_ITERATOR_NEXT_ASYNC_FN,
        next = sym async_iterator_next,
        task_alloc = sym swift_task_alloc,
        resume = sym results_task_resume,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::swift::SwiftMetadata;

    /// The failure path reads the thrown error straight out of its indirect
    /// return slot, which is only valid while `any Error` stays one word wide.
    #[test]
    fn error_existential_is_a_single_pointer() {
        let metadata = SwiftError::metadata();
        assert!(!metadata.is_null(), "Swift.Error metadata must exist");

        let layout = unsafe { abi::value_layout(metadata) };
        assert_eq!(core::mem::size_of::<*mut ()>(), layout.size);
    }
}
