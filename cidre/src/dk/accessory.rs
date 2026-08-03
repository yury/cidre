use std::{
    alloc,
    panic::{AssertUnwindSafe, catch_unwind},
    ptr::NonNull,
};

use crate::{
    cg, swift,
    swift::{
        abi,
        async_task::{
            swift_async_epilogue, swift_async_function_pointer, swift_async_load_parent,
            swift_async_load_resume, swift_async_prologue, swift_async_task_descriptor,
            swift_task_alloc, swift_task_dealloc, swift_task_switch,
        },
    },
};

crate::define_swift_class!(pub Accessory);

pub struct StateChanges {
    ptr: NonNull<u8>,
    layout: alloc::Layout,
    metadata: *const abi::TypeMetadata,
}

pub struct StateChangesIter {
    ptr: NonNull<u8>,
    layout: alloc::Layout,
    metadata: *const abi::TypeMetadata,
}

pub struct StateChange {
    pub state: State,
    pub tracking_button_enabled: bool,
}

struct StateChangeNextTask {
    state_changes: Option<StateChanges>,
    iter: Option<StateChangesIter>,
    result: NonNull<u8>,
    result_layout: alloc::Layout,
    result_in_task: bool,
    result_metadata: *const abi::TypeMetadata,
    payload: NonNull<u8>,
    payload_layout: alloc::Layout,
    payload_metadata: *const abi::TypeMetadata,
    state: NonNull<u8>,
    state_layout: alloc::Layout,
    state_metadata: *const abi::TypeMetadata,
    callback: Box<dyn FnMut(Option<StateChange>) + Send>,
}

swift_async_task_descriptor!(
    #[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
    cidre_dk_state_changes_next_task_descriptor,
    entry: state_changes_next_task_entry,
    context_size: "64",
);

/// Allocates the callee context, wires up the resume symbol, and tail-calls
/// `Accessory.StateChanges.Iterator.next()`.
///
/// Shared by the first iteration and by every subsequent one, which is why the
/// context slots are re-read rather than kept in registers.
macro_rules! state_changes_next_call {
    () => {
        concat!(
            // Word 1 of the async function pointer is the callee's context size.
            "adrp x8, {next_async}@GOTPAGE\n",
            "ldr x8, [x8, {next_async}@GOTPAGEOFF]\n",
            "ldr w0, [x8, #4]\n",
            "bl {task_alloc}\n",
            "mov x9, x0\n",
            "str x9, [x22, #48]\n",
            $crate::swift::async_task::swift_async_store_parent!(), "\n",
            $crate::swift::async_task::swift_async_store_resume!("{resume}"), "\n",
            "ldr x0, [x22, #40]\n",
            "ldr x20, [x22, #32]\n",
            "mov x22, x9\n",
            "mov x21, #0\n",
            $crate::swift::async_task::swift_async_epilogue!(frame: "32", fp: "16"), "\n",
            "b {next}",
        )
    };
}

/// Moves the result buffer into the task's own allocation, then hops to a plain
/// frame so the Rust setup below can run off the async entry path.
#[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
#[unsafe(naked)]
unsafe extern "C" fn state_changes_next_task_entry() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "str x20, [x22, #24]",
        "str x22, [x22, #16]",
        "mov x0, x20",
        "bl {result_size}",
        "bl {task_alloc}",
        "mov x1, x0",
        "str x1, [x22, #40]",
        "ldr x0, [x22, #24]",
        "bl {set_task_result}",
        swift_async_function_pointer!("{run}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {task_switch}",
        result_size = sym cidre_dk_state_changes_next_result_size,
        set_task_result = sym cidre_dk_state_changes_next_set_task_result,
        task_alloc = sym swift_task_alloc,
        task_switch = sym swift_task_switch,
        run = sym state_changes_next_task_run,
    );
}

/// Makes the async iterator on the Rust side, then starts iterating.
#[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
#[unsafe(naked)]
unsafe extern "C" fn state_changes_next_task_run() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {prepare}",
        "ldr x0, [x22, #24]",
        "bl {iter_ptr}",
        "str x0, [x22, #32]",
        "str x22, [x22, #16]",
        state_changes_next_call!(),
        prepare = sym cidre_dk_state_changes_next_prepare,
        iter_ptr = sym cidre_dk_state_changes_next_iter_ptr,
        next_async = sym STATE_CHANGES_ITERATOR_NEXT_ASYNC_FN,
        next = sym state_changes_iterator_next,
        task_alloc = sym swift_task_alloc,
        resume = sym state_changes_next_task_resume,
    );
}

