use std::hash::{Hash, Hasher};

use crate::{arc, cf, define_opts};

pub mod range;
pub use range::Mapping as TimeMapping;
pub use range::Range as TimeRange;

/// An integer time value.
#[doc(alias = "CMTimeValue")]
pub type TimeValue = i64;

/// Timescales must be positive.
#[doc(alias = "CMTimeScale")]
pub type TimeScale = i32;

/// Note: cm::TIME_SCALE_MAX is NOT a good choice of timescale for movie files.  
/// (Recommended timescales for movie files range from 600 to 90000.)
#[doc(alias = "kCMTimeMaxTimescale")]
pub const TIME_SCALE_MAX: TimeScale = 0x7fffffff;

/// The epoch is typically 0, but you can use a different value — for example to denote a particular iteration of a loop.
#[doc(alias = "CMTimeEpoch")]
pub type TimeEpoch = i64;

define_opts!(
    #[doc(alias = "CMTimeFlags")]
    pub TimeFlags(u32)
);

impl TimeFlags {
    pub const VALID: Self = Self(1 << 0);
    pub const HAS_BEEN_ROUNDED: Self = Self(1 << 1);
    pub const POS_INFINITY: Self = Self(1 << 2);
    pub const NEG_INFINITY: Self = Self(1 << 3);
    pub const INDEFINITE: Self = Self(1 << 4);
    pub const IMPLIED_VALUE_FLAGS_MASK: Self =
        Self(Self::POS_INFINITY.0 | Self::NEG_INFINITY.0 | Self::INDEFINITE.0);
}

#[doc(alias = "CMTime")]
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
    #[inline]
    fn default() -> Self {
        Self::RoundAwayFromZero
    }
}

