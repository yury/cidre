pub mod time;
pub use time::TimeBaseInfo;
pub use time::absolute_time;
pub use time::approximate_time;
pub use time::continuous_approximate_time;
pub use time::continuous_time;

pub mod port;
use std::os::raw::c_int;

pub use port::MachPort;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct KernReturn(c_int);
