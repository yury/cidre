use std::{
    alloc,
    panic::{AssertUnwindSafe, catch_unwind},
    ptr::NonNull,
};

use crate::{cg, swift, swift::abi};

#[repr(C)]
pub struct Accessory {
    _priv: [u8; 0],
}

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

#[cfg(all(
    target_vendor = "apple",
    target_arch = "aarch64",
    not(target_feature = "paca")
))]
core::arch::global_asm!(
    r#"
    .section __TEXT,__const
    .globl _cidre_dk_state_changes_next_task_descriptor
    .p2align 3
_cidre_dk_state_changes_next_task_descriptor:
    .long _cidre_dk_state_changes_next_task_entry - _cidre_dk_state_changes_next_task_descriptor
    .long 64

    .text
    .globl _cidre_dk_state_changes_next_task_entry
    .p2align 2
_cidre_dk_state_changes_next_task_entry:
    orr x29, x29, #0x1000000000000000
    sub sp, sp, #32
    stp x29, x30, [sp, #16]
    str x22, [sp, #8]
    add x29, sp, #16
    str x20, [x22, #24]
    str x22, [x22, #16]
    mov x0, x20
    bl _cidre_dk_state_changes_next_result_size
    bl _swift_task_alloc
    mov x1, x0
    str x1, [x22, #40]
    ldr x0, [x22, #24]
    bl _cidre_dk_state_changes_next_set_task_result
    adrp x0, _cidre_dk_state_changes_next_task_run@PAGE
    add x0, x0, _cidre_dk_state_changes_next_task_run@PAGEOFF
    mov x1, #0
    mov x2, #0
    ldp x29, x30, [sp, #16]
    and x29, x29, #0xefffffffffffffff
    add sp, sp, #32
    b _swift_task_switch

    .globl _cidre_dk_state_changes_next_task_run
    .p2align 2
_cidre_dk_state_changes_next_task_run:
    orr x29, x29, #0x1000000000000000
    sub sp, sp, #32
    stp x29, x30, [sp, #16]
    str x22, [sp, #8]
    add x29, sp, #16
    ldr x0, [x22, #24]
    bl _cidre_dk_state_changes_next_prepare
    ldr x0, [x22, #24]
    bl _cidre_dk_state_changes_next_iter_ptr
    str x0, [x22, #32]
    str x22, [x22, #16]
    adrp x8, _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaFTu@GOTPAGE
    ldr x8, [x8, _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaFTu@GOTPAGEOFF]
    ldr w8, [x8, #4]
    mov x0, x8
    bl _swift_task_alloc
    mov x9, x0
    str x9, [x22, #48]
    ldr x8, [x22, #16]
    str x8, [x9]
    adrp x8, _cidre_dk_state_changes_next_task_resume@PAGE
    add x8, x8, _cidre_dk_state_changes_next_task_resume@PAGEOFF
    str x8, [x9, #8]
    ldr x0, [x22, #40]
    ldr x20, [x22, #32]
    mov x22, x9
    mov x21, #0
    ldp x29, x30, [sp, #16]
    and x29, x29, #0xefffffffffffffff
    add sp, sp, #32
    b _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaF

    .globl _cidre_dk_state_changes_next_task_resume
    .p2align 2
_cidre_dk_state_changes_next_task_resume:
    orr x29, x29, #0x1000000000000000
    sub sp, sp, #32
    stp x29, x30, [sp, #16]
    str x22, [sp, #8]
    add x29, sp, #16
    ldr x9, [x22]
    str x9, [sp]
    str x9, [x9, #16]
    ldr x0, [x9, #48]
    bl _swift_task_dealloc
    ldr x22, [sp]
    adrp x0, _cidre_dk_state_changes_next_task_complete@PAGE
    add x0, x0, _cidre_dk_state_changes_next_task_complete@PAGEOFF
    mov x1, #0
    mov x2, #0
    ldp x29, x30, [sp, #16]
    and x29, x29, #0xefffffffffffffff
    add sp, sp, #32
    b _swift_task_switch

    .globl _cidre_dk_state_changes_next_task_complete
    .p2align 2
_cidre_dk_state_changes_next_task_complete:
    orr x29, x29, #0x1000000000000000
    sub sp, sp, #32
    stp x29, x30, [sp, #16]
    str x22, [sp, #8]
    add x29, sp, #16
    ldr x0, [x22, #24]
    bl _cidre_dk_state_changes_next_process
    ldr x22, [sp, #8]
    cbnz x0, Lcidre_dk_state_changes_next_task_continue
    ldr x9, [x22, #16]
    ldr x0, [x9, #8]
    mov x22, x9
    ldp x29, x30, [sp, #16]
    and x29, x29, #0xefffffffffffffff
    add sp, sp, #32
    br x0

Lcidre_dk_state_changes_next_task_continue:
    str x22, [x22, #16]
    adrp x8, _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaFTu@GOTPAGE
    ldr x8, [x8, _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaFTu@GOTPAGEOFF]
    ldr w8, [x8, #4]
    mov x0, x8
    bl _swift_task_alloc
    mov x9, x0
    ldr x22, [sp, #8]
    str x9, [x22, #48]
    mov x8, x22
    str x8, [x9]
    adrp x8, _cidre_dk_state_changes_next_task_resume@PAGE
    add x8, x8, _cidre_dk_state_changes_next_task_resume@PAGEOFF
    str x8, [x9, #8]
    ldr x0, [x22, #40]
    ldr x20, [x22, #32]
    mov x22, x9
    mov x21, #0
    ldp x29, x30, [sp, #16]
    and x29, x29, #0xefffffffffffffff
    add sp, sp, #32
    b _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaF
    "#
);

#[cfg(all(
    target_vendor = "apple",
    target_arch = "aarch64",
    target_feature = "paca"
))]
core::arch::global_asm!(
    r#"
    .arch_extension pauth
    .section __TEXT,__const
    .globl _cidre_dk_state_changes_next_task_descriptor
    .p2align 3
_cidre_dk_state_changes_next_task_descriptor:
    .long _cidre_dk_state_changes_next_task_entry - _cidre_dk_state_changes_next_task_descriptor
    .long 64

    .text
    .globl _cidre_dk_state_changes_next_task_entry
    .p2align 2
_cidre_dk_state_changes_next_task_entry:
    pacibsp
    orr x29, x29, #0x1000000000000000
    sub sp, sp, #32
    stp x29, x30, [sp, #16]
    str x22, [sp, #8]
    add x29, sp, #16
    str x20, [x22, #24]
    str x22, [x22, #16]
    mov x0, x20
    bl _cidre_dk_state_changes_next_result_size
    bl _swift_task_alloc
    mov x1, x0
    str x1, [x22, #40]
    ldr x0, [x22, #24]
    bl _cidre_dk_state_changes_next_set_task_result
    adrp x16, _cidre_dk_state_changes_next_task_run@PAGE
    add x16, x16, _cidre_dk_state_changes_next_task_run@PAGEOFF
    paciza x16
    mov x0, x16
    mov x1, #0
    mov x2, #0
    ldp x29, x30, [sp, #16]
    and x29, x29, #0xefffffffffffffff
    add sp, sp, #32
    autibsp
    b _swift_task_switch

    .globl _cidre_dk_state_changes_next_task_run
    .p2align 2
_cidre_dk_state_changes_next_task_run:
    pacibsp
    orr x29, x29, #0x1000000000000000
    sub sp, sp, #32
    stp x29, x30, [sp, #16]
    str x22, [sp, #8]
    add x29, sp, #16
    ldr x0, [x22, #24]
    bl _cidre_dk_state_changes_next_prepare
    ldr x0, [x22, #24]
    bl _cidre_dk_state_changes_next_iter_ptr
    str x0, [x22, #32]
    str x22, [x22, #16]
    adrp x8, _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaFTu@GOTPAGE
    ldr x8, [x8, _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaFTu@GOTPAGEOFF]
    ldr w8, [x8, #4]
    mov x0, x8
    bl _swift_task_alloc
    mov x9, x0
    str x9, [x22, #48]
    ldr x8, [x22, #16]
    mov x16, x9
    movk x16, #48546, lsl #48
    pacda x8, x16
    str x8, [x9]
    add x8, x9, #8
    adrp x16, _cidre_dk_state_changes_next_task_resume@PAGE
    add x16, x16, _cidre_dk_state_changes_next_task_resume@PAGEOFF
    mov x17, x8
    movk x17, #55047, lsl #48
    pacia x16, x17
    str x16, [x9, #8]
    ldr x0, [x22, #40]
    ldr x20, [x22, #32]
    mov x22, x9
    mov x21, #0
    ldp x29, x30, [sp, #16]
    and x29, x29, #0xefffffffffffffff
    add sp, sp, #32
    autibsp
    b _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaF

    .globl _cidre_dk_state_changes_next_task_resume
    .p2align 2
_cidre_dk_state_changes_next_task_resume:
    pacibsp
    orr x29, x29, #0x1000000000000000
    sub sp, sp, #32
    stp x29, x30, [sp, #16]
    str x22, [sp, #8]
    add x29, sp, #16
    ldr x16, [x22]
    mov x17, x22
    movk x17, #48546, lsl #48
    autda x16, x17
    str x16, [sp]
    str x16, [x16, #16]
    ldr x0, [x16, #48]
    bl _swift_task_dealloc
    ldr x22, [sp]
    adrp x16, _cidre_dk_state_changes_next_task_complete@PAGE
    add x16, x16, _cidre_dk_state_changes_next_task_complete@PAGEOFF
    paciza x16
    mov x0, x16
    mov x1, #0
    mov x2, #0
    ldp x29, x30, [sp, #16]
    and x29, x29, #0xefffffffffffffff
    add sp, sp, #32
    autibsp
    b _swift_task_switch

    .globl _cidre_dk_state_changes_next_task_complete
    .p2align 2
_cidre_dk_state_changes_next_task_complete:
    pacibsp
    orr x29, x29, #0x1000000000000000
    sub sp, sp, #32
    stp x29, x30, [sp, #16]
    str x22, [sp, #8]
    add x29, sp, #16
    ldr x0, [x22, #24]
    bl _cidre_dk_state_changes_next_process
    ldr x22, [sp, #8]
    cbnz x0, Lcidre_dk_state_changes_next_task_continue
    ldr x9, [x22, #16]
    ldr x16, [x9, #8]
    add x17, x9, #8
    movk x17, #55047, lsl #48
    autia x16, x17
    mov x22, x9
    ldp x29, x30, [sp, #16]
    and x29, x29, #0xefffffffffffffff
    add sp, sp, #32
    autibsp
    br x16

Lcidre_dk_state_changes_next_task_continue:
    str x22, [x22, #16]
    adrp x8, _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaFTu@GOTPAGE
    ldr x8, [x8, _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaFTu@GOTPAGEOFF]
    ldr w8, [x8, #4]
    mov x0, x8
    bl _swift_task_alloc
    mov x9, x0
    ldr x22, [sp, #8]
    str x9, [x22, #48]
    mov x8, x22
    mov x16, x9
    movk x16, #48546, lsl #48
    pacda x8, x16
    str x8, [x9]
    add x8, x9, #8
    adrp x16, _cidre_dk_state_changes_next_task_resume@PAGE
    add x16, x16, _cidre_dk_state_changes_next_task_resume@PAGEOFF
    mov x17, x8
    movk x17, #55047, lsl #48
    pacia x16, x17
    str x16, [x9, #8]
    ldr x0, [x22, #40]
    ldr x20, [x22, #32]
    mov x22, x9
    mov x21, #0
    ldp x29, x30, [sp, #16]
    and x29, x29, #0xefffffffffffffff
    add sp, sp, #32
    autibsp
    b _$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaF
    "#
);

unsafe extern "C" {
    static cidre_dk_state_changes_next_task_descriptor: u8;

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
            let flags = 0x15 | (1 << 12) | (1 << 14) | (1 << 15);
            let (_task, _) = abi::task_create(
                flags,
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

#[unsafe(no_mangle)]
extern "C" fn cidre_dk_state_changes_next_result_size(task: *mut StateChangeNextTask) -> usize {
    unsafe { (*task).result_layout.size() }
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
extern "C" fn cidre_dk_state_changes_next_result_ptr(task: *mut StateChangeNextTask) -> *mut () {
    unsafe {
        let task = &mut *task;
        task.result.as_ptr().cast()
    }
}

#[unsafe(no_mangle)]
extern "C" fn cidre_dk_state_changes_next_iter_ptr(task: *mut StateChangeNextTask) -> *mut () {
    unsafe {
        let task = &mut *task;
        let iter = task.iter.as_mut().expect("state changes iterator missing");
        iter.as_mut_ptr().cast()
    }
}

#[unsafe(no_mangle)]
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
            pub fn hash_value(&self) -> swift::Int {
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
            pub fn hash_value(&self) -> swift::Int {
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

impl swift::Object<Accessory> {
    #[doc(alias = "DockAccessory.debugDescription")]
    #[inline]
    pub fn debug_desc(&self) -> swift::String {
        unsafe {
            swift::String::from_raw(abi::call_object_to_string(
                dock_accessory_debug_description as *const (),
                self.as_raw().cast_const().cast(),
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
                self.as_raw().cast_const().cast(),
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
                self.as_raw().cast_const().cast(),
            )
        };
        cg::Rect {
            origin: cg::Point { x, y },
            size: cg::Size { width, height },
        }
    }
}
