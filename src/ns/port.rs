use std::mem::transmute;

use crate::{cf, define_obj_type, mach, ns};

define_obj_type!(Port(ns::Id));
define_obj_type!(MachPort(Port));

impl Port {
    pub fn new() -> cf::Retained<Port> {
        unsafe { NSPort_port() }
    }
}

impl MachPort {
    pub fn new() -> cf::Retained<Self> {
        unsafe { transmute(Port::new()) }
    }

    pub fn mach_port(&self) -> mach::Port {
        unsafe { rsel_machPort(self) }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSPort_port() -> cf::Retained<Port>;
    fn rsel_machPort(id: &ns::Id) -> mach::Port;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn create() {
        let mach = ns::MachPort::new();
        let port = mach.mach_port();
        assert_ne!(port.0, 0);
    }
}
