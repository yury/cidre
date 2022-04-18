use std::os::raw::c_uint;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct MachPort(c_uint);

pub type Pid = i32;
