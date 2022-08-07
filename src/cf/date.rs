use crate::{cf, define_cf_type};
use std::{ffi::c_void, os::raw::c_double};

pub type TimeInterval = c_double;
pub type AbsoluteTime = TimeInterval;

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
pub fn absolute_time_current() -> AbsoluteTime {
    unsafe { CFAbsoluteTimeGetCurrent() }
}

define_cf_type!(Date(cf::Type));

impl Date {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFDateGetTypeID() }
    }

    #[inline]
    pub fn create(
        allocator: Option<&cf::Allocator>,
        at: AbsoluteTime,
    ) -> Option<cf::Retained<Date>> {
        unsafe { CFDateCreate(allocator, at) }
    }

    #[inline]
    pub fn absolute_time(&self) -> AbsoluteTime {
        unsafe { CFDateGetAbsoluteTime(self) }
    }

    #[inline]
    pub fn time_interval_since_date(&self, other_date: &Date) -> TimeInterval {
        unsafe { CFDateGetTimeIntervalSinceDate(self, other_date) }
    }

    #[inline]
    pub unsafe fn compare(&self, other_date: &Date, context: *mut c_void) -> cf::ComparisonResult {
        CFDateCompare(self, other_date, context)
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFAbsoluteTimeGetCurrent() -> AbsoluteTime;
    fn CFDateGetTypeID() -> cf::TypeId;

    fn CFDateCreate(
        allocator: Option<&cf::Allocator>,
        at: AbsoluteTime,
    ) -> Option<cf::Retained<Date>>;
    fn CFDateGetAbsoluteTime(the_date: &Date) -> AbsoluteTime;
    fn CFDateGetTimeIntervalSinceDate(the_date: &Date, other_date: &Date) -> TimeInterval;

    fn CFDateCompare(
        the_date: &Date,
        other_date: &Date,
        context: *mut c_void,
    ) -> cf::ComparisonResult;
}
