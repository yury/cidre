use crate::{define_opts, os::Error};

pub const PROPERTY_NOT_SUPPORTED: Error = Error::new_unchecked(-12900);
pub const PROPERTY_READ_ONLY: Error = Error::new_unchecked(-12901);
pub const PARAMETER: Error = Error::new_unchecked(-12902);
pub const INVALID_SESSION: Error = Error::new_unchecked(-12903);
pub const ALLOCATION_FAILED: Error = Error::new_unchecked(-12904);
pub const PIXEL_TRANSFER_NOT_SUPPORTED: Error = Error::new_unchecked(-12905); // c.f. -8961 ?
pub const COULD_NOT_FIND_VIDEO_DECODER: Error = Error::new_unchecked(-12906);
pub const COULD_NOT_CREATE_INSTANCE: Error = Error::new_unchecked(-12907);
pub const COULD_NOT_FIND_VIDEO_ENCODER: Error = Error::new_unchecked(-12908);
pub const VIDEO_DECODER_BAD_DATA: Error = Error::new_unchecked(-12909); // c.f. -8969
pub const VIDEO_DECODER_UNSUPPORTED_DATA_FORMAT: Error = Error::new_unchecked(-12910); // c.f. -8970
pub const VIDEO_DECODER_MALFUNCTION: Error = Error::new_unchecked(-12911); // c.f. -8960
pub const VIDEO_ENCODER_MALFUNCTION: Error = Error::new_unchecked(-12912);
pub const VIDEO_DECODER_NOT_AVAILABLE_NOW: Error = Error::new_unchecked(-12913);
pub const IMAGE_ROTATION_NOT_SUPPORTED: Error = Error::new_unchecked(-12914);
pub const VIDEO_ENCODER_NOT_AVAILABLE_NOW: Error = Error::new_unchecked(-12915);
pub const FORMAT_DESCRIPTION_CHANGE_NOT_SUPPORTED: Error = Error::new_unchecked(-12916);
pub const INSUFFICIENT_SOURCE_COLOR_DATA: Error = Error::new_unchecked(-12917);
pub const COULD_NOT_CREATE_COLOR_CORRECTION_DATA: Error = Error::new_unchecked(-12918);
pub const COLOR_SYNC_TRANSFORM_CONVERT_FAILED: Error = Error::new_unchecked(-12919);
pub const VIDEO_DECODER_AUTHORIZATION: Error = Error::new_unchecked(-12210);
pub const VIDEO_ENCODER_AUTHORIZATION: Error = Error::new_unchecked(-12211);
pub const COLOR_CORRECTION_PIXEL_TRANSFER_FAILED: Error = Error::new_unchecked(-12212);
pub const MULTI_PASS_STORAGE_IDENTIFIER_MISMATCH: Error = Error::new_unchecked(-12213);
pub const MULTI_PASS_STORAGE_INVALID: Error = Error::new_unchecked(-12214);
pub const FRAME_SILO_INVALID_TIME_STAMP: Error = Error::new_unchecked(-12215);
pub const FRAME_SILO_INVALID_TIME_RANGE: Error = Error::new_unchecked(-12216);
pub const COULD_NOT_FIND_TEMPORAL_FILTER: Error = Error::new_unchecked(-12217);
pub const PIXEL_TRANSFER_NOT_PERMITTED: Error = Error::new_unchecked(-12218);
pub const COLOR_CORRECTION_IMAGE_ROTATION_FAILED: Error = Error::new_unchecked(-12219);
pub const VIDEO_DECODER_REMOVED: Error = Error::new_unchecked(-17690);
pub const SESSION_MALFUNCTION: Error = Error::new_unchecked(-17691);
pub const VIDEO_DECODER_NEEDS_ROSETTA: Error = Error::new_unchecked(-17692);
pub const VIDEO_ENCODER_NEEDS_ROSETTA: Error = Error::new_unchecked(-17693);
pub const VIDEO_DECODER_REFERENCE_MISSING: Error = Error::new_unchecked(-17694);
pub const VIDEO_DECODER_CALLBACK_MESSAGING: Error = Error::new_unchecked(-17695);

define_opts!(
    #[doc(alias = "VTDecodeFrameFlags")]
    pub DecodeFrameFlags(u32)
);

impl DecodeFrameFlags {
    pub const ENABLE_ASYNCHRONOUS_DECOMPRESSION: Self = Self(1 << 0);
    pub const DO_NOT_OUTPUT_FRAME: Self = Self(1 << 1);
    pub const _1X_REAL_TIME_PLAYBACK: Self = Self(1 << 2);
    pub const ENABLE_TEMPORAL_PROCESSING: Self = Self(1 << 3);
}

define_opts!(
    #[doc(alias = "VTDecodeInfoFlags")]
    pub DecodeInfoFlags(u32)
);

impl DecodeInfoFlags {
    pub const ASYNCHRONOUS: Self = Self(1u32 << 0);
    pub const FRAME_DROPPED: Self = Self(1u32 << 1);
    pub const IMAGE_BUFFER_MODIFIABLE: Self = Self(1u32 << 2);
    pub const SKIPPED_LEADING_FRAME_DROPPED: Self = Self(1u32 << 3);
}

define_opts!(
    #[doc(alias = "VTEncodeInfoFlags")]
    pub EncodeInfoFlags(u32)
);

/// Flags that indicate encoder state.
impl EncodeInfoFlags {
    /// A flag that indicates that an encode operation ran asynchronously.
    pub const ASYNCHRONOUS: Self = Self(1u32 << 0);

    /// A flag that indicates that a frame dropped during encoding.
    pub const FRAME_DROPPED: Self = Self(1u32 << 1);
}
