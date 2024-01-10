use std::{
    ffi::{c_ulong, c_void},
    mem::transmute,
    time::Duration,
};

use crate::{arc, define_obj_type, define_opts, dispatch, mach};

define_obj_type!(pub Source(dispatch::Object));
define_obj_type!(pub TimerSource(Source));

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

define_opts!(pub MachSendFlags(c_ulong));

impl MachSendFlags {
    pub const NONE: Self = Self(0);
    pub const SEND_DEAD: Self = Self(0x1);
}

define_opts!(pub MachRecvFlags(c_ulong));

define_opts!(pub MemoryPressureFlags(c_ulong));

impl MemoryPressureFlags {
    pub const NORMAL: Self = Self(0x01);
    pub const WARN: Self = Self(0x02);
    pub const CRITICAL: Self = Self(0x04);
}

define_opts!(pub ProcFlags(c_ulong));

impl ProcFlags {
    pub const EXIT: Self = Self(0x80000000);
    pub const FORK: Self = Self(0x40000000);
    pub const EXEC: Self = Self(0x20000000);
    pub const SIGNAL: Self = Self(0x08000000);
}

define_opts!(pub VNodeFlags(c_ulong));

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

define_opts!(pub TimerFlags(c_ulong));

impl TimerFlags {
    pub const STRICT: Self = Self(0x1);
}

impl Source {
    /// # Safity
    /// use typed factory function instead
    #[inline]
    pub unsafe fn create(
        type_: &Type,
        handle: std::ffi::c_ulong,
        mask: std::ffi::c_ulong,
        queue: Option<&dispatch::Queue>,
    ) -> Option<arc::R<Source>> {
        dispatch_source_create(type_, handle, mask, queue)
    }

    #[inline]
    pub fn new_mach_send(
        port: mach::Port,
        flags: MachSendFlags,
        queue: Option<&dispatch::Queue>,
    ) -> Option<arc::R<Source>> {
        unsafe { Self::create(Type::mach_send(), port.0 as _, flags.0 as _, queue) }
    }
    #[inline]
    pub fn new_mach_recv(
        port: mach::Port,
        flags: MachRecvFlags,
        queue: Option<&dispatch::Queue>,
    ) -> Option<arc::R<Source>> {
        unsafe { Self::create(Type::mach_recv(), port.0 as _, flags.0 as _, queue) }
    }

    #[inline]
    pub fn new_memory_pressure(
        flags: MemoryPressureFlags,
        queue: Option<&dispatch::Queue>,
    ) -> Option<arc::R<Source>> {
        unsafe { Self::create(Type::memory_pressure(), 0, flags.0 as _, queue) }
    }

    #[inline]
    pub fn new_proc(
        pid: crate::sys::Pid,
        flags: ProcFlags,
        queue: Option<&dispatch::Queue>,
    ) -> Option<arc::R<Source>> {
        unsafe { Self::create(Type::proc(), pid as _, flags.0 as _, queue) }
    }

    #[inline]
    pub fn new_timer(
        flags: TimerFlags,
        queue: Option<&dispatch::Queue>,
    ) -> Option<arc::R<TimerSource>> {
        unsafe { transmute(Self::create(Type::timer(), 0, flags.0 as _, queue)) }
    }

    #[inline]
    pub fn new_read(fd: i32, queue: Option<&dispatch::Queue>) -> Option<arc::R<Source>> {
        unsafe { Self::create(Type::read(), fd as _, 0, queue) }
    }

    #[inline]
    pub fn new_write(fd: i32, queue: Option<&dispatch::Queue>) -> Option<arc::R<Source>> {
        unsafe { Self::create(Type::write(), fd as _, 0, queue) }
    }

    #[inline]
    pub fn cancel(&mut self) {
        unsafe { dispatch_source_cancel(self) }
    }

    #[inline]
    pub fn handle(&self) -> c_ulong {
        unsafe { dispatch_source_get_handle(self) }
    }

    #[inline]
    pub fn mask(&self) -> c_ulong {
        unsafe { dispatch_source_get_mask(self) }
    }

    #[inline]
    pub fn data(&self) -> c_ulong {
        unsafe { dispatch_source_get_data(self) }
    }

    #[inline]
    pub fn merge_data(&self, value: c_ulong) -> c_ulong {
        unsafe { dispatch_source_merge_data(self, value) }
    }

    #[inline]
    pub fn set_event_handler_f<T>(&mut self, handler: Option<&dispatch::Function<T>>) {
        unsafe { dispatch_source_set_event_handler_f(self, transmute(handler)) }
    }

    #[inline]
    pub fn set_cancel_handler_f<T>(&mut self, handler: Option<&dispatch::Function<T>>) {
        unsafe { dispatch_source_set_cancel_handler_f(self, transmute(handler)) }
    }

    ///
    /// # Safety
    ///
    ///  use TimerSource::set
    #[inline]
    unsafe fn source_set_timer(&mut self, start: dispatch::Time, interval: u64, leeway: u64) {
        dispatch_source_set_timer(self, start, interval, leeway)
    }
}

impl TimerSource {
    pub fn set(&mut self, start: dispatch::Time, interval: Duration, leeway: Duration) {
        unsafe { self.source_set_timer(start, interval.as_nanos() as _, leeway.as_nanos() as _) }
    }

    pub fn fired_count(&self) -> usize {
        self.data() as _
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

    fn dispatch_source_create(
        type_: &Type,
        handle: std::ffi::c_ulong,
        mask: std::ffi::c_ulong,
        queue: Option<&dispatch::Queue>,
    ) -> Option<arc::R<Source>>;

    fn dispatch_source_cancel(source: &mut Source);
    fn dispatch_source_get_handle(source: &Source) -> c_ulong;
    fn dispatch_source_get_mask(source: &Source) -> c_ulong;
    fn dispatch_source_get_data(source: &Source) -> c_ulong;
    fn dispatch_source_merge_data(source: &Source, value: c_ulong) -> c_ulong;
    fn dispatch_source_set_event_handler_f(
        source: &mut Source,
        handler: Option<&dispatch::Function<c_void>>,
    );
    fn dispatch_source_set_cancel_handler_f(
        source: &mut Source,
        handler: Option<&dispatch::Function<c_void>>,
    );

    fn dispatch_source_set_timer(
        source: &mut Source,
        start: dispatch::Time,
        interval: u64,
        leeway: u64,
    );

}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use crate::dispatch;

    #[test]
    fn timer() {
        let mut timer = dispatch::Source::new_timer(Default::default(), None)
            .unwrap()
            .retained();

        timer.set(
            dispatch::Time::NOW,
            Duration::from_nanos(100),
            Duration::from_secs(0),
        );
        assert_eq!(timer.fired_count(), 0);

        timer.activate();

        sleep(Duration::from_millis(500));

        let times = timer.fired_count();
        println!("timer fired {}", times);
        assert!(timer.fired_count() > 30);
    }
}
