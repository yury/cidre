pub mod port;
use std::os::raw::c_int;

pub use port::MachPort;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct KernReturn(c_int);