impl Time {
    /// Returns the absolute value of a Time.
    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_secs(-5.0, 10);
    /// let t2 = t1.abs();
    /// assert_eq!(t2.scale, 10);
    /// assert_eq!(t2.as_secs(), 5.0);
    /// ```
    #[doc(alias = "CMTimeAbsoluteValue")]
    #[inline]
    pub fn abs(self) -> Time {
        unsafe { CMTimeAbsoluteValue(self) }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_secs(100.0, 10);
    /// let t2 = cm::Time::with_secs(200.0, 10);
    /// let t3 = t1.add(t2);
    /// assert!(t3.is_valid());
    /// assert_eq!(t3.scale, 10);
    /// assert_eq!(t3.as_secs(), 300.0);
    /// ```
    #[inline]
    pub fn add(self, rhs: Time) -> Time {
        unsafe { CMTimeAdd(self, rhs) }
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
        self,
        new_time_scale: TimeScale,
        rounding_method: TimeRoundingMethod,
    ) -> Time {
        unsafe { CMTimeConvertScale(self, new_time_scale, rounding_method) }
    }

    #[inline]
    pub fn desc_in(self, allocator: Option<&cf::Allocator>) -> Option<arc::R<cf::String>> {
        unsafe { CMTimeCopyDescription(allocator, self) }
    }

    #[inline]
    pub fn desc(self) -> Option<arc::R<cf::String>> {
        unsafe { CMTimeCopyDescription(None, self) }
    }

    /// Converts a Time to seconds.
    #[inline]
    pub fn as_secs(self) -> f64 {
        unsafe { CMTimeGetSeconds(self) }
    }

    #[inline]
    pub fn indefinit() -> Time {
        unsafe { kCMTimeIndefinite }
    }

    #[inline]
    pub fn invalid() -> Time {
        unsafe { kCMTimeInvalid }
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
    /// assert!(cm::Time::neg_infinity().is_neg_infinity())
    /// ```
    #[doc(alias = "CMTIME_IS_NEGATIVE_INFINITY")]
    #[inline]
    pub const fn is_neg_infinity(&self) -> bool {
        self.is_valid() && (self.flags.0 & TimeFlags::NEG_INFINITY.0) != 0
    }

    /// ```
    /// use cidre::cm;
    ///
    /// assert!(cm::Time::infinity().is_pos_infinity());
    /// ```
    #[doc(alias = "CMTIME_IS_POSITIVE_INFINITY")]
    #[inline]
    pub const fn is_pos_infinity(&self) -> bool {
        self.is_valid() && (self.flags.0 & TimeFlags::POS_INFINITY.0) != 0
    }

    #[doc(alias = "CMTIME_IS_INDEFINITE")]
    #[inline]
    pub const fn is_indefinite(&self) -> bool {
        self.is_valid() && (self.flags.0 & TimeFlags::INDEFINITE.0) != 0
    }

    #[doc(alias = "CMTIME_IS_NUMERIC")]
    #[inline]
    pub const fn is_numeric(&self) -> bool {
        (self.flags.0 & (TimeFlags::VALID.0 | TimeFlags::IMPLIED_VALUE_FLAGS_MASK.0))
            == TimeFlags::VALID.0
    }

    /// Returns true if the cm::Time has been rounded, false if it is completely accurate.
    #[doc(alias = "CMTIME_HAS_BEEN_ROUNDED")]
    #[inline]
    pub const fn has_been_rounded(&self) -> bool {
        self.is_numeric() && (self.flags.0 & TimeFlags::HAS_BEEN_ROUNDED.0) != 0
    }

    /// Returns Time from a f64 number of seconds, and a preferred timescale.
    ///
    /// ```
    /// use cidre::cm;
    ///
    /// let time = cm::Time::with_secs(100.0, 40_000);
    /// assert!(time.is_valid());
    /// ```
    #[inline]
    pub const fn is_valid(&self) -> bool {
        (self.flags.0 & TimeFlags::VALID.0) != 0
    }

    #[doc(alias = "is_valid")]
    #[inline]
    pub const fn is_ok(&self) -> bool {
        self.is_valid()
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_secs(5.0, 10);
    /// let t2 = t1.mul_i32(2);
    /// assert!(t2.is_valid());
    /// assert_eq!(t2.scale, 10);
    /// assert_eq!(t2.as_secs(), 10.0);
    /// ```
    #[doc(alias = "CMTimeMultiply")]
    #[inline]
    pub fn mul_i32(self, multiplier: i32) -> Time {
        unsafe { CMTimeMultiply(self, multiplier) }
    }

    #[doc(alias = "CMTimeMultiplyByFloat64")]
    #[inline]
    pub fn mul_f64(self, multiplier: f64) -> Time {
        unsafe { CMTimeMultiplyByFloat64(self, multiplier) }
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
    #[doc(alias = "CMTimeMake")]
    #[inline]
    pub fn new(value: TimeValue, timescale: i32) -> Time {
        unsafe { CMTimeMake(value, timescale) }
    }

    #[doc(alias = "kCMTimePositiveInfinity")]
    #[inline]
    pub fn infinity() -> Time {
        unsafe { kCMTimePositiveInfinity }
    }

    #[doc(alias = "kCMTimeNegativeInfinity")]
    #[inline]
    pub fn neg_infinity() -> Time {
        unsafe { kCMTimeNegativeInfinity }
    }

    #[inline]
    pub fn show(self) {
        unsafe { CMTimeShow(self) }
    }

    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_secs(100.0, 10);
    /// let t2 = cm::Time::with_secs(100.0, 10);
    /// let t3 = t1.sub(t2);
    /// assert!(t3.is_valid());
    /// assert_eq!(t3.scale, 10);
    /// assert_eq!(t3.as_secs(), 0.0);
    /// ```
    #[inline]
    pub fn sub(self, rhs: Time) -> Time {
        unsafe { CMTimeSubtract(self, rhs) }
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
    /// let time = cm::Time::with_secs(100.0, 10);
    /// assert!(time.is_valid());
    /// assert_eq!(time.scale, 10);
    /// assert_eq!(time.as_secs(), 100.0);
    /// ```
    #[inline]
    pub fn with_secs(seconds: f64, preferred_timescale: TimeScale) -> Time {
        unsafe { CMTimeMakeWithSeconds(seconds, preferred_timescale) }
    }

    #[inline]
    pub const fn zero() -> Time {
        unsafe { kCMTimeZero }
    }

    #[inline]
    pub fn max(l: Time, r: Time) -> Time {
        unsafe { CMTimeMaximum(l, r) }
    }

    #[inline]
    pub fn min(l: Time, r: Time) -> Time {
        unsafe { CMTimeMinimum(l, r) }
    }

    #[inline]
    const fn comparison_rank(&self) -> u8 {
        if self.is_invalid() {
            4
        } else if self.is_pos_infinity() {
            3
        } else if self.is_indefinite() {
            2
        } else if self.is_neg_infinity() {
            0
        } else {
            1
        }
    }
}

impl PartialEq for Time {
    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_secs(5.0, 10);
    /// let t2 = t1.mul_i32(2);
    /// let t3 = cm::Time::with_secs(5.0, 100);
    /// assert!(t1 != t2);
    /// assert!(t1 == t1);
    /// assert!(t1 == t3);
    /// ```
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for Time {}

impl Hash for Time {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let rank = self.comparison_rank();
        rank.hash(state);
        if rank != 1 {
            return;
        }

        self.epoch.hash(state);
        let valid_scale = self.scale > 0;
        valid_scale.hash(state);
        if !valid_scale {
            self.value.hash(state);
            self.scale.hash(state);
            return;
        }

        let value = self.value.unsigned_abs();
        let scale = self.scale as u64;
        let divisor = gcd(value, scale);
        let negative = self.value.is_negative();
        let normalized_scale = if value == 0 { 1 } else { scale / divisor };

        negative.hash(state);
        (value / divisor).hash(state);
        normalized_scale.hash(state);
    }
}

#[inline]
fn gcd(mut lhs: u64, mut rhs: u64) -> u64 {
    while rhs != 0 {
        (lhs, rhs) = (rhs, lhs % rhs);
    }
    lhs.max(1)
}

impl Ord for Time {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rank = self.comparison_rank();
        let other_rank = other.comparison_rank();
        let ordering = rank.cmp(&other_rank);
        if !ordering.is_eq() || rank != 1 {
            return ordering;
        }

        let ordering = self.epoch.cmp(&other.epoch);
        if !ordering.is_eq() {
            return ordering;
        }

        // CoreMedia comparison is not transitive when a numeric time has a
        // non-positive scale. Keep those safely constructible values in a
        // separate bucket so Rust's `Ord` and `Eq` contracts still hold.
        let valid_scale = self.scale > 0;
        let other_valid_scale = other.scale > 0;
        let ordering = valid_scale.cmp(&other_valid_scale);
        if !ordering.is_eq() {
            return ordering;
        }

        if valid_scale {
            unsafe { CMTimeCompare(*self, *other) }.cmp(&0)
        } else {
            (self.value, self.scale).cmp(&(other.value, other.scale))
        }
    }
}

impl PartialOrd for Time {
    /// ```
    /// use cidre::cm;
    ///
    /// let t1 = cm::Time::with_secs(5.0, 10);
    /// let t2 = cm::Time::with_secs(5.5, 10);
    /// assert!(t1 < t2);
    /// assert!(cm::Time::neg_infinity() < cm::Time::zero());
    /// assert!(cm::Time::neg_infinity() < cm::Time::infinity());
    /// assert!(cm::Time::zero() < cm::Time::infinity());
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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

#[cfg(test)]
mod tests {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    use crate::cm;

    fn hash(time: cm::Time) -> u64 {
        let mut hasher = DefaultHasher::new();
        time.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn basics() {
        let invalid = cm::Time::invalid();
        assert!(invalid.is_invalid());
        assert!(!invalid.is_numeric());

        let valid = cm::Time::default();
        assert!(valid.is_valid());
        assert!(valid.is_numeric());

        let zero = cm::Time::zero();
        assert_eq!(zero.scale, 1);
        assert_eq!(zero.value, 0);
        assert_eq!(zero.epoch, 0);
        assert_eq!(zero.flags, cm::TimeFlags::VALID);

        let zero_epoch_1 = cm::Time::with_epoch(0, 1, 1);

        assert_ne!(zero, zero_epoch_1);
        assert!(zero < zero_epoch_1);
        assert!(zero_epoch_1 == zero_epoch_1);

        assert_eq!(zero, zero.min(zero_epoch_1));
        assert_eq!(zero_epoch_1, zero_epoch_1.min(zero_epoch_1));
    }

    #[test]
    fn equal_values_have_equal_hashes() {
        let half = cm::Time::new(1, 2);
        let scaled_half = cm::Time::new(50, 100);
        assert_eq!(half, scaled_half);
        assert_eq!(hash(half), hash(scaled_half));

        let rounded_half = cm::Time {
            flags: half.flags | cm::TimeFlags::HAS_BEEN_ROUNDED,
            ..half
        };
        assert_eq!(half, rounded_half);
        assert_eq!(hash(half), hash(rounded_half));

        let mut invalid = cm::Time::invalid();
        invalid.value = i64::MAX;
        invalid.scale = i32::MAX;
        invalid.epoch = i64::MAX;
        assert_eq!(cm::Time::invalid(), invalid);
        assert_eq!(hash(cm::Time::invalid()), hash(invalid));

        let mut infinity = cm::Time::infinity();
        infinity.value = i64::MIN;
        infinity.scale = i32::MIN;
        infinity.epoch = i64::MIN;
        assert_eq!(cm::Time::infinity(), infinity);
        assert_eq!(hash(cm::Time::infinity()), hash(infinity));
    }

    #[test]
    fn hash_matches_comparison_for_all_flag_combinations() {
        let fields = [
            (0, 1, 0),
            (0, 600, 0),
            (1, 2, 0),
            (50, 100, 0),
            (-1, 2, 0),
            (i64::MIN, 2, 0),
            (i64::MIN / 2, 1, 0),
            (1, 1, 1),
            (i64::MAX, i32::MAX, i64::MAX),
            (0, 0, 0),
            (1, 0, 0),
            (-1, 0, 0),
            (1, -2, 0),
            (-1, -2, 0),
            (i64::MIN, i32::MIN, 0),
        ];
        let mut times = Vec::with_capacity(32 * fields.len());

        for flags in 0..32 {
            for &(value, scale, epoch) in &fields {
                times.push(cm::Time {
                    value,
                    scale,
                    flags: cm::TimeFlags(flags),
                    epoch,
                });
            }
        }

        for lhs in &times {
            for rhs in &times {
                if lhs == rhs {
                    assert_eq!(hash(*lhs), hash(*rhs), "{lhs:?} != {rhs:?}");
                }

                let lhs_is_supported = lhs.comparison_rank() != 1 || lhs.scale > 0;
                let rhs_is_supported = rhs.comparison_rank() != 1 || rhs.scale > 0;
                if lhs_is_supported && rhs_is_supported {
                    let core_media_ordering = unsafe { super::CMTimeCompare(*lhs, *rhs) }.cmp(&0);
                    assert_eq!(lhs.cmp(rhs), core_media_ordering);
                }
            }
        }
    }
}

unsafe extern "C-unwind" {
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
    fn CMTimeMaximum(time1: Time, time2: Time) -> Time;
    fn CMTimeMinimum(time1: Time, time2: Time) -> Time;

    fn CMTimeCopyDescription(
        allocator: Option<&cf::Allocator>,
        time: Time,
    ) -> Option<arc::R<cf::String>>;
}
