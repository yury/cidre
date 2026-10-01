use crate::cg;

#[doc(alias = "NSPoint")]
pub type Point = cg::Point;

#[doc(alias = "NSSize")]
pub type Size = cg::Size;

#[doc(alias = "NSRect")]
pub type Rect = cg::Rect;

/// An edge of a rectangle.
#[doc(alias = "NSRectEdge")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum RectEdge {
    MinX = 0,
    MinY = 1,
    MaxX = 2,
    MaxY = 3,
}
