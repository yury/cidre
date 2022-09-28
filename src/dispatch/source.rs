use std::{
    ffi::{c_ulong, c_void},
    ptr::NonNull,
};

use crate::{define_obj_type, define_options, dispatch};

define_obj_type!(Source(dispatch::Object));

/// The dispatch framework provides a suite of interfaces for monitoring low-
/// level system objects (file descriptors, Mach ports, signals, VFS nodes, etc.)
/// for activity and automatically submitting event handler blocks to dispatch
/// queues when such activity occurs.
/// This suite of interfaces is known as the Dispatch Source API.

/// Dispatch sources are used to automatically submit event handler blocks to
/// dispatch queues in response to external events.
impl Source {}

#[repr(transparent)]
pub struct Type(c_void);

impl Type {
    #[inline]
    pub fn data_add() -> &'static TypeDataAdd {
        unsafe { &_dispatch_source_type_data_add }
    }

    #[inline]
    pub fn data_or() -> &'static TypeDataOr {
        unsafe { &_dispatch_source_type_data_or }
    }

    #[inline]
    pub fn data_replace() -> &'static TypeDataReplace {
        unsafe { &_dispatch_source_type_data_replace }
    }

    #[inline]
    pub fn mach_send() -> &'static TypeMachSend {
        unsafe { &_dispatch_source_type_mach_send }
    }

    #[inline]
    pub fn mach_recv() -> &'static TypeMachRecv {
        unsafe { &_dispatch_source_type_mach_recv }
    }

    #[inline]
    pub fn memory_pressure() -> &'static TypeMemoryPressure {
        unsafe { &_dispatch_source_type_memorypressure }
    }

    #[inline]
    pub fn proc() -> &'static TypeProc {
        unsafe { &_dispatch_source_type_proc }
    }

    #[inline]
    pub fn read() -> &'static TypeRead {
        unsafe { &_dispatch_source_type_read }
    }

    #[inline]
    pub fn signal() -> &'static TypeSignal {
        unsafe { &_dispatch_source_type_signal }
    }

    #[inline]
    pub fn timer() -> &'static TypeTimer {
        unsafe { &_dispatch_source_type_timer }
    }

    #[inline]
    pub fn vnode() -> &'static TypeVNode {
        unsafe { &_dispatch_source_type_vnode }
    }

    #[inline]
    pub fn write() -> &'static TypeWrite {
        unsafe { &_dispatch_source_type_write }
    }
}

pub type TypeDataAdd = Type;
pub type TypeDataOr = Type;
pub type TypeDataReplace = Type;
pub type TypeMachSend = Type;
pub type TypeMachRecv = Type;
pub type TypeMemoryPressure = Type;
pub type TypeProc = Type;
pub type TypeRead = Type;
pub type TypeSignal = Type;
pub type TypeTimer = Type;
pub type TypeVNode = Type;
pub type TypeWrite = Type;

define_options!(MachSendFlags(c_ulong));

impl MachSendFlags {
    pub const NONE: Self = Self(0);
    pub const SEND_DEAD: Self = Self(0x1);
}

define_options!(MachRecvFlags(c_ulong));

define_options!(MemoryPressureFlags(c_ulong));

impl MemoryPressureFlags {
    pub const NORMAL: Self = Self(0x01);
    pub const WARN: Self = Self(0x02);
    pub const CRITICAL: Self = Self(0x04);
}

define_options!(ProcFlags(c_ulong));

impl ProcFlags {
    pub const EXIT: Self = Self(0x80000000);
    pub const FORK: Self = Self(0x40000000);
    pub const EXEC: Self = Self(0x20000000);
    pub const SIGNAL: Self = Self(0x08000000);
}

define_options!(VNodeFlags(c_ulong));

impl VNodeFlags {
    pub const DELETE: Self = Self(0x1);
    pub const WRITE: Self = Self(0x2);
    pub const EXTEND: Self = Self(0x4);
    pub const ATTRIB: Self = Self(0x8);
    pub const LINK: Self = Self(0x10);
    pub const RENAME: Self = Self(0x20);
    pub const REVOKE: Self = Self(0x40);
    pub const FUNLOCK: Self = Self(0x100);
}

define_options!(TimerFlags(c_ulong));

impl TimerFlags {
    pub const STRICT: Self = Self(0x1);
}

impl Source {
    pub fn create<'b>(
        type_: &Type,
        handle: std::ffi::c_ulong,
        mask: std::ffi::c_ulong,
        queue: Option<&dispatch::Queue>,
    ) -> Option<&'b Source> {
        unsafe { dispatch_source_create(type_, handle, mask, queue) }
    }

    #[inline]
    pub fn cancel(&mut self) {
        unsafe { dispatch_source_cancel(self) }
    }

    pub fn handle(&self) -> c_ulong {
        unsafe { dispatch_source_get_handle(self) }
    }
    pub fn mask(&self) -> c_ulong {
        unsafe { dispatch_source_get_mask(self) }
    }

    pub fn data(&self) -> c_ulong {
        unsafe { dispatch_source_get_data(self) }
    }

    pub fn merge_data(&self, value: c_ulong) -> c_ulong {
        unsafe { dispatch_source_merge_data(self, value) }
    }
}

extern "C" {
    static _dispatch_source_type_data_add: TypeDataAdd;
    static _dispatch_source_type_data_or: TypeDataOr;
    static _dispatch_source_type_data_replace: TypeDataReplace;
    static _dispatch_source_type_mach_send: TypeMachSend;
    static _dispatch_source_type_mach_recv: TypeMachRecv;
    static _dispatch_source_type_memorypressure: TypeMemoryPressure;
    static _dispatch_source_type_proc: TypeProc;
    static _dispatch_source_type_read: TypeRead;
    static _dispatch_source_type_signal: TypeSignal;
    static _dispatch_source_type_timer: TypeTimer;
    static _dispatch_source_type_vnode: TypeVNode;
    static _dispatch_source_type_write: TypeWrite;

    fn dispatch_source_create<'b>(
        type_: &Type,
        handle: std::ffi::c_ulong,
        mask: std::ffi::c_ulong,
        queue: Option<&dispatch::Queue>,
    ) -> Option<&'b Source>;

    fn dispatch_source_cancel(source: &mut Source);

    fn dispatch_source_get_handle(source: &Source) -> c_ulong;
    fn dispatch_source_get_mask(source: &Source) -> c_ulong;
    fn dispatch_source_get_data(source: &Source) -> c_ulong;
    fn dispatch_source_merge_data(source: &Source, value: c_ulong) -> c_ulong;
}

#[cfg(test)]
mod tests {
    use crate::dispatch;

    #[test]
    fn basic_check() {
        let f = dispatch::SourceType::data_add();
        let source = dispatch::Source::create(f, 0, 0, dispatch::Queue::TARGET_QUEUE_DEFAULT);

        assert!(source.is_some());
    }
}
