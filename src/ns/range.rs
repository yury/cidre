use super::UInteger;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct Range {
    pub location: UInteger,
    pub length: UInteger,
}

impl Range {
    #[inline]
    pub fn new(location: UInteger, length: UInteger) -> Self {
        Self { location, length }
    }

    #[inline]
    pub fn max(&self) -> UInteger {
        self.location + self.length
    }

    /// ```
    /// use cidre::ns;
    /// let a = ns::Range::new(0, 10);
    /// let b = ns::Range::new(2, 8);
    /// assert_eq!(ns::Range::intersection(a, b), b);
    /// ```
    #[inline]
    pub fn intersection(a: Self, b: Self) -> Self {
        unsafe { NSIntersectionRange(a, b) }
    }

    /// ```
    /// use cidre::{ns};
    /// let a = ns::Range::new(0, 10);
    /// assert!(ns::Range::location_in_range(1, &a));
    /// assert!(!ns::Range::location_in_range(10, &a));
    /// ```
    #[inline]
    pub fn location_in_range(location: UInteger, range: &Self) -> bool {
        !(location < range.location) && (location - range.location) < range.length
    }

    /// ```
    /// use cidre::{ns};
    /// let a = ns::Range::new(0, 10);
    /// assert!(a.contains(5));
    /// assert!(!a.contains(10));
    /// ```
    #[inline]
    pub fn contains(&self, location: UInteger) -> bool {
        Range::location_in_range(location, self)
    }
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    fn NSIntersectionRange(a: Range, b: Range) -> Range;
}
