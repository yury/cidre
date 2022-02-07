use std::os::raw::c_int;

pub use crate::sys::_types::MachPort;

impl MachPort {
    pub fn task_self_deallocate(self) -> c_int {
        unsafe { mach_port_deallocate(mach_task_self_, self) }
    }
}

extern "C" {
    static mach_task_self_: MachPort;

    fn mach_port_deallocate(task: MachPort, port: MachPort) -> c_int;
}
