use std::{ffi::c_void, mem::transmute, ops::Deref};

use crate::{arc, cf, define_obj_type, mach, msg_send, ns, objc::Delegate};

define_obj_type!(Port(ns::Id));
define_obj_type!(MachPort(Port));

impl Port {
    pub fn new() -> arc::R<Port> {
        unsafe { NSPort_port() }
    }

    pub fn invalidate(&self) {
        msg_send!("common", self, sel_invalidate)
    }

    pub fn is_valid(&self) -> bool {
        msg_send!("common", self, sel_isValid)
    }
}

impl MachPort {
    pub fn new() -> arc::R<Self> {
        unsafe { transmute(Port::new()) }
    }

    pub fn mach_port(&self) -> mach::Port {
        unsafe { rsel_machPort(self) }
    }

    pub fn schedule_in_runloop(&self, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode) {
        unsafe { wsel_scheduleInRunLoop_forMode(self, run_loop, mode) }
    }

    pub fn remove_from_runloop(&self, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode) {
        unsafe { wsel_removeFromRunLoop_forMode(self, run_loop, mode) }
    }

    pub fn set_delegate<D>(&mut self, delegate: &Delegate<D>)
    where
        D: MachPortDelegate,
    {
        let obj = delegate.obj.deref();

        msg_send!("common", self, sel_setDelegate, obj)
    }

    pub fn remove_delegate(&mut self) {
        let none: Option<&ns::Id> = None;
        msg_send!("common", self, sel_setDelegate, none)
    }
}

pub trait MachPortDelegate {
    extern "C" fn handle_mach_message(msg: *mut c_void);

    fn delegate(self) -> Delegate<Self>
    where
        Self: Sized,
    {
        let b = Box::new(self);
        let table: [*const c_void; 2] =
            [b.as_ref() as *const _ as _, Self::handle_mach_message as _];

        let ptr = table.as_ptr();

        let obj = unsafe { make_mach_port_delegate(ptr as _) };

        Delegate { delegate: b, obj }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSPort_port() -> arc::R<Port>;
    fn rsel_machPort(id: &ns::Id) -> mach::Port;

    fn wsel_scheduleInRunLoop_forMode(id: &ns::Id, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode);
    fn wsel_removeFromRunLoop_forMode(id: &ns::Id, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode);

    fn make_mach_port_delegate(vtable: *const *const c_void) -> arc::R<ns::Id>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn create() {
        let mach = ns::MachPort::new();
        let port = mach.mach_port();
        assert_ne!(port.0, 0);
        assert!(mach.is_valid());
        mach.invalidate();
        assert!(!mach.is_valid());

        // TODO: investigate why invalidate decrease refs count
        std::mem::forget(mach);
    }
}
