use crate::cf;

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
    /// use cidre::{cf, cg};
    /// 
    /// let d = cg::Point::zero().dictionary_representaion();
    /// assert_eq!(d.len(), 2);
    /// ```
    pub fn dictionary_representaion<'a>(&self) -> cf::Retained<'a, cf::Dictionary> {
        unsafe {
            CGPointCreateDictionaryRepresentation(*self)
        }
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
    /// use cidre::{cf, cg};
    /// 
    /// let d = cg::Size::zero().dictionary_representaion();
    /// assert_eq!(d.len(), 2);
    /// ```
    pub fn dictionary_representaion<'a>(&self) -> cf::Retained<'a, cf::Dictionary> {
        unsafe {
            CGSizeCreateDictionaryRepresentation(*self)
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
#[repr(C)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

impl Rect {
    pub fn zero() -> Self {
        Default::default()
    }

    /// ```
    /// use cidre::{cf, cg};
    /// 
    /// let d = cg::Rect::zero().dictionary_representaion();
    /// assert_eq!(d.len(), 4);
    /// ```
    pub fn dictionary_representaion<'a>(&self) -> cf::Retained<'a, cf::Dictionary> {
        unsafe {
            CGRectCreateDictionaryRepresentation(*self)
        }
    }
}

extern "C" {
    fn CGPointCreateDictionaryRepresentation<'a>(point: Point) -> cf::Retained<'a, cf::Dictionary>;
    fn CGSizeCreateDictionaryRepresentation<'a>(size: Size) -> cf::Retained<'a, cf::Dictionary>;
    fn CGRectCreateDictionaryRepresentation<'a>(rect: Rect) -> cf::Retained<'a, cf::Dictionary>;
}