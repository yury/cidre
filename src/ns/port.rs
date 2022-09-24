use std::mem::transmute;

use crate::{cf, define_obj_type, mach, msg_send, ns};

define_obj_type!(Port(ns::Id));
define_obj_type!(MachPort(Port));

impl Port {
    pub fn new() -> cf::Retained<Port> {
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
    pub fn new() -> cf::Retained<Self> {
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
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSPort_port() -> cf::Retained<Port>;
    fn rsel_machPort(id: &ns::Id) -> mach::Port;

    fn wsel_scheduleInRunLoop_forMode(id: &ns::Id, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode);
    fn wsel_removeFromRunLoop_forMode(id: &ns::Id, run_loop: &cf::RunLoop, mode: &cf::RunLoopMode);
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
