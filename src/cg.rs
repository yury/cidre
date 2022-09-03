pub mod geometry;
pub use geometry::Float;
pub use geometry::Point;
pub use geometry::Rect;
pub use geometry::Size;
pub use geometry::Vector;

pub mod color_space;
pub use color_space::ColorRenderingIntent;
pub use color_space::ColorSpace;
pub use color_space::ColorSpaceModel;

pub mod color;
pub use color::Color;

pub mod window;
pub use window::ID as WindowID;

pub mod direct_display;
pub use direct_display::main_display_id;
pub use direct_display::ID as DirectDisplayID;

pub mod affine_transform;
pub use affine_transform::AffineTransform;
pub use affine_transform::Components as AffineTransformComponents;

pub mod error;
pub use error::Error;

pub mod image;
pub use image::Image;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}
