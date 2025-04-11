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
}
