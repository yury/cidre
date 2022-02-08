use crate::{cf, define_cf_type};
use std::{os::raw::c_double, ffi::c_void};

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
    pub fn create<'a>(allocator: Option<&cf::Allocator>, at: AbsoluteTime) -> Option<cf::Retained<'a, Date>> {
      unsafe {
        CFDateCreate(allocator, at)
      }
    }

    #[inline]
    pub fn get_absolute_time(&self) -> AbsoluteTime {
      unsafe {
        CFDateGetAbsoluteTime(self)
      }
    }

    #[inline]
    pub fn get_time_interval_since_date(&self, other_date: &Date) -> TimeInterval {
      unsafe {
        CFDateGetTimeIntervalSinceDate(self, other_date)
      }
    }

    #[inline]
    pub unsafe fn compare(&self, other_date: &Date, context: *mut c_void) -> cf::ComparisonResult {
      CFDateCompare(self, other_date, context)
    }
}

extern "C" {
    fn CFAbsoluteTimeGetCurrent() -> AbsoluteTime;
    fn CFDateGetTypeID() -> cf::TypeId;

    fn CFDateCreate<'a>(allocator: Option<&cf::Allocator>, at: AbsoluteTime) -> Option<cf::Retained<'a, Date>>;
    fn CFDateGetAbsoluteTime(the_date: &Date) -> AbsoluteTime;
    fn CFDateGetTimeIntervalSinceDate(the_date: &Date, other_date: &Date) -> TimeInterval;

    fn CFDateCompare(the_date: &Date, other_date: &Date, context: *mut c_void) -> cf::ComparisonResult ;
}
