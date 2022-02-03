mod time;
pub use time::Time;
pub use time::TimeEpoch;
pub use time::TimeFlags;
pub use time::TimeRoundingMethod;
pub use time::TimeScale;
pub use time::TimeValue;

mod sample_buffer;
pub use sample_buffer::SampleBuffer;

mod block_buffer;
pub use block_buffer::BlockBuffer;
pub use block_buffer::BlockBufferFlags;

#[link(name = "CoreMedia", kind = "framework")]
extern "C" {}
