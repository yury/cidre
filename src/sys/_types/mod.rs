use std::os::raw::c_uint;

#[repr(transparent)]
pub struct MachPort(c_uint);
