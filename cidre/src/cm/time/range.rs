use crate::cm;

#[doc(alias = "CMTimeRange")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub start: cm::Time,
    pub duration: cm::Time,
}

impl Range {
    #[doc(alias = "CMTIMERANGE_IS_INVALID")]
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.start.is_valid()
            && self.duration.is_valid()
            && self.duration.epoch == 0
            && self.duration.value >= 0
    }

    #[doc(alias = "CMTIMERANGE_IS_INDEFINITE")]
    #[inline]
    pub const fn is_indefinite(&self) -> bool {
        self.is_valid() && (self.start.is_indefinite() || self.duration.is_indefinite())
    }

    pub fn is_empty(&self) -> bool {
        self.is_valid() && self.duration == cm::Time::zero()
    }

    #[doc(alias = "CMTimeRangeContainsTime")]
    #[inline]
    pub fn contains_time(&self, time: &cm::Time) -> bool {
        unsafe { CMTimeRangeContainsTime(*self, *time) }
    }

    #[doc(alias = "kCMTimeRangeZero")]
    #[inline]
    pub const fn zero() -> Self {
        unsafe { kCMTimeRangeZero }
    }

    #[doc(alias = "kCMTimeRangeInvalid")]
    #[inline]
    pub const fn invalid() -> Self {
        unsafe { kCMTimeRangeInvalid }
    }
}

pub struct Mapping {
    pub source: cm::TimeRange,
    pub target: cm::TimeRange,
}

#[link(name = "CoreMedia", kind = "framework")]
unsafe extern "C-unwind" {
    static kCMTimeRangeZero: Range;
    static kCMTimeRangeInvalid: Range;

    fn CMTimeRangeContainsTime(range: Range, time: cm::Time) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::cm;

    #[test]
    fn basics() {
        let range = cm::TimeRange::zero();
        assert!(range.is_valid());
        assert!(range.is_empty());

        let range = cm::TimeRange::invalid();
        assert!(!range.is_valid());
        assert!(!range.is_empty());
    }
}
