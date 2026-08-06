use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::{
    api, arc, av, ns, swift,
    swift::{
        abi,
        concurrency::{
            self, swift_async_epilogue, swift_async_function_pointer, swift_async_load_parent,
            swift_async_load_resume, swift_async_prologue, swift_async_store_parent,
            swift_async_store_resume, swift_async_task_descriptor, swift_task_alloc,
            swift_task_dealloc, swift_task_switch,
        },
        value::Storage,
    },
};

use super::analysis_type::AnalysisType;
use super::results::{SessionResult, SessionResultValue};

crate::define_swift_class!(pub MusicUnderstandingSession = accessor session_metadata);

#[link(name = "MusicUnderstanding", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s18MusicUnderstanding0aB7SessionCMa"]
    fn session_metadata();

    /// The allocating initializer, which takes the `AVAsset` at +1.
    #[link_name = "$s18MusicUnderstanding0aB7SessionC5assetACs8Sendable_So7AVAssetCXc_tYaKcfC"]
    fn session_init_with_asset();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC5assetACs8Sendable_So7AVAssetCXc_tYaKcfCTu"]
    static SESSION_INIT_WITH_ASSET_ASYNC_FN: u8;

    #[link_name = "$s18MusicUnderstanding0aB7SessionC7analyzeAC0C6ResultVyYaKFTj"]
    fn session_analyze();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC7analyzeAC0C6ResultVyYaKFTjTu"]
    static SESSION_ANALYZE_ASYNC_FN: u8;
}

impl MusicUnderstandingSession {
    /// Creates a session for an asset.
    #[doc(alias = "MusicUnderstandingSession.init(asset:)")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn with_asset_handler<F>(asset: &av::Asset, callback: F)
    where
        F: FnOnce(Result<arc::R<Self>, arc::R<ns::Error>>) + Send + 'static,
    {
        InitTask::start(asset, callback);
    }

    #[doc(alias = "MusicUnderstandingSession.init(asset:)")]
    #[cfg(feature = "async")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn with_asset(
        asset: &av::Asset,
    ) -> impl Future<Output = Result<arc::R<Self>, arc::R<ns::Error>>> {
        let shared = crate::blocks::Shared::new();
        let comp = crate::blocks::Completion(shared.clone());
        InitTask::start(asset, move |res| shared.lock().ready(res));
        comp
    }

    /// Runs every analysis the session supports.
    #[doc(alias = "MusicUnderstandingSession.analyze()")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn analyze_handler<F>(&self, callback: F)
    where
        F: FnOnce(Result<SessionResult, arc::R<ns::Error>>) + Send + 'static,
    {
        AnalyzeTask::start(self, None, callback);
    }

    /// Runs only the requested analyses, which is much faster than
    /// [`Self::analyze`] when a caller needs one of them.
    #[doc(alias = "MusicUnderstandingSession.analyze(for:)")]
    #[cfg(feature = "async")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn analyze_for(
        &self,
        types: &[AnalysisType],
    ) -> impl Future<Output = Result<SessionResult, arc::R<ns::Error>>> {
        let shared = crate::blocks::Shared::new();
        let comp = crate::blocks::Completion(shared.clone());
        AnalyzeTask::start(self, Some(swift::Set::from_slice(types)), move |res| {
            shared.lock().ready(res)
        });
        comp
    }

    #[doc(alias = "MusicUnderstandingSession.analyze()")]
    #[cfg(feature = "async")]
    #[api::available(macos = 27.0, ios = 27.0, tvos = 27.0, watchos = 27.0, visionos = 27.0)]
    pub fn analyze(&self) -> impl Future<Output = Result<SessionResult, arc::R<ns::Error>>> {
        let shared = crate::blocks::Shared::new();
        let comp = crate::blocks::Completion(shared.clone());
        AnalyzeTask::start(self, None, move |res| shared.lock().ready(res));
        comp
    }
}

struct InitTask {
    asset: arc::R<av::Asset>,
    result: *mut (),
    error: *mut (),
    callback: Option<
        Box<dyn FnOnce(Result<arc::R<MusicUnderstandingSession>, arc::R<ns::Error>>) + Send>,
    >,
}

unsafe impl Send for InitTask {}

impl InitTask {
    fn start<F>(asset: &av::Asset, callback: F)
    where
        F: FnOnce(Result<arc::R<MusicUnderstandingSession>, arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            let task = Box::new(Self {
                asset: asset.retained(),
                result: core::ptr::null_mut(),
                error: core::ptr::null_mut(),
                callback: Some(Box::new(callback)),
            });
            let context = Box::into_raw(task).cast();
            let (_task, _) = concurrency::task_create(
                concurrency::ENQUEUED_DISCARDING_TASK_FLAGS,
                core::ptr::null(),
                (&raw const cidre_mu_init_task_descriptor).cast(),
                context,
            );
        }
    }
}

