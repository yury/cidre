#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Clock {
    RealTime = 0,
    Monotonic = 6,
    MonotonicRaw = 4,
    MonotonicRawApprox = 5,
    // use this for get time
    UptimeRaw = 8,
    UptimeRawApprox = 9,
    ProcessCpuTimeId = 12,
    ThreadCpuTimeId = 16,
}

impl Clock {
    #[inline]
    pub fn time_nsec_np(&self) -> u64 {
        unsafe { clock_gettime_nsec_np(*self) }
    }
}

extern "C" {
    fn clock_gettime_nsec_np(clock_id: Clock) -> u64;
}
