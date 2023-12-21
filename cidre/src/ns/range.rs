use crate::{
    arc,
    ns::{self, UInteger},
    objc,
};

#[doc(alias = "NSRange")]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct Range {
    pub loc: UInteger,
    pub len: UInteger,
}

impl Range {
    #[inline]
    pub fn new(loc: UInteger, len: UInteger) -> Self {
        Self { loc, len }
    }

    #[doc(alias = "NSMaxRange")]
    #[inline]
    pub fn max(&self) -> UInteger {
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
    /// assert!(ns::Range::location_in_range(1, &a));
    /// assert!(!ns::Range::location_in_range(10, &a));
    /// ```
    #[doc(alias = "NSLocationInRange")]
    #[inline]
    pub fn location_in_range(location: UInteger, range: &Self) -> bool {
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
    pub fn contains(&self, location: UInteger) -> bool {
        Range::location_in_range(location, self)
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
    #[objc::cls_msg_send(valueWithRange:)]
    pub fn with_range_ar(range: ns::Range) -> arc::Rar<Self>;

    /// Creates a new value object containing the specified Foundation range structure.
    #[objc::cls_rar_retain]
    pub fn with_range(range: ns::Range) -> arc::R<Self>;
}

#[link(name = "Foundation", kind = "framework")]
extern "C" {
    fn NSIntersectionRange(a: Range, b: Range) -> Range;
    fn NSUnionRange(a: Range, b: Range) -> Range;
}
