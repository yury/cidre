pub mod errors;
pub use errors::DecodeFrameFlags;
pub use errors::DecodeInfoFlags;
pub use errors::EncodeInfoFlags;

pub mod session;
pub use session::Session;

pub mod compression;
pub use compression::CompressionSession;

#[link(name = "VideoToolbox", kind = "framework")]
extern "C" {}