/// Resumed once per element. Hops to a plain frame before calling into Rust.
#[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
#[unsafe(naked)]
unsafe extern "C" fn state_changes_next_task_resume() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        swift_async_load_parent!(),
        "str x9, [sp]",
        "str x9, [x9, #16]",
        "ldr x0, [x9, #48]",
        "bl {task_dealloc}",
        "ldr x22, [sp]",
        swift_async_function_pointer!("{complete}"),
        "mov x1, #0",
        "mov x2, #0",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "b {task_switch}",
        task_dealloc = sym swift_task_dealloc,
        task_switch = sym swift_task_switch,
        complete = sym state_changes_next_task_complete,
    );
}

/// Delivers one state change to Rust, then either asks for the next one or
/// returns to whoever created the task.
#[cfg(all(target_vendor = "apple", target_arch = "aarch64"))]
#[unsafe(naked)]
unsafe extern "C" fn state_changes_next_task_complete() {
    core::arch::naked_asm!(
        swift_async_prologue!(frame: "32", fp: "16", ctx: "8"),
        "ldr x0, [x22, #24]",
        "bl {process}",
        "ldr x22, [sp, #8]",
        "cbnz x0, 0f",
        "ldr x9, [x22, #16]",
        swift_async_load_resume!(),
        "mov x22, x9",
        swift_async_epilogue!(frame: "32", fp: "16"),
        "br x16",
        "0:",
        "str x22, [x22, #16]",
        state_changes_next_call!(),
        process = sym cidre_dk_state_changes_next_process,
        next_async = sym STATE_CHANGES_ITERATOR_NEXT_ASYNC_FN,
        next = sym state_changes_iterator_next,
        task_alloc = sym swift_task_alloc,
        resume = sym state_changes_next_task_resume,
    );
}

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaF"]
    fn state_changes_iterator_next();

    #[link_name = "$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaFTu"]
    static STATE_CHANGES_ITERATOR_NEXT_ASYNC_FN: u8;

    #[link_name = "$s7DockKit0A9AccessoryC12StateChangesV8IteratorVMa"]
    fn dock_accessory_state_changes_iterator_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC12StateChangesV17makeAsyncIteratorAE0H0VyF"]
    fn dock_accessory_state_changes_make_async_iterator();

    #[link_name = "$s7DockKit0A9AccessoryC11StateChangeVMa"]
    fn dock_accessory_state_change_metadata();

    #[link_name = "$s7DockKit0A9AccessoryC11StateChangeV5stateAC0D0Ovg"]
    fn dock_accessory_state_change_state();

    #[link_name = "$s7DockKit0A9AccessoryC11StateChangeV21trackingButtonEnabledSbvg"]
    fn dock_accessory_state_change_tracking_button_enabled();

}

