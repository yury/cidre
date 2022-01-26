mod time;
pub use time::Time;
pub use time::TimeEpoch;
pub use time::TimeFlags;
pub use time::TimeRoundingMethod;
pub use time::TimeScale;
pub use time::TimeValue;

mod sample_buffer;
pub use sample_buffer::SampleBufferRef;

mod block_buffer;
pub use block_buffer::block_buffer_get_type_id;
pub use block_buffer::BlockBufferFlags;
pub use block_buffer::BlockBufferRef;

#[link(name = "CoreMedia", kind = "framework")]
extern "C" {}
