use crate::{arc, ns, objc};

#[doc(alias = "NSRange")]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct Range {
    pub loc: ns::UInteger,
    pub len: ns::UInteger,
}

impl Range {
    #[inline]
    pub const fn new(loc: ns::UInteger, len: ns::UInteger) -> Self {
        Self { loc, len }
    }

    #[doc(alias = "NSMaxRange")]
    #[inline]
    pub const fn max(&self) -> ns::UInteger {
        self.loc + self.len
    }

    /// ```
    /// use cidre::ns;
    /// let a = ns::Range::new(0, 10);
    /// let b = ns::Range::new(2, 8);
    /// assert_eq!(ns::Range::intersection(a, b), b);
    /// ```
    #[doc(alias = "NSIntersectionRange")]
    #[inline]
    pub fn intersection(a: Self, b: Self) -> Self {
        unsafe { NSIntersectionRange(a, b) }
    }

    #[doc(alias = "NSUnionRange")]
    #[inline]
    pub fn union(a: Self, b: Self) -> Self {
        unsafe { NSUnionRange(a, b) }
    }

    /// ```
    /// use cidre::{ns};
    /// let a = ns::Range::new(0, 10);
    /// assert!(ns::Range::loc_in_range(1, &a));
    /// assert!(!ns::Range::loc_in_range(10, &a));
    /// ```
    #[doc(alias = "NSLocationInRange")]
    #[inline]
    pub const fn loc_in_range(location: ns::UInteger, range: &Self) -> bool {
        location >= range.loc && (location - range.loc) < range.len
    }

    /// ```
    /// use cidre::ns;
    ///
    /// let a = ns::Range::new(0, 10);
    /// assert!(a.contains(5));
    /// assert!(!a.contains(10));
    /// ```
    #[inline]
    pub const fn contains(&self, location: ns::UInteger) -> bool {
        Range::loc_in_range(location, self)
    }
}

impl From<std::ops::Range<usize>> for Range {
    #[inline]
    fn from(value: std::ops::Range<usize>) -> Self {
        Self {
            loc: value.start,
            len: value.len(),
        }
    }
}

/// NSValueRangeExtensions
impl ns::Value {
    /// Creates a new value object containing the specified Foundation range structure.
    #[objc::msg_send(valueWithRange:)]
    pub fn with_range(range: ns::Range) -> arc::R<Self>;
}

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    fn NSIntersectionRange(a: Range, b: Range) -> Range;
    fn NSUnionRange(a: Range, b: Range) -> Range;
}
