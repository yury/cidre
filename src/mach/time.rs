
#[derive(Default)]
#[repr(C)]
pub struct TimeBaseInfo {
    pub numer: i32,
    pub denom: i32,
}

impl TimeBaseInfo {
    /// ```
    /// use cidre::mach;
    /// 
    /// let mut tbi = mach::TimeBaseInfo::default();
    /// assert_eq!(tbi.numer, 0);
    /// assert_eq!(tbi.denom, 0);
    /// 
    /// tbi.fill(); 
    /// 
    /// assert!(tbi.numer > 0);
    /// assert!(tbi.denom > 0);
    /// ```
    pub fn fill(&mut self) -> i32 {
        unsafe { mach_timebase_info(self) }
    }
}

#[inline]
pub fn absolute_time() -> u64 {
    unsafe { mach_absolute_time() }
}

#[inline]
pub fn approximate_time() -> u64 {
    unsafe { mach_approximate_time() }
}

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

    fn mach_timebase_info(info: &mut TimeBaseInfo) -> i32;
}
