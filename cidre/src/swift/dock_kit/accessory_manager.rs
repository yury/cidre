use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::{
    api, arc, ns,
    swift::dock_kit::StateChanges,
    swift::{
        abi,
        concurrency::{
            self, swift_async_epilogue, swift_async_function_pointer, swift_async_load_parent,
            swift_async_load_resume, swift_async_prologue, swift_async_store_parent,
            swift_async_store_resume, swift_async_task_descriptor, swift_task_alloc,
            swift_task_dealloc, swift_task_switch,
        },
    },
};

crate::define_swift_class!(pub AccessoryManager);

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s7DockKit0A16AccessoryManagerCMa"]
    fn dock_accessory_manager_metadata();

    #[link_name = "$s7DockKit0A16AccessoryManagerC6sharedACvgZ"]
    fn dock_accessory_manager_shared();

    #[link_name = "$s7DockKit0A16AccessoryManagerC23isSystemTrackingEnabledSbvgTj"]
    fn dock_accessory_manager_is_system_tracking_enabled();

    #[link_name = "$s7DockKit0A16AccessoryManagerC21accessoryStateChangesAA0aC0C0fG0VvgTj"]
    fn dock_accessory_manager_accessory_state_changes();

    #[link_name = "$s7DockKit0A16AccessoryManagerC24setSystemTrackingEnabledyySbYaKFTj"]
    fn dock_accessory_manager_set_system_tracking_enabled();

    #[link_name = "$s7DockKit0A16AccessoryManagerC24setSystemTrackingEnabledyySbYaKFTjTu"]
    static SET_SYSTEM_TRACKING_ENABLED_ASYNC_FN: u8;
}

impl AccessoryManager {
    /// DockKit `DockAccessoryManager.shared`.
    #[doc(alias = "DockAccessoryManager.shared")]
    #[inline]
    pub fn shared() -> arc::R<Self> {
        unsafe {
            // Swift emits a class metadata access before calling this static getter.
            let metadata =
                abi::call_int_to_int(dock_accessory_manager_metadata as *const (), 0) as *const ();
            arc::R::from_raw(
                abi::call_static0_object(dock_accessory_manager_shared as *const (), metadata)
                    .cast(),
            )
        }
    }

    #[doc(alias = "DockAccessoryManager.isSystemTrackingEnabled")]
    #[inline]
    pub fn is_system_tracking_enabled(&self) -> bool {
        unsafe {
            abi::call_object_to_bool(
                dock_accessory_manager_is_system_tracking_enabled as *const (),
                (self as *const Self).cast(),
            )
        }
    }

    #[doc(alias = "DockAccessoryManager.accessoryStateChanges")]
    #[inline]
    pub fn accessory_state_changes(&self) -> Result<StateChanges, arc::R<ns::Error>> {
        unsafe {
            let mut storage = StateChanges::storage();
            let error = abi::call_object_to_throwing_value(
                dock_accessory_manager_accessory_state_changes as *const (),
                (self as *const Self).cast(),
                storage.as_mut_ptr(),
            );
            if error.is_null() {
                Ok(StateChanges::from_storage(storage))
            } else {
                // Bridging returns the error box itself, so our reference
                // becomes the `ns::Error`'s and must not be released again.
                Err(arc::R::from_raw(abi::error_as_ns_error(error).cast()))
            }
        }
    }

    /// Turns system tracking on or off.
    #[doc(alias = "DockAccessoryManager.setSystemTrackingEnabled(_:)")]
    #[api::available(macos = 14.0, ios = 17.0, visionos = 1.0)]
    pub fn set_system_tracking_enabled_handler<F>(&self, enabled: bool, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        SetTrackingTask::start(self, enabled, callback);
    }

    #[doc(alias = "DockAccessoryManager.setSystemTrackingEnabled(_:)")]
    #[cfg(feature = "async")]
    #[api::available(macos = 14.0, ios = 17.0, visionos = 1.0)]
    pub fn set_system_tracking_enabled(
        &self,
        enabled: bool,
    ) -> impl Future<Output = Result<(), arc::R<ns::Error>>> {
        let shared = crate::blocks::Shared::new();
        let comp = crate::blocks::Completion(shared.clone());
        SetTrackingTask::start(self, enabled, move |res| shared.lock().ready(res));
        comp
    }
}

