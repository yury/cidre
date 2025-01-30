/// The current host time.
///
/// In macOS, the host time bases for Core Video and CoreAudio
/// are identical—both are based on the mach_absolute_time function—so
/// the values returned from either API can be used interchangeably.
#[doc(alias = "CVGetCurrentHostTime")]
#[inline]
pub fn current_host_time() -> u64 {
    unsafe { CVGetCurrentHostTime() }
}

/// Retrieve the frequency of the host time base.
///
/// On Mac OS X, the host time base for CoreVideo and CoreAudio are identical, and the values returned from either API
/// may be used interchangeably.
#[inline]
#[doc(alias = "CVGetHostClockFrequency")]
pub fn host_clock_frequency() -> f64 {
    unsafe { CVGetHostClockFrequency() }
}

/// Retrieve the smallest possible increment in the host time base.
#[doc(alias = "CVGetHostClockMinimumTimeDelta")]
#[inline]
pub fn host_clock_minimum_time_delta() -> u32 {
    unsafe { CVGetHostClockMinimumTimeDelta() }
}

#[link(name = "CoreVideo", kind = "framework")]
extern "C-unwind" {
    fn CVGetCurrentHostTime() -> u64;
    fn CVGetHostClockFrequency() -> f64;
    fn CVGetHostClockMinimumTimeDelta() -> u32;
}
