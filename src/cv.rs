pub mod base;
pub use base::OptionFlags;
pub use base::SMPTETime;
pub use base::Time;
pub use base::TimeStamp;

pub mod _return;
pub use _return::Return;

pub mod buffer;

pub use buffer::AttachmentMode;
pub use buffer::Buffer;

pub mod image_buffer;
pub use image_buffer::ImageBuffer;

pub mod pixel_buffer;
pub use pixel_buffer::PixelBuffer;
pub use pixel_buffer::PixelFormatType;

pub mod pixel_buffer_pool;
pub use pixel_buffer_pool::FlushFlags as PixelBufferPoolFlushFlags;
pub use pixel_buffer_pool::PixelBufferPool;

pub mod pixel_format_description;
pub use pixel_format_description::all_pixel_formats as pixel_format_description_array_with_all_pixel_format_types;
pub use pixel_format_description::avaiable_compressed as compressed_pixel_format_available;
pub use pixel_format_description::create as pixel_format_description_create;

pub mod metal;
pub use metal::texture_cache_keys as metal_texture_cache_keys;
pub use metal::texture_keys as metal_texture_keys;
pub use metal::Texture as MetalTexture;
pub use metal::TextureCache as MetalTextureCache;

#[cfg(target_os = "macos")]
pub mod display_link;
#[cfg(target_os = "macos")]
pub use display_link::DisplayLink;
#[cfg(target_os = "macos")]
pub use display_link::DisplayLinkOutputCallback;

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {}