/// The initializer consumes its argument, so hand over a reference of our own.
extern "C" fn cidre_mu_init_asset(task: *mut InitTask) -> *mut () {
    unsafe { (*task).asset.retained().into_raw().cast() }
}

extern "C" fn cidre_mu_init_metadata(_: *mut InitTask) -> *const () {
    <MusicUnderstandingSession as crate::swift::SwiftMetadata>::metadata().cast()
}

extern "C" fn cidre_mu_init_set_result(task: *mut InitTask, result: *mut (), error: *mut ()) {
    unsafe {
        (*task).result = result;
        (*task).error = error;
    }
}

extern "C" fn cidre_mu_init_complete(task: *mut InitTask) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        let mut task = Box::from_raw(task);
        let callback = task.callback.take().expect("session callback missing");
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

struct AnalyzeTask {
    session: arc::R<MusicUnderstandingSession>,
    /// Kept alive for the call; `analyze(for:)` borrows it.
    types: Option<swift::Set<AnalysisType>>,
    result: Option<Storage<SessionResultValue>>,
    error: *mut (),
    callback: Option<Box<dyn FnOnce(Result<SessionResult, arc::R<ns::Error>>) + Send>>,
}

unsafe impl Send for AnalyzeTask {}

impl AnalyzeTask {
    fn start<F>(
        session: &MusicUnderstandingSession,
        types: Option<swift::Set<AnalysisType>>,
        callback: F,
    ) where
        F: FnOnce(Result<SessionResult, arc::R<ns::Error>>) + Send + 'static,
    {
        let descriptor = if types.is_some() {
            (&raw const cidre_mu_analyze_for_task_descriptor).cast()
        } else {
            (&raw const cidre_mu_analyze_task_descriptor).cast()
        };
        unsafe {
            let task = Box::new(Self {
                session: arc::Retain::retained(session),
                types,
                result: Some(Storage::new()),
                error: core::ptr::null_mut(),
                callback: Some(Box::new(callback)),
            });
            let context = Box::into_raw(task).cast();
            let (_task, _) = concurrency::task_create(
                concurrency::ENQUEUED_DISCARDING_TASK_FLAGS,
                core::ptr::null(),
                descriptor,
                context,
            );
        }
    }
}

extern "C" fn cidre_mu_analyze_result(task: *mut AnalyzeTask) -> *mut () {
    unsafe {
        (*task)
            .result
            .as_mut()
            .expect("analyze result storage missing")
            .as_mut_ptr()
    }
}

extern "C" fn cidre_mu_analyze_session(task: *mut AnalyzeTask) -> *mut () {
    unsafe { (*task).session.as_ptr().cast() }
}

extern "C" fn cidre_mu_analyze_types(task: *mut AnalyzeTask) -> *mut () {
    unsafe {
        (*task)
            .types
            .as_ref()
            .expect("analysis types missing")
            .as_raw()
    }
}

extern "C" fn cidre_mu_analyze_set_error(task: *mut AnalyzeTask, error: *mut ()) {
    unsafe { (*task).error = error }
}

extern "C" fn cidre_mu_analyze_complete(task: *mut AnalyzeTask) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        let mut task = Box::from_raw(task);
        let callback = task.callback.take().expect("analyze callback missing");
        if task.error.is_null() {
            // Swift wrote the result in place, so take ownership of the
            // storage and let the rest of the task drop normally.
            let storage = task.result.take().expect("analyze result storage missing");
            callback(Ok(SessionResult {
                value: storage.assume_init(),
            }));
        } else {
            callback(Err(arc::R::from_raw(
                abi::error_as_ns_error(task.error).cast(),
            )));
        }
    }));
}

swift_async_task_descriptor!(
    cidre_mu_init_task_descriptor,
    entry: init_task_entry,
    context_size: "96",
);

/// Fills the async context from the Rust task, then tail-calls
/// `MusicUnderstandingSession.init(asset:)`.
#[unsafe(naked)]
unsafe extern "C" fn init_task_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "bl {asset}",
        "str x0, [x22, #48]",
        "ldr x0, [x22, #24]",
        "bl {metadata}",
        "str x0, [x22, #56]",
        // Word 1 of the async function pointer is the callee's context size.
        "adrp x8, {init_async}@GOTPAGE",
        "ldr x8, [x8, {init_async}@GOTPAGEOFF]",
        "ldr w0, [x8, #4]",
        "bl {task_alloc}",
        "mov x9, x0",
        "str x9, [x22, #40]",
        swift_async_store_parent!(),
        swift_async_store_resume!("{resume}"),
        "ldr x0, [x22, #48]",
        "ldr x20, [x22, #56]",
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {init}",
        asset = sym cidre_mu_init_asset,
        metadata = sym cidre_mu_init_metadata,
        init_async = sym SESSION_INIT_WITH_ASSET_ASYNC_FN,
        init = sym session_init_with_asset,
        task_alloc = sym swift_task_alloc,
        resume = sym init_task_resume,
    );
}