impl StateChanges {
    pub(crate) unsafe fn alloc(metadata: *const abi::TypeMetadata) -> Self {
        let value_layout = unsafe { abi::value_layout(metadata) };
        let layout =
            alloc::Layout::from_size_align(value_layout.stride, value_layout.align).unwrap();
        let ptr = unsafe { alloc::alloc(layout) };
        let ptr = std::ptr::NonNull::new(ptr).expect("swift value allocation failed");
        Self {
            ptr,
            layout,
            metadata,
        }
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut () {
        self.ptr.as_ptr().cast()
    }

    pub(crate) unsafe fn dealloc_uninit(self) {
        let ptr = self.ptr;
        let layout = self.layout;
        std::mem::forget(self);
        unsafe { alloc::dealloc(ptr.as_ptr(), layout) };
    }

    #[inline]
    pub fn make_async_iter(&self) -> StateChangesIter {
        unsafe {
            let metadata = abi::call_int_to_int(
                dock_accessory_state_changes_iterator_metadata as *const (),
                0,
            ) as *const abi::TypeMetadata;
            let mut iter = StateChangesIter::alloc(metadata);
            abi::call_value_to_value(
                dock_accessory_state_changes_make_async_iterator as *const (),
                self.ptr.as_ptr().cast_const().cast(),
                iter.as_mut_ptr(),
            );
            iter
        }
    }

    #[inline]
    pub fn next<F>(self, callback: F)
    where
        F: FnMut(Option<StateChange>) + Send + 'static,
    {
        StateChangeNextTask::start(self, callback);
    }
}

impl Drop for StateChanges {
    fn drop(&mut self) {
        unsafe {
            abi::destroy_value(self.as_mut_ptr(), self.metadata);
            alloc::dealloc(self.ptr.as_ptr(), self.layout);
        }
    }
}

impl StateChangesIter {
    pub(crate) unsafe fn alloc(metadata: *const abi::TypeMetadata) -> Self {
        let value_layout = unsafe { abi::value_layout(metadata) };
        let layout =
            alloc::Layout::from_size_align(value_layout.stride, value_layout.align).unwrap();
        let ptr = unsafe { alloc::alloc(layout) };
        let ptr = NonNull::new(ptr).expect("swift value allocation failed");
        Self {
            ptr,
            layout,
            metadata,
        }
    }

    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> *mut () {
        self.ptr.as_ptr().cast()
    }
}

impl Drop for StateChangesIter {
    fn drop(&mut self) {
        unsafe {
            abi::destroy_value(self.as_mut_ptr(), self.metadata);
            alloc::dealloc(self.ptr.as_ptr(), self.layout);
        }
    }
}

impl StateChangeNextTask {
    fn start<F>(state_changes: StateChanges, callback: F)
    where
        F: FnMut(Option<StateChange>) + Send + 'static,
    {
        unsafe {
            let payload_metadata =
                abi::call_int_to_int(dock_accessory_state_change_metadata as *const (), 0)
                    as *const abi::TypeMetadata;
            let result_metadata =
                abi::type_by_mangled_name("7DockKit0A9AccessoryC11StateChangeVSg");
            let state_metadata = abi::type_by_mangled_name("7DockKit0A9AccessoryC5StateO");

            let payload_layout = layout_for(payload_metadata);
            let result_layout = layout_for(result_metadata);
            let state_layout = layout_for(state_metadata);
            let task = Box::new(Self {
                state_changes: Some(state_changes),
                iter: None,
                result: alloc_value(result_layout),
                result_layout,
                result_in_task: false,
                result_metadata,
                payload: alloc_value(payload_layout),
                payload_layout,
                payload_metadata,
                state: alloc_value(state_layout),
                state_layout,
                state_metadata,
                callback: Box::new(callback),
            });
            let context: *mut () = Box::into_raw(task).cast();
            let (_task, _) = abi::task_create(
                abi::ENQUEUED_DISCARDING_TASK_FLAGS,
                core::ptr::null(),
                (&raw const cidre_dk_state_changes_next_task_descriptor).cast(),
                context,
            );
        }
    }
}

impl Drop for StateChangeNextTask {
    fn drop(&mut self) {
        unsafe {
            if !self.result_in_task {
                alloc::dealloc(self.result.as_ptr(), self.result_layout);
            }
            alloc::dealloc(self.payload.as_ptr(), self.payload_layout);
            alloc::dealloc(self.state.as_ptr(), self.state_layout);
        }
    }
}

#[inline]
unsafe fn layout_for(metadata: *const abi::TypeMetadata) -> alloc::Layout {
    let value_layout = unsafe { abi::value_layout(metadata) };
    alloc::Layout::from_size_align(value_layout.stride, value_layout.align).unwrap()
}

#[inline]
unsafe fn alloc_value(layout: alloc::Layout) -> NonNull<u8> {
    NonNull::new(unsafe { alloc::alloc(layout) }).expect("swift value allocation failed")
}

extern "C" fn cidre_dk_state_changes_next_result_size(task: *mut StateChangeNextTask) -> usize {
    unsafe { (*task).result_layout.size() }
}

extern "C" fn cidre_dk_state_changes_next_set_task_result(
    task: *mut StateChangeNextTask,
    result: *mut u8,
) {
    unsafe {
        let task = &mut *task;
        alloc::dealloc(task.result.as_ptr(), task.result_layout);
        task.result = NonNull::new(result).expect("swift task result allocation failed");
        task.result_in_task = true;
    }
}

extern "C" fn cidre_dk_state_changes_next_prepare(task: *mut StateChangeNextTask) {
    unsafe {
        let task = &mut *task;
        if task.iter.is_some() {
            return;
        }
        let state_changes = task
            .state_changes
            .take()
            .expect("state changes sequence already prepared");
        task.iter = Some(state_changes.make_async_iter());
    }
}

extern "C" fn cidre_dk_state_changes_next_iter_ptr(task: *mut StateChangeNextTask) -> *mut () {
    unsafe {
        let task = &mut *task;
        let iter = task.iter.as_mut().expect("state changes iterator missing");
        iter.as_mut_ptr().cast()
    }
}

extern "C" fn cidre_dk_state_changes_next_process(task: *mut StateChangeNextTask) -> bool {
    match catch_unwind(AssertUnwindSafe(|| unsafe {
        let task = &mut *task;
        let tag =
            abi::get_enum_tag_single_payload(task.result.as_ptr().cast(), 1, task.result_metadata);
        if tag == 1 {
            abi::destroy_value(task.result.as_ptr().cast(), task.result_metadata);
            (task.callback)(None);
            return false;
        }

        abi::call_value_to_value(
            dock_accessory_state_change_state as *const (),
            task.result.as_ptr().cast_const().cast(),
            task.state.as_ptr().cast(),
        );
        let state = State(*(task.state.as_ptr().cast::<u8>()));
        abi::destroy_value(task.state.as_ptr().cast(), task.state_metadata);

        let tracking_button_enabled = abi::call_value_to_bool(
            dock_accessory_state_change_tracking_button_enabled as *const (),
            task.result.as_ptr().cast_const().cast(),
        );
        (task.callback)(Some(StateChange {
            state,
            tracking_button_enabled,
        }));

        abi::destroy_value(task.result.as_ptr().cast(), task.payload_metadata);
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

macro_rules! resilient_enum {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            hash = $hash_fn:ident = $hash_link:literal,
            debug = $debug_fn:ident = $debug_link:literal,
            cases { $($case:ident => $symbol:ident = $link:literal),+ $(,)? }
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(transparent)]
        $vis struct $name(u8);

        #[link(name = "DockKit", kind = "framework")]
        unsafe extern "C" {
            $(
                #[link_name = $link]
                static $symbol: u8;
            )+

            #[link_name = $hash_link]
            fn $hash_fn();

            #[link_name = $debug_link]
            fn $debug_fn();
        }

        impl $name {
            $(
                #[inline]
                pub fn $case() -> Self {
                    unsafe { Self($symbol) }
                }
            )+

            #[inline]
            pub fn as_abi_ptr(&self) -> *const () {
                (self as *const Self).cast()
            }

            #[inline]
            pub fn hash_value(&self) -> isize {
                unsafe { abi::call_value_to_int($hash_fn as *const (), self.as_abi_ptr()) }
            }

            #[inline]
            pub fn debug_desc(&self) -> swift::String {
                unsafe {
                    swift::String::from_raw(abi::call_value_to_string(
                        $debug_fn as *const (),
                        self.as_abi_ptr(),
                    ))
                }
            }
        }
    };
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            hash = $hash_fn:ident = $hash_link:literal,
            cases { $($case:ident => $symbol:ident = $link:literal),+ $(,)? }
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[repr(transparent)]
        $vis struct $name(u8);

        #[link(name = "DockKit", kind = "framework")]
        unsafe extern "C" {
            $(
                #[link_name = $link]
                static $symbol: u8;
            )+

            #[link_name = $hash_link]
            fn $hash_fn();
        }

        impl $name {
            $(
                #[inline]
                pub fn $case() -> Self {
                    unsafe { Self($symbol) }
                }
            )+

            #[inline]
            pub fn as_abi_ptr(&self) -> *const () {
                (self as *const Self).cast()
            }

            #[inline]
            pub fn hash_value(&self) -> isize {
                unsafe { abi::call_value_to_int($hash_fn as *const (), self.as_abi_ptr()) }
            }
        }
    };
}

resilient_enum! {
    /// DockKit `DockAccessory.State`.
    #[doc(alias = "DockAccessory.State")]
    pub struct State {
        hash = dock_accessory_state_hash_value = "$s7DockKit0A9AccessoryC5StateO9hashValueSivg",
        debug = dock_accessory_state_debug_description = "$s7DockKit0A9AccessoryC5StateO16debugDescriptionSSvg",
        cases {
            undocked => DOCK_ACCESSORY_STATE_UNDOCKED = "$s7DockKit0A9AccessoryC5StateO8undockedyA2EmFWC",
            docked => DOCK_ACCESSORY_STATE_DOCKED = "$s7DockKit0A9AccessoryC5StateO6dockedyA2EmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.Category`.
    #[doc(alias = "DockAccessory.Category")]
    pub struct Category {
        hash = dock_accessory_category_hash_value = "$s7DockKit0A9AccessoryC8CategoryO9hashValueSivg",
        debug = dock_accessory_category_debug_description = "$s7DockKit0A9AccessoryC8CategoryO16debugDescriptionSSvg",
        cases {
            tracking_stand => DOCK_ACCESSORY_CATEGORY_TRACKING_STAND = "$s7DockKit0A9AccessoryC8CategoryO13trackingStandyA2EmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.CameraOrientation`.
    #[doc(alias = "DockAccessory.CameraOrientation")]
    pub struct CameraOrientation {
        hash = dock_accessory_camera_orientation_hash_value = "$s7DockKit0A9AccessoryC17CameraOrientationO9hashValueSivg",
        cases {
            unknown => DOCK_ACCESSORY_CAMERA_ORIENTATION_UNKNOWN = "$s7DockKit0A9AccessoryC17CameraOrientationO7unknownyA2EmFWC",
            portrait => DOCK_ACCESSORY_CAMERA_ORIENTATION_PORTRAIT = "$s7DockKit0A9AccessoryC17CameraOrientationO8portraityA2EmFWC",
            portrait_upside_down => DOCK_ACCESSORY_CAMERA_ORIENTATION_PORTRAIT_UPSIDE_DOWN = "$s7DockKit0A9AccessoryC17CameraOrientationO18portraitUpsideDownyA2EmFWC",
            landscape_right => DOCK_ACCESSORY_CAMERA_ORIENTATION_LANDSCAPE_RIGHT = "$s7DockKit0A9AccessoryC17CameraOrientationO14landscapeRightyA2EmFWC",
            landscape_left => DOCK_ACCESSORY_CAMERA_ORIENTATION_LANDSCAPE_LEFT = "$s7DockKit0A9AccessoryC17CameraOrientationO13landscapeLeftyA2EmFWC",
            face_up => DOCK_ACCESSORY_CAMERA_ORIENTATION_FACE_UP = "$s7DockKit0A9AccessoryC17CameraOrientationO6faceUpyA2EmFWC",
            face_down => DOCK_ACCESSORY_CAMERA_ORIENTATION_FACE_DOWN = "$s7DockKit0A9AccessoryC17CameraOrientationO8faceDownyA2EmFWC",
            corrected => DOCK_ACCESSORY_CAMERA_ORIENTATION_CORRECTED = "$s7DockKit0A9AccessoryC17CameraOrientationO9correctedyA2EmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.Observation.ObservationType`.
    #[doc(alias = "DockAccessory.Observation.ObservationType")]
    pub struct ObservationType {
        hash = dock_accessory_observation_type_hash_value = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO9hashValueSivg",
        cases {
            human_face => DOCK_ACCESSORY_OBSERVATION_TYPE_HUMAN_FACE = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO9humanFaceyA2GmFWC",
            human_body => DOCK_ACCESSORY_OBSERVATION_TYPE_HUMAN_BODY = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO9humanBodyyA2GmFWC",
            object => DOCK_ACCESSORY_OBSERVATION_TYPE_OBJECT = "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO6objectyA2GmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.BatteryChargeState`.
    #[doc(alias = "DockAccessory.BatteryChargeState")]
    pub struct BatteryChargeState {
        hash = dock_accessory_battery_charge_state_hash_value = "$s7DockKit0A9AccessoryC18BatteryChargeStateO9hashValueSivg",
        cases {
            not_charging => DOCK_ACCESSORY_BATTERY_CHARGE_STATE_NOT_CHARGING = "$s7DockKit0A9AccessoryC18BatteryChargeStateO11notChargingyA2EmFWC",
            charging => DOCK_ACCESSORY_BATTERY_CHARGE_STATE_CHARGING = "$s7DockKit0A9AccessoryC18BatteryChargeStateO8chargingyA2EmFWC",
            not_chargeable => DOCK_ACCESSORY_BATTERY_CHARGE_STATE_NOT_CHARGEABLE = "$s7DockKit0A9AccessoryC18BatteryChargeStateO13notChargeableyA2EmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.FramingMode`.
    #[doc(alias = "DockAccessory.FramingMode")]
    pub struct FramingMode {
        hash = dock_accessory_framing_mode_hash_value = "$s7DockKit0A9AccessoryC11FramingModeO9hashValueSivg",
        cases {
            automatic => DOCK_ACCESSORY_FRAMING_MODE_AUTOMATIC = "$s7DockKit0A9AccessoryC11FramingModeO9automaticyA2EmFWC",
            center => DOCK_ACCESSORY_FRAMING_MODE_CENTER = "$s7DockKit0A9AccessoryC11FramingModeO6centeryA2EmFWC",
            left => DOCK_ACCESSORY_FRAMING_MODE_LEFT = "$s7DockKit0A9AccessoryC11FramingModeO4leftyA2EmFWC",
            right => DOCK_ACCESSORY_FRAMING_MODE_RIGHT = "$s7DockKit0A9AccessoryC11FramingModeO5rightyA2EmFWC",
        }
    }
}

resilient_enum! {
    /// DockKit `DockAccessory.Animation`.
    #[doc(alias = "DockAccessory.Animation")]
    pub struct Animation {
        hash = dock_accessory_animation_hash_value = "$s7DockKit0A9AccessoryC9AnimationO9hashValueSivg",
        cases {
            wakeup => DOCK_ACCESSORY_ANIMATION_WAKEUP = "$s7DockKit0A9AccessoryC9AnimationO6wakeupyA2EmFWC",
            yes => DOCK_ACCESSORY_ANIMATION_YES = "$s7DockKit0A9AccessoryC9AnimationO3yesyA2EmFWC",
            no => DOCK_ACCESSORY_ANIMATION_NO = "$s7DockKit0A9AccessoryC9AnimationO2noyA2EmFWC",
            kapow => DOCK_ACCESSORY_ANIMATION_KAPOW = "$s7DockKit0A9AccessoryC9AnimationO5kapowyA2EmFWC",
        }
    }
}

#[link(name = "DockKit", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s7DockKit0A9AccessoryC16debugDescriptionSSvg"]
    fn dock_accessory_debug_description();

    #[link_name = "$s7DockKit0A9AccessoryC11framingModeAC07FramingE0Ovg"]
    fn dock_accessory_framing_mode();

    #[link_name = "$s7DockKit0A9AccessoryC16regionOfInterestSo6CGRectVvg"]
    fn dock_accessory_region_of_interest();
}

impl Accessory {
    #[doc(alias = "DockAccessory.debugDescription")]
    #[inline]
    pub fn debug_desc(&self) -> swift::String {
        unsafe {
            swift::String::from_raw(abi::call_object_to_string(
                dock_accessory_debug_description as *const (),
                (self as *const Self).cast(),
            ))
        }
    }

    #[doc(alias = "DockAccessory.framingMode")]
    #[inline]
    pub fn framing_mode(&self) -> FramingMode {
        let mut value = FramingMode::automatic();
        unsafe {
            abi::call_object_to_value(
                dock_accessory_framing_mode as *const (),
                (self as *const Self).cast(),
                (&mut value as *mut FramingMode).cast(),
            );
        }
        value
    }

    #[doc(alias = "DockAccessory.regionOfInterest")]
    #[inline]
    pub fn region_of_interest(&self) -> cg::Rect {
        let (x, y, width, height) = unsafe {
            abi::call_object_to_rect(
                dock_accessory_region_of_interest as *const (),
                (self as *const Self).cast(),
            )
        };
        cg::Rect {
            origin: cg::Point { x, y },
            size: cg::Size { width, height },
        }
    }
}
