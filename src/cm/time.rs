use std::cmp::Ordering;

use crate::{cf::{Allocator, Retained, String}, define_options};

pub type TimeValue = i64;
pub type TimeScale = i32;
pub type TimeEpoch = i64;

define_options!(TimeFlags(u32));

impl TimeFlags {
    pub const VALID: Self = Self(1 << 0);
    pub const HAS_BEEN_ROUNDED: Self = Self(1 << 1);
    pub const POSITIVE_INFINITY: Self = Self(1 << 2);
    pub const NEGATIVE_INFINITY: Self = Self(1 << 3);
    pub const INDEFINITE: Self = Self(1 << 4);
    pub const IMPLIED_VALUE_FLAGS_MASK: Self =
        Self(Self::POSITIVE_INFINITY.0 | Self::NEGATIVE_INFINITY.0 | Self::INDEFINITE.0);
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Time {
    pub value: TimeValue,
    pub scale: TimeScale,
    pub flags: TimeFlags,
    pub epoch: TimeEpoch,
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum TimeRoundingMethod {
    RoundHalfAwayFromZero = 1,
    RoundTowardZero = 2,
    RoundAwayFromZero = 3,
    QuickTime = 4,
    RoundTowardPositiveInfinity = 5,
    RoundTowardNegativeInfinity = 6,
}

impl Default for TimeRoundingMethod {
    fn default() -> Self {
        Self::RoundAwayFromZero
    }
}

impl Time {
    /// Returns the absolute value of a Time.
    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_seconds(-5.0, 10);
    /// let t2 = t1.absolute_value();
    /// assert_eq!(t2.scale, 10);
    /// assert_eq!(t2.get_seconds(), 5.0);
    /// ```
    #[inline]
    pub fn absolute_value(&self) -> Time {
        unsafe { CMTimeAbsoluteValue(*self) }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_seconds(100.0, 10);
    /// let t2 = cm::Time::with_seconds(200.0, 10);
    /// let t3 = t1.add(&t2);
    /// assert!(t3.is_valid());
    /// assert_eq!(t3.scale, 10);
    /// assert_eq!(t3.get_seconds(), 300.0);
    /// ```
    #[inline]
    pub fn add(&self, rhs: &Time) -> Time {
        unsafe { CMTimeAdd(*self, *rhs) }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let time = cm::Time::default().convert_scale(100, cm::TimeRoundingMethod::default());
    /// assert!(time.is_valid());
    /// assert_eq!(time.scale, 100);
    /// ```
    #[inline]
    pub fn convert_scale(
        &self,
        new_time_scale: TimeScale,
        rounding_method: TimeRoundingMethod,
    ) -> Time {
        unsafe { CMTimeConvertScale(*self, new_time_scale, rounding_method) }
    }

    #[inline]
    pub fn copy_description<'a>(
        &self,
        allocator: Option<&Allocator>,
    ) -> Option<Retained<'a, String>> {
        unsafe { CMTimeCopyDescription(allocator, *self) }
    }

    /// Converts a Time to seconds.
    #[inline]
    pub fn get_seconds(&self) -> f64 {
        unsafe { CMTimeGetSeconds(*self) }
    }

    #[inline]
    pub fn indefinit() -> Time {
        unsafe { kCMTimeIndefinite }
    }

    #[inline]
    pub fn invalid() -> Time {
        unsafe { kCMTimeInvalid }
    }

    #[inline]
    pub const fn is_indefinite(&self) -> bool {
        self.is_valid() && (self.flags.0 & TimeFlags::INDEFINITE.0) != 0
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let time = cm::Time::invalid();
    /// assert!(!time.is_valid());
    /// assert!(time.is_invalid());
    /// ```
    #[inline]
    pub const fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    /// ```
    /// use cidre::cm;
    ///
    /// assert!(cm::Time::negative_infinity().is_negative_infinity())
    /// ```
    #[inline]
    pub const fn is_negative_infinity(&self) -> bool {
        self.is_valid() && (self.flags.0 & TimeFlags::NEGATIVE_INFINITY.0) != 0
    }

    /// ```
    /// use cidre::cm;
    ///
    /// assert!(cm::Time::positive_infinity().is_positive_infinity())
    /// ```
    #[inline]
    pub const fn is_positive_infinity(&self) -> bool {
        self.is_valid() && (self.flags.0 & TimeFlags::POSITIVE_INFINITY.0) != 0
    }

    /// Returns Time from a f64 number of seconds, and a preferred timescale.
    ///
    /// ```
    /// use cidre::cm;
    ///
    /// let time = cm::Time::with_seconds(100.0, 40_000);
    /// assert!(time.is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.flags.0 & TimeFlags::VALID.0 != 0
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_seconds(5.0, 10);
    /// let t2 = t1.multiply(2);
    /// assert!(t2.is_valid());
    /// assert_eq!(t2.scale, 10);
    /// assert_eq!(t2.get_seconds(), 10.0);
    /// ```
    #[inline]
    pub fn multiply(&self, multiplier: i32) -> Time {
        unsafe { CMTimeMultiply(*self, multiplier) }
    }

    #[inline]
    pub fn multiply_by_f64(&self, multiplier: f64) -> Time {
        unsafe { CMTimeMultiplyByFloat64(*self, multiplier) }
    }

    #[inline]
    pub fn negative_infinity() -> Time {
        unsafe { kCMTimeNegativeInfinity }
    }

    /// Returns valid Time with value and timescale. Epoch is implied to be 0.
    ///
    /// ```
    /// use cidre::cm;
    ///
    /// let time = cm::Time::new(100, 10);
    /// assert!(time.is_valid());
    /// assert_eq!(time.epoch, 0);
    /// ```
    #[inline]
    pub fn new(value: TimeValue, timescale: i32) -> Time {
        unsafe { CMTimeMake(value, timescale) }
    }

    #[inline]
    pub fn positive_infinity() -> Time {
        unsafe { kCMTimePositiveInfinity }
    }

    #[inline]
    pub fn show(&self) {
        unsafe { CMTimeShow(*self) }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_seconds(100.0, 10);
    /// let t2 = cm::Time::with_seconds(100.0, 10);
    /// let t3 = t1.subtract(&t2);
    /// assert!(t3.is_valid());
    /// assert_eq!(t3.scale, 10);
    /// assert_eq!(t3.get_seconds(), 0.0);
    /// ```
    #[inline]
    pub fn subtract(&self, rhs: &Time) -> Time {
        unsafe { CMTimeSubtract(*self, *rhs) }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let time = cm::Time::with_epoch(100, 10, 5);
    /// assert!(time.is_valid());
    /// assert_eq!(time.epoch, 5);
    /// ```
    #[inline]
    pub fn with_epoch(value: TimeValue, timescale: i32, epoch: TimeEpoch) -> Time {
        unsafe { CMTimeMakeWithEpoch(value, timescale, epoch) }
    }

    /// Returns Time from a f64 number of seconds, and a preferred timescale.
    ///
    /// ```
    /// use cidre::cm;
    ///
    /// let time = cm::Time::with_seconds(100.0, 10);
    /// assert!(time.is_valid());
    /// assert_eq!(time.scale, 10);
    /// assert_eq!(time.get_seconds(), 100.0);
    /// ```
    #[inline]
    pub fn with_seconds(seconds: f64, preferred_timescale: TimeScale) -> Time {
        unsafe { CMTimeMakeWithSeconds(seconds, preferred_timescale) }
    }

    #[inline]
    pub fn zero() -> Time {
        unsafe { kCMTimeZero }
    }
}

impl PartialEq for Time {
    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_seconds(5.0, 10);
    /// let t2 = t1.multiply(2);
    /// let t3 = cm::Time::with_seconds(5.0, 100);
    /// assert!(t1 != t2);
    /// assert!(t1 == t1);
    /// assert!(t1 == t3);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        unsafe { CMTimeCompare(*self, *other) == 0 }
    }
}

impl Eq for Time {}

impl PartialOrd for Time {
    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_seconds(5.0, 10);
    /// let t2 = cm::Time::with_seconds(5.5, 10);
    /// assert!(t1 < t2);
    /// assert!(cm::Time::negative_infinity() < cm::Time::zero());
    /// assert!(cm::Time::negative_infinity() < cm::Time::positive_infinity());
    /// assert!(cm::Time::zero() < cm::Time::positive_infinity());
    /// ```
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let res = unsafe { CMTimeCompare(*self, *other) };
        match res {
            -1 => Some(Ordering::Less),
            0 => Some(Ordering::Equal),
            1 => Some(Ordering::Greater),
            _ => None,
        }
    }
}

impl Default for Time {
    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::default();
    /// assert_eq!(t1, cm::Time::zero());
    fn default() -> Self {
        Self::zero()
    }
}

// pub type CMSampleBufferRef = *const c_void;
// pub type CMFormatDescriptionRef = *const c_void;

extern "C" {
    static kCMTimeInvalid: Time;
    static kCMTimeIndefinite: Time;
    static kCMTimePositiveInfinity: Time;
    static kCMTimeNegativeInfinity: Time;
    static kCMTimeZero: Time;

    fn CMTimeMake(value: TimeValue, timescale: TimeScale) -> Time;
    fn CMTimeMakeWithEpoch(value: TimeValue, timescale: TimeScale, epoch: TimeEpoch) -> Time;
    fn CMTimeMakeWithSeconds(seconds: f64, preferred_timescale: TimeScale) -> Time;
    fn CMTimeGetSeconds(time: Time) -> f64;
    fn CMTimeConvertScale(
        time: Time,
        new_time_scale: TimeScale,
        rounding_method: TimeRoundingMethod,
    ) -> Time;

    fn CMTimeAdd(lhs: Time, rhs: Time) -> Time;
    fn CMTimeSubtract(lhs: Time, rhs: Time) -> Time;
    fn CMTimeMultiply(time: Time, multiplier: i32) -> Time;
    fn CMTimeMultiplyByFloat64(time: Time, multiplier: f64) -> Time;

    fn CMTimeCompare(time1: Time, time2: Time) -> i32;
    fn CMTimeAbsoluteValue(time: Time) -> Time;
    fn CMTimeShow(time: Time);

    fn CMTimeCopyDescription<'a>(
        allocator: Option<&Allocator>,
        time: Time,
    ) -> Option<Retained<'a, String>>;
}
