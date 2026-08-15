#[doc(alias = "AudioGetCurrentHostTime")]
#[inline]
pub fn current_host_time() -> u64 {
    unsafe { AudioGetCurrentHostTime() }
}

/// A f64 containing the number of ticks per second in the host time base.
#[doc(alias = "AudioGetHostClockFrequency")]
#[inline]
pub fn host_clock_frequency() -> f64 {
    unsafe { AudioGetHostClockFrequency() }
}

/// Gets the smallest number of ticks that two succeeding values will ever differ by.
///
/// A u32 containing the smallest number of ticks that two succeeding values will ever differ.
#[doc(alias = "AudioGetHostClockMinimumTimeDelta")]
#[inline]
pub fn host_clock_min_time_delta() -> u32 {
    unsafe { AudioGetHostClockMinimumTimeDelta() }
}

#[doc(alias = "AudioConvertHostTimeToNanos")]
#[inline]
pub fn host_time_to_nanos(host_time: u64) -> u64 {
    unsafe { AudioConvertHostTimeToNanos(host_time) }
}

#[doc(alias = "AudioConvertHostTimeToNanos")]
#[inline]
pub fn nanos_to_host_time(nanos: u64) -> u64 {
    unsafe { AudioConvertNanosToHostTime(nanos) }
}

unsafe extern "C" {
    fn AudioGetCurrentHostTime() -> u64;
    fn AudioGetHostClockFrequency() -> f64;
    fn AudioGetHostClockMinimumTimeDelta() -> u32;
    fn AudioConvertHostTimeToNanos(in_host_time: u64) -> u64;
    fn AudioConvertNanosToHostTime(in_nanos: u64) -> u64;
}

#[cfg(test)]
mod tests {
    use crate::core_audio as ca;

    #[test]
    fn basics() {
        let h0 = ca::current_host_time();
        let n0 = ca::host_time_to_nanos(h0);
        assert_ne!(h0, n0);
        println!("{h0} {n0}");

        let h1 = ca::nanos_to_host_time(n0);
        assert_eq!(h0, h1);
    }
}
