pub mod base;
pub use base::Flags as TimeFlags;
pub use base::OptionFlags;
pub use base::SMPTETime;
pub use base::Time;
pub use base::TimeStamp;
pub use base::TimeStampFlags;

mod _return;
pub use _return::Return;

pub mod buffer;
pub use buffer::AttachmentMode;
pub use buffer::Buf;

mod image_buffer;
pub use image_buffer::attachment;
pub use image_buffer::ImageBuf;

pub mod pixel_buffer;
pub use pixel_buffer::keys as pixel_buffer_keys;
pub use pixel_buffer::PixelBuf;
pub use pixel_buffer::PixelFormat;

pub mod pixel_buffer_pool;
pub use pixel_buffer_pool::FlushFlags as PixelBufferPoolFlushFlags;
pub use pixel_buffer_pool::PixelBufferPool;

pub mod pixel_format_description;
pub use pixel_format_description::all_pixel_formats as pixel_format_description_array_with_all_pixel_format_types;
pub use pixel_format_description::avaiable_compressed as compressed_pixel_format_available;
pub use pixel_format_description::create as pixel_format_description_create;

#[cfg(feature = "mtl")]
pub mod metal;
#[cfg(feature = "mtl")]
pub use metal::texture_cache_keys as metal_texture_cache_keys;
#[cfg(feature = "mtl")]
pub use metal::texture_keys as metal_texture_keys;
#[cfg(feature = "mtl")]
pub use metal::Texture as MetalTexture;
#[cfg(feature = "mtl")]
pub use metal::TextureCache as MetalTextureCache;

#[cfg(target_os = "macos")]
pub mod display_link;
#[cfg(target_os = "macos")]
pub use display_link::DisplayLink;
#[cfg(target_os = "macos")]
pub use display_link::OutputCallback as DisplayLinkOutputCallback;

mod host_time;
pub use host_time::current_host_time;
pub use host_time::host_clock_frequency;
pub use host_time::host_clock_minimum_time_delta;

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {}
