use std::ffi::c_long;

pub use crate::mach::Port as MachPort;

pub type Pid = i32;

#[doc(alias = "__darwin_time_t")]
pub type DarwinTime = c_long;

#[doc(alias = "timespec")]
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(C)]
pub struct TimeSpec {
    pub tv_sec: DarwinTime,
    pub tv_nsec: c_long,
}
