use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Time(u64);

impl Time {
    pub const NOW: Time = Time(0);
    pub const FOREVER: Time = Time(!0);

    #[inline]
    pub fn when_delta_nanos(when: Time, delta: i64) -> Time {
        unsafe { dispatch_time(when, delta) }
    }

    #[inline]
    pub fn when(when: Time, delta: Duration) -> Time {
        unsafe { dispatch_time(when, delta.as_nanos() as _) }
    }

    #[inline]
    pub fn now(delta: Duration) -> Time {
        unsafe { dispatch_time(Time::NOW, delta.as_nanos() as _) }
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_time(when: Time, delta: i64) -> Time;
}
