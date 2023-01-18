use crate::{arc, cf};

pub type Float = f64;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}

impl Point {
    pub fn zero() -> Self {
        Default::default()
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
    pub fn new(x: Float, y: Float) -> Self {
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
    pub fn zero() -> Self {
        Default::default()
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
    pub fn new(width: Float, height: Float) -> Self {
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
    pub fn zero() -> Self {
        Default::default()
    }

    /// ```
    /// use cidre::cg;
    ///
    /// let d = cg::Rect::zero().dictionary_representaion();
    /// assert_eq!(d.len(), 4);
    /// ```
    pub fn dictionary_representaion(&self) -> arc::R<cf::Dictionary> {
        unsafe { CGRectCreateDictionaryRepresentation(*self) }
    }

    #[inline]
    pub fn new(x: Float, y: Float, width: Float, height: Float) -> Self {
        Self {
            origin: Point { x, y },
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

extern "C" {
    fn CGPointCreateDictionaryRepresentation(point: Point) -> arc::R<cf::Dictionary>;
    fn CGSizeCreateDictionaryRepresentation(size: Size) -> arc::R<cf::Dictionary>;
    fn CGRectCreateDictionaryRepresentation(rect: Rect) -> arc::R<cf::Dictionary>;
}
