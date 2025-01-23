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
pub use window::Id as WindowId;

mod window_level;
pub use window_level::WindowLevel;

pub mod direct_display;
#[cfg(target_os = "macos")]
pub use direct_display::main_display_id;
pub use direct_display::Id as DirectDisplayId;

pub mod affine_transform;
pub use affine_transform::AffineTransform;
pub use affine_transform::Components as AffineTransformComponents;

mod error;
pub use error::Callback as ErrorCallback;
pub use error::Error;
pub use error::Status;

pub mod image;
#[cfg(feature = "iio")]
pub use image::animate_image_at_url;
#[cfg(feature = "iio")]
pub use image::animate_image_at_url_with_block;
#[cfg(feature = "iio")]
pub use image::animate_image_data;
#[cfg(feature = "iio")]
pub use image::animate_image_data_with_block;
#[cfg(feature = "iio")]
pub use image::animation_err as image_animation_err;
#[cfg(feature = "iio")]
pub use image::AnimationBlock as ImageAnimationBlock;
#[cfg(feature = "iio")]
pub use image::AnimationOptKey as ImageAnimationOptKey;
#[cfg(feature = "iio")]
pub use image::ImageDst;
#[cfg(feature = "iio")]
pub use image::ImageSrc;

pub use image::AlphaInfo as ImageAlphaInfo;
pub use image::Image;

pub mod image_properties;
pub use image_properties::Orientation as ImagePropOrientation;

mod font;
pub use font::Font;
pub use font::FontPostScriptFormat;
pub use font::Glyph;
pub use font::Index as FontIndex;

mod path;

#[cfg(feature = "blocks")]
pub use path::ApplyBlock as PathApplyBlock;
pub use path::Element as PathElement;
pub use path::ElementType as PathElementType;
pub use path::LineCap;
pub use path::LineJoin;
pub use path::Path;
pub use path::PathApplierFn;
pub use path::PathMut;

#[cfg(target_os = "macos")]
mod display_stream;
#[cfg(target_os = "macos")]
pub use display_stream::DisplayStream;
#[cfg(all(target_os = "macos", feature = "blocks", feature = "io"))]
pub use display_stream::FrameAvailableHandler as DisplayStreamFrameAvailableHandler;
#[cfg(target_os = "macos")]
pub use display_stream::FrameStatus as DisplayStreamFrameStatus;
#[cfg(target_os = "macos")]
pub use display_stream::PropKey as DisplayStreamPropertyKey;
#[cfg(target_os = "macos")]
pub use display_stream::Update as DisplayStreamUpdate;
#[cfg(target_os = "macos")]
pub use display_stream::UpdateRectType as DisplayStreamUpdateRectType;
#[cfg(target_os = "macos")]
pub use display_stream::YCbCrMatrix as DisplayStreamYCbCrMatrix;

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}
