pub type Float = f64;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(C)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(C)]
pub struct Size {
    pub width: Float,
    pub height: Float,
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(C)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}