/// Resumed with the session in `x0` or the thrown error in `x20`.
#[unsafe(naked)]
unsafe extern "C" fn init_task_resume() {
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
        set_result = sym cidre_mu_init_set_result,
        finish = sym init_task_finish,
        task_switch = sym swift_task_switch,
    );
}

/// Runs the Rust completion, then returns to whoever created the task.
#[unsafe(naked)]
unsafe extern "C" fn init_task_finish() {
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
        complete = sym cidre_mu_init_complete,
    );
}

swift_async_task_descriptor!(
    cidre_mu_analyze_task_descriptor,
    entry: analyze_task_entry,
    context_size: "96",
);

/// Fills the async context from the Rust task, then tail-calls
/// `MusicUnderstandingSession.analyze()` through its dispatch thunk.
#[unsafe(naked)]
unsafe extern "C" fn analyze_task_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "bl {result}",
        "str x0, [x22, #48]",
        "ldr x0, [x22, #24]",
        "bl {session}",
        "str x0, [x22, #56]",
        "adrp x8, {analyze_async}@GOTPAGE",
        "ldr x8, [x8, {analyze_async}@GOTPAGEOFF]",
        "ldr w0, [x8, #4]",
        "bl {task_alloc}",
        "mov x9, x0",
        "str x9, [x22, #40]",
        swift_async_store_parent!(),
        swift_async_store_resume!("{resume}"),
        // The result is returned indirectly through x0.
        "ldr x0, [x22, #48]",
        "ldr x20, [x22, #56]",
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {analyze}",
        result = sym cidre_mu_analyze_result,
        session = sym cidre_mu_analyze_session,
        analyze_async = sym SESSION_ANALYZE_ASYNC_FN,
        analyze = sym session_analyze,
        task_alloc = sym swift_task_alloc,
        resume = sym analyze_task_resume,
    );
}

/// Resumed with any thrown error in `x20`; the result was written in place.
#[unsafe(naked)]
unsafe extern "C" fn analyze_task_resume() {
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
        set_error = sym cidre_mu_analyze_set_error,
        finish = sym analyze_task_finish,
        task_switch = sym swift_task_switch,
    );
}

/// Runs the Rust completion, then returns to whoever created the task.
#[unsafe(naked)]
unsafe extern "C" fn analyze_task_finish() {
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
        complete = sym cidre_mu_analyze_complete,
    );
}

#[link(name = "MusicUnderstanding", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s18MusicUnderstanding0aB7SessionC7analyze3forAC0C6ResultVShyAA12AnalysisTypeVG_tYaKFTj"]
    fn session_analyze_for();

    #[link_name = "$s18MusicUnderstanding0aB7SessionC7analyze3forAC0C6ResultVShyAA12AnalysisTypeVG_tYaKFTjTu"]
    static SESSION_ANALYZE_FOR_ASYNC_FN: u8;
}

swift_async_task_descriptor!(
    cidre_mu_analyze_for_task_descriptor,
    entry: analyze_for_task_entry,
    context_size: "96",
);

/// Same shape as [`analyze_task_entry`], with the requested analyses passed as
/// a borrowed `Set<AnalysisType>` in `x1`.
#[unsafe(naked)]
unsafe extern "C" fn analyze_for_task_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "bl {result}",
        "str x0, [x22, #48]",
        "ldr x0, [x22, #24]",
        "bl {session}",
        "str x0, [x22, #56]",
        "ldr x0, [x22, #24]",
        "bl {types}",
        "str x0, [x22, #64]",
        "adrp x8, {analyze_async}@GOTPAGE",
        "ldr x8, [x8, {analyze_async}@GOTPAGEOFF]",
        "ldr w0, [x8, #4]",
        "bl {task_alloc}",
        "mov x9, x0",
        "str x9, [x22, #40]",
        swift_async_store_parent!(),
        swift_async_store_resume!("{resume}"),
        "ldr x0, [x22, #48]",
        "ldr x1, [x22, #64]",
        "ldr x20, [x22, #56]",
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {analyze}",
        result = sym cidre_mu_analyze_result,
        session = sym cidre_mu_analyze_session,
        types = sym cidre_mu_analyze_types,
        analyze_async = sym SESSION_ANALYZE_FOR_ASYNC_FN,
        analyze = sym session_analyze_for,
        task_alloc = sym swift_task_alloc,
        resume = sym analyze_task_resume,
    );
}
