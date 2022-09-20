use crate::{cf, define_cf_type};

define_cf_type!(MachPort(cf::Type));

impl MachPort {
    pub fn invalidate(&self) {
        unsafe { CFMachPortInvalidate(self) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { CFMachPortIsValid(self) }
    }

    pub fn create_run_loop_source(
        &self,
        allocator: Option<&cf::Allocator>,
        index: cf::Index,
    ) -> Option<cf::Retained<cf::RunLoopSource>> {
        unsafe { CFMachPortCreateRunLoopSource(allocator, self, index) }
    }

    pub fn port(&self) -> crate::mach::MachPort {
        unsafe { CFMachPortGetPort(self) }
    }
}

extern "C" {
    fn CFMachPortInvalidate(port: &MachPort);
    fn CFMachPortIsValid(port: &MachPort) -> bool;
    fn CFMachPortCreateRunLoopSource(
        allocator: Option<&cf::Allocator>,
        port: &MachPort,
        index: cf::Index,
    ) -> Option<cf::Retained<cf::RunLoopSource>>;

    fn CFMachPortGetPort(port: &MachPort) -> crate::mach::MachPort;
}
