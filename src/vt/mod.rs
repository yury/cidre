pub mod errors;
pub use errors::DecodeFrameFlags;
pub use errors::DecodeInfoFlags;
pub use errors::EncodeInfoFlags;

pub mod session;
pub use session::Session;

pub mod compression_session;
pub use compression_session::CompressionSession;

#[link(name = "VideoToolbox", kind = "framework")]
extern "C" {}
