pub mod base;

pub use base::ItemCount;
pub use base::ItemIndex;

pub mod format_description;
pub use format_description::FormatDescription;
pub use format_description::MediaType;
pub use format_description::PixelFormatType;
pub use format_description::VideoCodecType;
pub use format_description::VideoDimensions;
pub use format_description::VideoFormatDescription;

pub mod time;
pub use time::Time;
pub use time::TimeEpoch;
pub use time::TimeFlags;
pub use time::TimeRoundingMethod;
pub use time::TimeScale;
pub use time::TimeValue;

pub mod sample_buffer;
pub use sample_buffer::SampleBuffer;

pub mod block_buffer;
pub use block_buffer::BlockBuffer;
pub use block_buffer::BlockBufferFlags;

#[link(name = "CoreMedia", kind = "framework")]
extern "C" {}
