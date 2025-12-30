use crate::{arc, cf};

#[cfg(target_pointer_width = "32")]
pub type Float = f32;

#[cfg(target_pointer_width = "64")]
pub type Float = f64;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}

impl Point {
    #[inline]
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// ```
    /// use cidre::cg;
    ///
    /// let d = cg::Point::zero().dictionary_representaion();
    /// assert_eq!(d.len(), 2);
    /// ```
    pub fn dictionary_representaion(&self) -> arc::R<cf::Dictionary> {
        unsafe { CGPointCreateDictionaryRepresentation(*self) }
    }

    #[inline]
    pub const fn new(x: Float, y: Float) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Size {
    pub width: Float,
    pub height: Float,
}

impl Size {
    #[inline]
    pub const fn zero() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }

    /// ```
    /// use cidre::cg;
    ///
    /// let d = cg::Size::zero().dictionary_representaion();
    /// assert_eq!(d.len(), 2);
    /// ```
    pub fn dictionary_representaion(&self) -> arc::R<cf::Dictionary> {
        unsafe { CGSizeCreateDictionaryRepresentation(*self) }
    }

    #[inline]
    pub const fn new(width: Float, height: Float) -> Self {
        Self { width, height }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    #[inline]
    pub const fn zero() -> Self {
        Self {
            origin: Point::zero(),
            size: Size::zero(),
        }
    }

    #[inline]
    pub fn null() -> Self {
        unsafe { CGRectNull }
    }

    /// ```
    /// use cidre::cg;
    ///
    /// let d = cg::Rect::zero().dictionary_representaion();
    /// assert_eq!(d.len(), 4);
    /// ```
    #[inline]
    pub fn dictionary_representaion(&self) -> arc::R<cf::Dictionary> {
        unsafe { CGRectCreateDictionaryRepresentation(*self) }
    }

    #[inline]
    pub const fn new(x: Float, y: Float, width: Float, height: Float) -> Self {
        Self {
            origin: Point { x, y },
            size: Size { width, height },
        }
    }

    #[inline]
    pub const fn with_size(width: Float, height: Float) -> Self {
        Self {
            origin: Point::zero(),
            size: Size { width, height },
        }
    }

    #[doc(alias = "CGRectGetMaxX")]
    #[inline]
    pub const fn max_x(&self) -> Float {
        if self.size.width < 0.0 {
            self.origin.x
        } else {
            self.origin.x + self.size.width
        }
    }

    #[doc(alias = "CGRectGetMaxY")]
    #[inline]
    pub const fn max_y(&self) -> Float {
        if self.size.height < 0.0 {
            self.origin.y
        } else {
            self.origin.y + self.size.height
        }
    }

    #[doc(alias = "CGRectGetMinX")]
    #[inline]
    pub const fn min_x(&self) -> Float {
        if self.size.width < 0.0 {
            self.origin.x + self.size.width
        } else {
            self.origin.x
        }
    }

    #[doc(alias = "CGRectGetMinY")]
    #[inline]
    pub const fn min_y(&self) -> Float {
        if self.size.height < 0.0 {
            self.origin.y + self.size.height
        } else {
            self.origin.y
        }
    }

    #[inline]
    pub const fn x(&self) -> Float {
        self.origin.x
    }

    #[inline]
    pub const fn y(&self) -> Float {
        self.origin.y
    }

    #[inline]
    pub const fn width(&self) -> Float {
        self.size.width
    }

    #[inline]
    pub const fn height(&self) -> Float {
        self.size.height
    }

    /// Standardize rect -- i.e., convert it to an equivalent rect which has
    /// positive width and height.
    #[doc(alias = "CGRectStandardize")]
    #[must_use]
    #[inline]
    pub fn standardized(&self) -> Self {
        unsafe { CGRectStandardize(*self) }
    }

    #[doc(alias = "CGRectContainsPoint")]
    #[inline]
    pub fn contains_point(&self, point: &Point) -> bool {
        unsafe { CGRectContainsPoint(*self, *point) }
    }

    #[doc(alias = "CGRectContainsRect")]
    #[inline]
    pub fn contains_rect(&self, other: &Self) -> bool {
        unsafe { CGRectContainsRect(*self, *other) }
    }

    #[doc(alias = "CGRectUnion")]
    #[must_use]
    #[inline]
    pub fn union(&self, other: &Self) -> Rect {
        unsafe { CGRectUnion(*self, *other) }
    }

    #[doc(alias = "CGRectIntersectsRect")]
    #[inline]
    pub fn intersects_rect(&self, other: &Self) -> bool {
        unsafe { CGRectIntersectsRect(*self, *other) }
    }

    #[doc(alias = "CGRectEqualToRect")]
    #[inline]
    pub fn geometry_eq(&self, other: &Self) -> bool {
        unsafe { CGRectEqualToRect(*self, *other) }
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.size.width == 0.0 || self.size.height == 0.0
    }

    #[doc(alias = "CGRectIsInfinite")]
    #[inline]
    pub fn is_infinite(&self) -> bool {
        unsafe { CGRectIsInfinite(*self) }
    }

    #[doc(alias = "CGRectIsNull")]
    #[inline]
    pub fn is_null(&self) -> bool {
        unsafe { CGRectIsNull(*self) }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Vector {
    pub dx: Float,
    pub dy: Float,
}

unsafe extern "C" {
    static CGRectNull: Rect;

    fn CGPointCreateDictionaryRepresentation(point: Point) -> arc::R<cf::Dictionary>;
    fn CGSizeCreateDictionaryRepresentation(size: Size) -> arc::R<cf::Dictionary>;
    fn CGRectCreateDictionaryRepresentation(rect: Rect) -> arc::R<cf::Dictionary>;

    fn CGRectEqualToRect(rect1: Rect, rect2: Rect) -> bool;
    fn CGRectIntersectsRect(rect1: Rect, rect2: Rect) -> bool;
    fn CGRectUnion(rect1: Rect, rect2: Rect) -> Rect;
    fn CGRectStandardize(rect: Rect) -> Rect;
    fn CGRectContainsPoint(rect: Rect, point: Point) -> bool;
    fn CGRectContainsRect(rect1: Rect, rect2: Rect) -> bool;
    fn CGRectIsInfinite(rect: Rect) -> bool;
    fn CGRectIsNull(rect: Rect) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::cg;

    #[test]
    fn basics() {
        let r = cg::Rect::new(10.0, 15.0, -100.0, -200.0);
        assert_eq!(r.min_x(), -90.0);
        assert_eq!(r.min_y(), -185.0);
        assert_eq!(r.max_x(), 10.0);
        assert_eq!(r.max_y(), 15.0);

        let sr = r.standardized();
        assert_eq!(sr.x(), -90.0);
        assert_eq!(sr.y(), -185.0);
        assert_eq!(sr.width(), 100.0);
        assert_eq!(sr.height(), 200.0);
        assert_eq!(sr.min_x(), -90.0);
        assert_eq!(sr.min_y(), -185.0);
        assert_eq!(sr.max_x(), 10.0);
        assert_eq!(sr.max_y(), 15.0);

        assert_ne!(r, sr);
        assert!(r.geometry_eq(&sr));

        assert!(!r.is_empty());
        assert!(!r.is_infinite());
        assert!(!r.is_null());
        assert!(cg::Rect::null().is_null());
    }
}
