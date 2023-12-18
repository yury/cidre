use crate::{arc, cf, define_cf_type};
use std::ffi::{c_double, c_void};

pub type TimeInterval = c_double;
pub type AbsTime = TimeInterval;

/// The current absolute time.
///
/// Absolute time is measured in seconds relative to the absolute reference date of
/// Jan 1 2001 00:00:00 GMT. A positive value represents a date after
/// the reference date, a negative value represents a date before it.
/// For example, the absolute time -32940326 is equivalent to
/// December 16th, 1999 at 17:54:34. Repeated calls to this function do not guarantee
/// monotonically increasing results. The system time may decrease due to synchronization
/// with external time references or due to an explicit user change of the clock.
#[inline]
pub fn abs_time_current() -> AbsTime {
    unsafe { CFAbsoluteTimeGetCurrent() }
}

define_cf_type!(Date(cf::Type));

impl Date {
    #[doc(alias = "CFDateGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFDateGetTypeID() }
    }

    #[doc(alias = "CFDateCreate")]
    #[inline]
    pub fn new_at_in(at: AbsTime, allocator: Option<&cf::Allocator>) -> Option<arc::R<Self>> {
        unsafe { CFDateCreate(allocator, at) }
    }

    #[doc(alias = "CFDateCreate")]
    #[inline]
    pub fn new_at(at: AbsTime) -> Option<arc::R<Self>> {
        Self::new_at_in(at, None)
    }

    #[inline]
    pub fn current() -> Option<arc::R<Self>> {
        Self::new_at(abs_time_current())
    }

    #[doc(alias = "CFDateGetAbsoluteTime")]
    #[inline]
    pub fn abs_time(&self) -> AbsTime {
        unsafe { CFDateGetAbsoluteTime(self) }
    }

    #[doc(alias = "CFDateGetTimeIntervalSinceDate")]
    #[inline]
    pub fn time_interval_since_date(&self, other_date: &Date) -> TimeInterval {
        unsafe { CFDateGetTimeIntervalSinceDate(self, other_date) }
    }

    #[doc(alias = "CFDateCompare")]
    #[inline]
    pub unsafe fn compare(&self, other_date: &Date, context: *mut c_void) -> cf::ComparisonResult {
        CFDateCompare(self, other_date, context)
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.compare(other, std::ptr::null_mut()) == cf::ComparisonResult::EqualTo }
    }
}

impl Eq for Date {}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let res = unsafe { self.compare(other, std::ptr::null_mut()) };
        let val: isize = res as _;
        let ival: i8 = val as _;
        Some(unsafe { std::mem::transmute(ival) })
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFAbsoluteTimeGetCurrent() -> AbsTime;
    fn CFDateGetTypeID() -> cf::TypeId;
    fn CFDateCreate(allocator: Option<&cf::Allocator>, at: AbsTime) -> Option<arc::R<Date>>;
    fn CFDateGetAbsoluteTime(the_date: &Date) -> AbsTime;
    fn CFDateGetTimeIntervalSinceDate(the_date: &Date, other_date: &Date) -> TimeInterval;
    fn CFDateCompare(date: &Date, other_date: &Date, context: *mut c_void) -> cf::ComparisonResult;
}

#[cfg(test)]
mod tests {
    use crate::cf;

    #[test]
    fn basics() {
        let _d1 = cf::Date::current().unwrap();
        let _d2 = cf::Date::current().unwrap();

        // assert_ne!(d1, d2);
        // assert!(d1 < d2);

        // let interval = d2.time_interval_since_date(&d1);
        // assert!(interval > 0f64);
    }
}
