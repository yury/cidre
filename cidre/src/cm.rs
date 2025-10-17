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
pub use format_description::MuxedFormatDesc;
pub use format_description::MuxedStreamType;
pub use format_description::PixelFormat;
pub use format_description::TaggedBufGroupFormatDesc;
pub use format_description::TaggedBufGroupFormatType;
pub use format_description::VideoCodec;
pub use format_description::VideoDimensions;
pub use format_description::VideoFormatDesc;

mod format_description_bridge;
pub use format_description_bridge::ImageDescFlavor;
pub use format_description_bridge::SoundDescFlavor;
pub use format_description_bridge::TextDescFlavor;
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

mod tagged_buffer_group;
pub use tagged_buffer_group::TaggedBufGroup;
pub use tagged_buffer_group::err as tagged_buf_group_err;

mod time;
pub use time::TIME_SCALE_MAX;
pub use time::Time;
pub use time::TimeEpoch;
pub use time::TimeFlags;
pub use time::TimeMapping;
pub use time::TimeRange;
pub use time::TimeRoundingMethod;
pub use time::TimeScale;
pub use time::TimeValue;

pub mod sample_buffer;

pub mod buffer_queue;
pub use buffer_queue::Buf;
pub use buffer_queue::BufBoolCb;
#[cfg(feature = "blocks")]
pub use buffer_queue::BufBoolHandler;
pub use buffer_queue::BufCompareCb;
#[cfg(feature = "blocks")]
pub use buffer_queue::BufCompareHandler;
#[cfg(feature = "blocks")]
pub use buffer_queue::BufHandlers;
pub use buffer_queue::BufQueue;
pub use buffer_queue::BufQueueOf;
pub use buffer_queue::BufSizeCb;
#[cfg(feature = "blocks")]
pub use buffer_queue::BufSizeHandler;
pub use buffer_queue::BufTimeCb;
#[cfg(feature = "blocks")]
pub use buffer_queue::BufTimeHandler;
pub use buffer_queue::SampleBufQueue;
pub use buffer_queue::err as buf_queue_err;

#[cfg(feature = "cat")]
pub use sample_buffer::BlockBufAudioBufList;
#[cfg(feature = "cat")]
pub use sample_buffer::BlockBufAudioBufListN;
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

#[cfg(not(target_os = "macos"))]
pub mod audio_clock;

#[cfg(target_os = "macos")]
pub mod audio_device_clock;

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
