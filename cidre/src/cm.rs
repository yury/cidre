mod base;
pub use base::ItemCount;
pub use base::ItemIndex;

mod format_description;
pub use format_description::AudioFormatDesc;
pub use format_description::FormatDesc;
pub use format_description::FormatDescExtensionKey;
pub use format_description::MediaType;
pub use format_description::PixelFormat;
pub use format_description::VideoCodec;
pub use format_description::VideoDimensions;
pub use format_description::VideoFormatDesc;

mod format_description_bridge;
pub use format_description_bridge::errors as format_description_bridge_errors;
pub use format_description_bridge::swap_be_image_desc_to_host;
pub use format_description_bridge::swap_be_sound_desc_to_host;
pub use format_description_bridge::swap_host_image_desc_to_be;
pub use format_description_bridge::swap_host_sound_desc_to_be;
pub use format_description_bridge::ImageDescriptionFlavor;
pub use format_description_bridge::SoundDescriptionFlavor;

mod time;
pub use time::Time;
pub use time::TimeEpoch;
pub use time::TimeFlags;
pub use time::TimeRange;
pub use time::TimeRoundingMethod;
pub use time::TimeScale;
pub use time::TimeValue;

pub mod sample_buffer;

#[cfg(feature = "cat")]
pub use sample_buffer::BlockBufAudioBufList;
pub use sample_buffer::Flags as SampleBufFlags;
pub use sample_buffer::SampleBuf;
pub use sample_buffer::SampleTimingInfo;

pub mod attachment;
pub use attachment::Bearer as AttachmentBearer;
pub use attachment::Mode as AttachmentMode;

pub mod block_buffer;
pub use block_buffer::BlockBuf;
pub use block_buffer::Flags as BlockBufFlags;

pub mod sync;
pub use sync::Clock;
pub use sync::ClockOrTimebase;
pub use sync::Timebase;

pub mod memory_pool;
pub use memory_pool::keys as memory_pool_options;
pub use memory_pool::MemoryPool;

pub mod simple_queue;
pub use simple_queue::Error as SimpleQueueError;
pub use simple_queue::SimpleQueue;

#[link(name = "CoreMedia", kind = "framework")]
extern "C" {}

#[cfg(all(feature = "cmio", target_os = "macos"))]
pub mod io;
