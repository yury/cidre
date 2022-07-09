use crate::{cf, define_cf_type};
use std::{ffi::c_void, os::raw::c_double};

pub type TimeInterval = c_double;
pub type AbsoluteTime = TimeInterval;

pub fn absolute_time_get_current() -> AbsoluteTime {
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
