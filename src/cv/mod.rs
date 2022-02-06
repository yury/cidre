pub mod buffer;

pub use buffer::AttachmentMode;
pub use buffer::Buffer;

pub mod image_buffer;
pub use image_buffer::ImageBuffer;

pub mod pixel_buffer;
pub use pixel_buffer::PixelBuffer;
pub use pixel_buffer::PixelFormat;
pub use pixel_buffer::buffer_attribute_keys;

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {}
