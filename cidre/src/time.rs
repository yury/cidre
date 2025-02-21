#[doc(alias = "clockid_t")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Clock {
    #[doc(alias = "CLOCK_REALTIME")]
    RealTime = 0,

    #[doc(alias = "CLOCK_MONOTONIC")]
    Monotonic = 6,

    #[doc(alias = "CLOCK_MONOTONIC_RAW")]
    MonotonicRaw = 4,

    #[doc(alias = "CLOCK_MONOTONIC_RAW_APPROX")]
    MonotonicRawApprox = 5,
    // use this for get time
    #[doc(alias = "CLOCK_UPTIME_RAW")]
    UptimeRaw = 8,

    #[doc(alias = "CLOCK_UPTIME_RAW_APPROX")]
    UptimeRawApprox = 9,

    #[doc(alias = "CLOCK_PROCESS_CPUTIME_ID")]
    ProcessCpuTimeId = 12,

    #[doc(alias = "CLOCK_THREAD_CPUTIME_ID")]
    ThreadCpuTimeId = 16,
}

impl Clock {
    #[inline]
    pub fn time_nsec_np(&self) -> u64 {
        unsafe { clock_gettime_nsec_np(*self) }
    }
}

unsafe extern "C-unwind" {
    fn clock_gettime_nsec_np(clock_id: Clock) -> u64;
}
