pub mod buffer;

pub use buffer::AttachmentMode;
pub use buffer::Buffer;

pub mod image_buffer;
pub use image_buffer::ImageBuffer;

pub mod pixel_buffer;
pub use pixel_buffer::PixelBuffer;

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {}