struct SetTrackingTask {
    manager: arc::R<AccessoryManager>,
    enabled: bool,
    error: *mut (),
    callback: Option<Box<dyn FnOnce(Result<(), arc::R<ns::Error>>) + Send>>,
}

unsafe impl Send for SetTrackingTask {}

impl SetTrackingTask {
    fn start<F>(manager: &AccessoryManager, enabled: bool, callback: F)
    where
        F: FnOnce(Result<(), arc::R<ns::Error>>) + Send + 'static,
    {
        unsafe {
            let task = Box::new(Self {
                manager: arc::Retain::retained(manager),
                enabled,
                error: core::ptr::null_mut(),
                callback: Some(Box::new(callback)),
            });
            let context = Box::into_raw(task).cast();
            let (_task, _) = concurrency::task_create(
                concurrency::ENQUEUED_DISCARDING_TASK_FLAGS,
                core::ptr::null(),
                (&raw const cidre_dk_set_tracking_task_descriptor).cast(),
                context,
            );
        }
    }
}

extern "C" fn cidre_dk_set_tracking_manager(task: *mut SetTrackingTask) -> *mut () {
    unsafe { (*task).manager.as_ptr().cast() }
}

extern "C" fn cidre_dk_set_tracking_enabled(task: *mut SetTrackingTask) -> usize {
    unsafe { (*task).enabled as usize }
}

extern "C" fn cidre_dk_set_tracking_set_error(task: *mut SetTrackingTask, error: *mut ()) {
    unsafe { (*task).error = error }
}

extern "C" fn cidre_dk_set_tracking_complete(task: *mut SetTrackingTask) {
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        let mut task = Box::from_raw(task);
        let callback = task.callback.take().expect("tracking callback missing");
        if task.error.is_null() {
            callback(Ok(()));
        } else {
            callback(Err(arc::R::from_raw(
                abi::error_as_ns_error(task.error).cast(),
            )));
        }
    }));
}

swift_async_task_descriptor!(
    cidre_dk_set_tracking_task_descriptor,
    entry: set_tracking_task_entry,
    context_size: "96",
);

/// Fills the async context from the Rust task, then tail-calls
/// `DockAccessoryManager.setSystemTrackingEnabled(_:)`.
#[unsafe(naked)]
unsafe extern "C" fn set_tracking_task_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "bl {manager}",
        "str x0, [x22, #48]",
        "ldr x0, [x22, #24]",
        "bl {enabled}",
        "str x0, [x22, #56]",
        "adrp x8, {set_async}@GOTPAGE",
        "ldr x8, [x8, {set_async}@GOTPAGEOFF]",
        "ldr w0, [x8, #4]",
        "bl {task_alloc}",
        "mov x9, x0",
        "str x9, [x22, #40]",
        swift_async_store_parent!(),
        swift_async_store_resume!("{resume}"),
        "ldr x0, [x22, #56]",
        "ldr x20, [x22, #48]",
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {set_enabled}",
        manager = sym cidre_dk_set_tracking_manager,
        enabled = sym cidre_dk_set_tracking_enabled,
        set_async = sym SET_SYSTEM_TRACKING_ENABLED_ASYNC_FN,
        set_enabled = sym dock_accessory_manager_set_system_tracking_enabled,
        task_alloc = sym swift_task_alloc,
        resume = sym set_tracking_task_resume,
    );
}

/// Resumed with any thrown error in `x20`.
#[unsafe(naked)]
unsafe extern "C" fn set_tracking_task_resume() {
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
        set_error = sym cidre_dk_set_tracking_set_error,
        finish = sym set_tracking_task_finish,
        task_switch = sym swift_task_switch,
    );
}

/// Runs the Rust completion, then returns to whoever created the task.
#[unsafe(naked)]
unsafe extern "C" fn set_tracking_task_finish() {
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
        complete = sym cidre_dk_set_tracking_complete,
    );
}
