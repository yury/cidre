mod base;
pub use base::ItemCount;
pub use base::ItemIndex;
pub use base::PersistentTrackId;

mod format_description;
pub use format_description::AudioFormatDesc;
pub use format_description::FormatDesc;
pub use format_description::FormatDescExtKey;
pub use format_description::LogTransferFn;
pub use format_description::MediaType;
pub use format_description::PixelFormat;
pub use format_description::VideoCodec;
pub use format_description::VideoDimensions;
pub use format_description::VideoFormatDesc;

mod format_description_bridge;
pub use format_description_bridge::ImageDescFlavor;
pub use format_description_bridge::SoundDescFlavor;
pub use format_description_bridge::err as format_desc_bridge_err;
pub use format_description_bridge::swap_be_image_desc_to_host;
pub use format_description_bridge::swap_be_sound_desc_to_host;
pub use format_description_bridge::swap_host_image_desc_to_be;
pub use format_description_bridge::swap_host_sound_desc_to_be;

mod tag;
pub use tag::Tag;
pub use tag::TagCategory;
pub use tag::TagDataType;
pub use tag::err as tag_err;

mod tag_collection;
pub use tag_collection::TagCollection;
pub use tag_collection::TagCollectionMut;
pub use tag_collection::err as tag_collection_err;

mod time;
pub use time::Time;
pub use time::TimeEpoch;
pub use time::TimeFlags;
pub use time::TimeMapping;
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
pub use sample_buffer::err as sample_buf_err;

pub mod attachment;
pub use attachment::Bearer as AttachBearer;
pub use attachment::Mode as AttachMode;

pub mod block_buffer;
pub use block_buffer::BlockBuf;
pub use block_buffer::Flags as BlockBufFlags;
pub use block_buffer::err as block_buf_err;

pub mod sync;
pub use sync::Clock;
pub use sync::ClockOrTimebase;
pub use sync::Timebase;

pub mod memory_pool;
pub use memory_pool::MemPool;
pub use memory_pool::keys as memory_pool_options;

pub mod simple_queue;
pub use simple_queue::SimpleQueue;
pub use simple_queue::err as simple_queue_err;

#[link(name = "CoreMedia", kind = "framework")]
unsafe extern "C" {}

#[cfg(all(feature = "cmio", target_os = "macos"))]
pub mod io;
