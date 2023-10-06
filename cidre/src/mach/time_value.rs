use crate::mach;

/// Time value returned by kernel
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[doc(alias = "time_value_t")]
#[repr(C)]
pub struct TimeValue {
    pub seconds: mach::Integer,
    pub microseconds: mach::Integer,
}
