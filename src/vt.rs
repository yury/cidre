pub mod errors;
pub use errors::DecodeFrameFlags;
pub use errors::DecodeInfoFlags;
pub use errors::EncodeInfoFlags;

pub mod session;
pub use session::Session;

pub mod compression;
pub use compression::properties as compression_properties;
pub use compression::Session as CompressionSession;

pub mod pixel_transfer;
#[cfg(not(target_os = "ios"))]
pub use pixel_transfer::session as PixelTransferSession;
pub use pixel_transfer::properties as pixel_transfer_properties;

pub mod video_encoder_list;

#[link(name = "VideoToolbox", kind = "framework")]
extern "C" {}
