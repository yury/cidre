use std::ffi::c_ulonglong;

use crate::sys::_types::TimeSpec;

pub const MSEC_PER_SEC: c_ulonglong = 1000;
pub const NSEC_PER_SEC: c_ulonglong = 1000000000;
pub const NSEC_PER_MSEC: c_ulonglong = 1000000;
pub const USEC_PER_SEC: c_ulonglong = 1000000;
pub const NSEC_PER_USEC: c_ulonglong = 1000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Time(u64);

impl Time {
    #[doc(alias("DISPATCH_TIME_NOW"))]
    pub const NOW: Time = Time(0);

    #[doc(alias("DISPATCH_TIME_FOREVER"))]
    pub const FOREVER: Time = Time(!0);

    #[inline]
    pub fn when_delta_nanos(when: Time, delta: i64) -> Time {
        unsafe { dispatch_time(when, delta) }
    }

    #[inline]
    pub fn when(when: Time, delta: std::time::Duration) -> Time {
        unsafe { dispatch_time(when, delta.as_nanos() as _) }
    }

    #[inline]
    pub fn with_delta(delta: std::time::Duration) -> Time {
        unsafe { dispatch_time(Time::NOW, delta.as_nanos() as _) }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct WallTime(pub Time);

impl WallTime {
    pub const NOW: Self = Self(Time(!1));

    #[doc(alias("DISPATCH_TIME_FOREVER"))]
    pub const FOREVER: Self = Self(Time::FOREVER);

    // pub fn now() -> Self {
    //     unsafe { dispatch_walltime(std::ptr::null(), 0) }
    // }

    #[inline]
    pub fn with_delta_nanos(delta: i64) -> Self {
        unsafe { dispatch_walltime(std::ptr::null(), delta) }
    }

    #[inline]
    pub fn with_delta(delta: std::time::Duration) -> WallTime {
        unsafe { dispatch_walltime(std::ptr::null(), delta.as_nanos() as _) }
    }

    #[inline]
    pub fn with_timespec_delta_nanos(when: *const TimeSpec, delta: i64) -> WallTime {
        unsafe { dispatch_walltime(when, delta) }
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_time(when: Time, delta: i64) -> Time;
    fn dispatch_walltime(when: *const TimeSpec, delta: i64) -> WallTime;
}

#[cfg(test)]
mod tests {
    //use crate::dispatch;
    #[test]
    fn basics() {}
}
