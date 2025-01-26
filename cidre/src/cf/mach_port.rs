use crate::{arc, cf, define_cf_type};

define_cf_type!(
    #[doc(alias = "CFMachPortRef")]
    MachPort(cf::Type)
);

impl MachPort {
    #[inline]
    pub fn invalidate(&self) {
        unsafe { CFMachPortInvalidate(self) }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { CFMachPortIsValid(self) }
    }

    #[inline]
    pub fn run_loop_src_in(
        &self,
        index: cf::Index,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::RunLoopSrc>> {
        unsafe { CFMachPortCreateRunLoopSource(allocator, self, index) }
    }

    #[inline]
    pub fn run_loop_src(&self, index: cf::Index) -> Option<arc::R<cf::RunLoopSrc>> {
        unsafe { CFMachPortCreateRunLoopSource(None, self, index) }
    }

    #[inline]
    pub fn port(&self) -> crate::mach::Port {
        unsafe { CFMachPortGetPort(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C-unwind" {
    fn CFMachPortInvalidate(port: &MachPort);
    fn CFMachPortIsValid(port: &MachPort) -> bool;
    fn CFMachPortCreateRunLoopSource(
        allocator: Option<&cf::Allocator>,
        port: &MachPort,
        index: cf::Index,
    ) -> Option<arc::R<cf::RunLoopSrc>>;

    fn CFMachPortGetPort(port: &MachPort) -> crate::mach::Port;
}
