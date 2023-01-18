mod geometry;
pub use geometry::Float;
pub use geometry::Point;
pub use geometry::Rect;
pub use geometry::Size;
pub use geometry::Vector;

pub mod color_space;
pub use color_space::ColorRenderingIntent;
pub use color_space::ColorSpace;
pub use color_space::ColorSpaceModel;

mod color;
pub use color::Color;

mod window;
pub use window::ID as WindowID;

pub mod direct_display;
#[cfg(target_os = "macos")]
pub use direct_display::main_display_id;
pub use direct_display::ID as DirectDisplayID;

pub mod affine_transform;
pub use affine_transform::AffineTransform;
pub use affine_transform::Components as AffineTransformComponents;

mod error;
pub use error::Callback as ErrorCallback;
pub use error::Error;

pub mod image;
pub use image::AlphaInfo as ImageAlphaInfo;
pub use image::Image;

pub mod image_properties;
pub use image_properties::Orientation as ImagePropertyOrientation;

#[cfg(target_os = "macos")]
mod display_stream;
pub use display_stream::DisplayStream;
pub use display_stream::FrameStatus as DisplayStreamFrameStatus;
pub use display_stream::PropertyKey as DisplayStreamPropertyKey;
pub use display_stream::Update as DisplayStreamUpdate;
pub use display_stream::UpdateRectType as DisplayStreamUpdateRectType;
pub use display_stream::YCbCrMatrix as DisplayStreamYCbCrMatrix;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}
