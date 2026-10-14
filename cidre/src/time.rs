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

/// A time broken down into its calendar fields, as `localtime_r` and `gmtime_r` fill it.
#[doc(alias = "tm")]
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Tm {
    /// Seconds after the minute, 0...60.
    pub sec: i32,
    /// Minutes after the hour, 0...59.
    pub min: i32,
    /// Hours since midnight, 0...23.
    pub hour: i32,
    /// Day of the month, 1...31.
    pub mday: i32,
    /// Months since January, 0...11.
    pub mon: i32,
    /// Years since 1900.
    pub year: i32,
    /// Days since Sunday, 0...6.
    pub wday: i32,
    /// Days since January 1, 0...365.
    pub yday: i32,
    /// Daylight saving time: positive in effect, 0 not, negative unknown.
    pub isdst: i32,
    /// Seconds east of UTC.
    pub gmtoff: std::ffi::c_long,
    /// The time zone's abbreviation.
    pub zone: *const std::ffi::c_char,
}

impl Tm {
    /// `secs` since the epoch in the system's time zone. A change of the zone is picked up
    /// (libc watches for it), and nothing is allocated.
    #[doc(alias = "localtime_r")]
    #[inline]
    pub fn local(secs: i64) -> Option<Self> {
        let mut tm = std::mem::MaybeUninit::uninit();
        let res = unsafe { localtime_r(&secs, tm.as_mut_ptr()) };
        if res.is_null() {
            None
        } else {
            Some(unsafe { tm.assume_init() })
        }
    }

    /// `secs` since the epoch in UTC.
    #[doc(alias = "gmtime_r")]
    #[inline]
    pub fn utc(secs: i64) -> Option<Self> {
        let mut tm = std::mem::MaybeUninit::uninit();
        let res = unsafe { gmtime_r(&secs, tm.as_mut_ptr()) };
        if res.is_null() {
            None
        } else {
            Some(unsafe { tm.assume_init() })
        }
    }
}

unsafe extern "C-unwind" {
    fn clock_gettime_nsec_np(clock_id: Clock) -> u64;
    fn localtime_r(clock: *const i64, result: *mut Tm) -> *mut Tm;
    fn gmtime_r(clock: *const i64, result: *mut Tm) -> *mut Tm;
}

#[cfg(test)]
mod tests {
    use super::Tm;

    #[test]
    fn utc_breaks_down_the_epoch() {
        // 2026-10-14 08:00:00 UTC.
        let tm = Tm::utc(1_791_964_800).unwrap();
        assert_eq!((tm.year + 1900, tm.mon + 1, tm.mday), (2026, 10, 14));
        assert_eq!((tm.hour, tm.min, tm.sec, tm.gmtoff), (8, 0, 0, 0));
    }

    #[test]
    fn local_is_utc_moved_by_its_offset() {
        let secs = 1_791_964_800;
        let local = Tm::local(secs).unwrap();
        let utc = Tm::utc(secs + local.gmtoff as i64).unwrap();
        assert_eq!((local.hour, local.min, local.mday), (utc.hour, utc.min, utc.mday));
    }
}
