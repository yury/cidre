use super::KernReturn;

#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct TimeBaseInfo {
    pub numer: u32,
    pub denom: u32,
}

impl TimeBaseInfo {
    /// ```
    /// use cidre::mach;
    ///
    /// let mut tbi = mach::TimeBaseInfo::default();
    /// assert_eq!(tbi.numer, 0);
    /// assert_eq!(tbi.denom, 0);
    ///
    /// assert!(tbi.fill().is_ok());
    ///
    /// assert!(tbi.numer > 0);
    /// assert!(tbi.denom > 0);
    /// ```
    #[inline]
    pub fn fill(&mut self) -> KernReturn {
        unsafe { mach_timebase_info(self) }
    }

    #[inline]
    pub fn new() -> Self {
        let mut res = Self::default();
        let r = res.fill();
        debug_assert!(r.is_ok());
        res
    }
}

/// Returns current value of a clock that increments monotonically in tick units
/// (starting at an arbitrary point), this clock does not increment while
/// the system is asleep.
#[inline]
pub fn absolute_time() -> u64 {
    unsafe { mach_absolute_time() }
}

#[inline]
pub fn approximate_time() -> u64 {
    unsafe { mach_approximate_time() }
}

/// Returns current value of a clock that increments monotonically in tick units
/// (starting at an arbitrary point), including while the system is asleep.
#[inline]
pub fn continuous_time() -> u64 {
    unsafe { mach_continuous_time() }
}

#[inline]
pub fn continuous_approximate_time() -> u64 {
    unsafe { mach_continuous_approximate_time() }
}

extern "C" {
    fn mach_absolute_time() -> u64;
    fn mach_approximate_time() -> u64;
    fn mach_continuous_time() -> u64;
    fn mach_continuous_approximate_time() -> u64;

    fn mach_timebase_info(info: &mut TimeBaseInfo) -> KernReturn;
}
