pub mod geometry;
pub use geometry::Float;
pub use geometry::Point;
pub use geometry::Rect;
pub use geometry::Size;

pub mod color_space;
pub use color_space::ColorSpace;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}
