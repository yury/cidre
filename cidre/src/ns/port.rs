use std::{ffi::c_void, mem::transmute, ops::Deref};

use crate::{
    arc, cf, define_obj_type, mach, ns,
    objc::Delegate,
    objc::{self, msg_send, Obj},
};

define_obj_type!(Port(ns::Id));
define_obj_type!(MachPort(Port));

impl Port {
    pub fn new() -> arc::R<Port> {
        unsafe { NSPort_port() }
    }

    #[objc::msg_send2(invalidate)]
    pub fn invalidate(&self);

    #[objc::msg_send2(isValid)]
    pub fn is_valid(&self) -> bool;
}

impl MachPort {
    pub fn new() -> arc::R<Self> {
        unsafe { transmute(Port::new()) }
    }

    #[objc::msg_send2(machPort)]
    pub fn mach_port(&self) -> mach::Port;

    #[objc::msg_send2(scheduleInRunLoop:forMode:)]
    pub fn schedule_in_runloop(&self, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode);

    #[objc::msg_send2(scheduleInRunLoop:forMode:)]
    pub fn remove_from_runloop(&self, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode);

    pub fn set_delegate<D>(&mut self, delegate: &Delegate<D>)
    where
        D: MachPortDelegate,
    {
        let obj = delegate.obj.deref();

        unsafe { self.call1(msg_send::set_delegate, obj) }
    }

    pub fn remove_delegate(&mut self) {
        let none: Option<&ns::Id> = None;
        unsafe { self.call1(msg_send::set_delegate, none) }
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
