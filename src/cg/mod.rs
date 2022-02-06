pub type Float = f64;
#[repr(C)]
pub struct Point {
    pub x: Float,
    pub y: Float,
}

#[repr(C)]
pub struct Size {
    pub width: Float,
    pub height: Float,
}

#[repr(C)]
pub struct Rect {
    pub origin: Point,
    pub size: Size,
}

pub mod color_space;
pub use color_space::ColorSpace;