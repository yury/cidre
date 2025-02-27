pub mod errors;
pub use errors::DecodeFrameFlags;
pub use errors::DecodeInfoFlags;
pub use errors::EncodeInfoFlags;

pub mod session;
pub use session::Session;

pub mod compression;
pub use compression::Session as CompressionSession;
pub use compression::properties as compression_properties;

pub mod decompression;
#[cfg(feature = "blocks")]
pub use decompression::MultiImageCapableOutputHandler as DecompressionMultiImageCapableOutputHandler;
pub use decompression::OutputCb as DecompressionOutputCb;
pub use decompression::OutputCbRecord as DecompressionOutputCbRecord;
pub use decompression::OutputMultiImageCb as DecompressionOutputMultiImageCb;
pub use decompression::Session as DecompressionSession;
pub use decompression::properties as decompression_properties;
pub use decompression::session::is_hardware_decode_supported;
pub use decompression::session::is_stereo_mv_hevc_decode_supported;

pub mod pixel_transfer;
pub use pixel_transfer::Session as PixelTransferSession;
pub use pixel_transfer::properties as pixel_transfer_properties;

pub mod pixel_rotation;
pub use pixel_rotation::Session as PixelRotationSession;
pub use pixel_rotation::properties as pixel_rotation_properties;

pub mod video_encoder_list;

pub mod utilities;
pub use utilities::cg_image_from_cv_pixel_buf;

#[link(name = "VideoToolbox", kind = "framework")]
unsafe extern "C" {}
