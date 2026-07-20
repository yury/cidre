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
    #[doc(alias = "CGPointCreateDictionaryRepresentation")]
    #[inline]
    pub fn dictionary_representaion(&self) -> arc::R<cf::Dictionary> {
        unsafe { CGPointCreateDictionaryRepresentation(*self) }
    }

    #[doc(alias = "CGPointMakeWithDictionaryRepresentation")]
    #[inline]
    pub fn from_dictionary_representation(dict: &cf::Dictionary) -> Option<Self> {
        let mut rect = std::mem::MaybeUninit::uninit();
        if unsafe { CGPointMakeWithDictionaryRepresentation(dict, rect.as_mut_ptr()) } {
            Some(unsafe { rect.assume_init() })
        } else {
            None
        }
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
    #[doc(alias = "CGSizeCreateDictionaryRepresentation")]
    #[inline]
    pub fn dictionary_representaion(&self) -> arc::R<cf::Dictionary> {
        unsafe { CGSizeCreateDictionaryRepresentation(*self) }
    }

    #[doc(alias = "CGSizeMakeWithDictionaryRepresentation")]
    #[inline]
    pub fn from_dictionary_representation(dict: &cf::Dictionary) -> Option<Self> {
        let mut rect = std::mem::MaybeUninit::uninit();
        if unsafe { CGSizeMakeWithDictionaryRepresentation(dict, rect.as_mut_ptr()) } {
            Some(unsafe { rect.assume_init() })
        } else {
            None
        }
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
    #[doc(alias = "CGRectCreateDictionaryRepresentation")]
    #[inline]
    pub fn dictionary_representaion(&self) -> arc::R<cf::Dictionary> {
        unsafe { CGRectCreateDictionaryRepresentation(*self) }
    }

    #[doc(alias = "CGRectMakeWithDictionaryRepresentation")]
    #[inline]
    pub fn from_dictionary_representation(dict: &cf::Dictionary) -> Option<Self> {
        let mut rect = std::mem::MaybeUninit::uninit();
        if unsafe { CGRectMakeWithDictionaryRepresentation(dict, rect.as_mut_ptr()) } {
            Some(unsafe { rect.assume_init() })
        } else {
            None
        }
    }

    #[inline]
    pub const fn new(x: Float, y: Float, width: Float, height: Float) -> Self {
        Self {
            origin: Point { x, y },
            size: Size { width, height },
        }
    }

    #[inline]
    pub const fn with_wh(width: Float, height: Float) -> Self {
        Self {
            origin: Point::zero(),
            size: Size { width, height },
        }
    }

    #[inline]
    pub const fn with_size(size: Size) -> Self {
        Self {
            origin: Point::zero(),
            size,
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

impl From<(f32, f32)> for Point {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0 as _,
            y: value.1 as _,
        }
    }
}

impl From<(Float, Float)> for Point {
    fn from(value: (Float, Float)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Rect> for Point {
    fn from(value: Rect) -> Self {
        value.origin
    }
}

impl From<Rect> for Size {
    fn from(value: Rect) -> Self {
        value.size
    }
}

impl From<Size> for Rect {
    fn from(value: Size) -> Self {
        Rect::with_size(value)
    }
}

impl Into<arc::R<cf::Dictionary>> for Point {
    fn into(self) -> arc::R<cf::Dictionary> {
        self.dictionary_representaion()
    }
}

impl Into<arc::R<cf::Dictionary>> for Size {
    fn into(self) -> arc::R<cf::Dictionary> {
        self.dictionary_representaion()
    }
}

impl Into<arc::R<cf::Dictionary>> for Rect {
    fn into(self) -> arc::R<cf::Dictionary> {
        self.dictionary_representaion()
    }
}

impl TryFrom<&cf::Dictionary> for Point {
    type Error = ();

    fn try_from(value: &cf::Dictionary) -> Result<Self, Self::Error> {
        Self::from_dictionary_representation(value).ok_or(())
    }
}

impl TryFrom<&cf::Dictionary> for Size {
    type Error = ();

    fn try_from(value: &cf::Dictionary) -> Result<Self, Self::Error> {
        Self::from_dictionary_representation(value).ok_or(())
    }
}

impl TryFrom<&cf::Dictionary> for Rect {
    type Error = ();

    fn try_from(value: &cf::Dictionary) -> Result<Self, Self::Error> {
        Self::from_dictionary_representation(value).ok_or(())
    }
}

unsafe extern "C" {
    static CGRectNull: Rect;

    fn CGPointCreateDictionaryRepresentation(point: Point) -> arc::R<cf::Dictionary>;
    fn CGSizeCreateDictionaryRepresentation(size: Size) -> arc::R<cf::Dictionary>;
    fn CGRectCreateDictionaryRepresentation(rect: Rect) -> arc::R<cf::Dictionary>;

    fn CGPointMakeWithDictionaryRepresentation(dict: &cf::Dictionary, rect: *mut Point) -> bool;
    fn CGSizeMakeWithDictionaryRepresentation(dict: &cf::Dictionary, rect: *mut Size) -> bool;
    fn CGRectMakeWithDictionaryRepresentation(dict: &cf::Dictionary, rect: *mut Rect) -> bool;

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
    use crate::{arc, cg};

    #[test]
    fn point_basics() {
        let p0 = cg::Point::new(10.0, 20.0);
        let d: arc::R<_> = p0.into();
        assert_eq!(d.len(), 2);
        let p1: cg::Point = d
            .as_ref()
            .try_into()
            .expect("failed to convert dict to point");
        assert_eq!(p0, p1);
    }

    #[test]
    fn size_basics() {
        let s0 = cg::Size::new(10.0, 20.0);
        let d: arc::R<_> = s0.into();
        assert_eq!(d.len(), 2);
        let s1: cg::Size = d
            .as_ref()
            .try_into()
            .expect("failed to convert dict to point");
        assert_eq!(s0, s1);
    }

    #[test]
    fn rect_basics() {
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

        let d: arc::R<_> = cg::Rect::zero().into();
        assert_eq!(d.len(), 4);
        let rect: cg::Rect = d
            .as_ref()
            .try_into()
            .expect("failed to convert dictionary to cg::Rect");
        assert_eq!(rect, cg::Rect::zero());
    }
}
