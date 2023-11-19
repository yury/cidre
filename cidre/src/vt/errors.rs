use crate::{define_options, os::Status};

pub const PROPERTY_NOT_SUPPORTED: Status = Status(-12900);
pub const PROPERTY_READ_ONLY: Status = Status(-12901);
pub const PARAMETER: Status = Status(-12902);
pub const INVALID_SESSION: Status = Status(-12903);
pub const ALLOCATION_FAILED: Status = Status(-12904);
pub const PIXEL_TRANSFER_NOT_SUPPORTED: Status = Status(-12905); // c.f. -8961 ?
pub const COULD_NOT_FIND_VIDEO_DECODER: Status = Status(-12906);
pub const COULD_NOT_CREATE_INSTANCE: Status = Status(-12907);
pub const COULD_NOT_FIND_VIDEO_ENCODER: Status = Status(-12908);
pub const VIDEO_DECODER_BAD_DATA: Status = Status(-12909); // c.f. -8969
pub const VIDEO_DECODER_UNSUPPORTED_DATA_FORMAT: Status = Status(-12910); // c.f. -8970
pub const VIDEO_DECODER_MALFUNCTION: Status = Status(-12911); // c.f. -8960
pub const VIDEO_ENCODER_MALFUNCTION: Status = Status(-12912);
pub const VIDEO_DECODER_NOT_AVAILABLE_NOW: Status = Status(-12913);
pub const IMAGE_ROTATION_NOT_SUPPORTED: Status = Status(-12914);
pub const VIDEO_ENCODER_NOT_AVAILABLE_NOW: Status = Status(-12915);
pub const FORMAT_DESCRIPTION_CHANGE_NOT_SUPPORTED: Status = Status(-12916);
pub const INSUFFICIENT_SOURCE_COLOR_DATA: Status = Status(-12917);
pub const COULD_NOT_CREATE_COLOR_CORRECTION_DATA: Status = Status(-12918);
pub const COLOR_SYNC_TRANSFORM_CONVERT_FAILED: Status = Status(-12919);
pub const VIDEO_DECODER_AUTHORIZATION: Status = Status(-12210);
pub const VIDEO_ENCODER_AUTHORIZATION: Status = Status(-12211);
pub const COLOR_CORRECTION_PIXEL_TRANSFER_FAILED: Status = Status(-12212);
pub const MULTI_PASS_STORAGE_IDENTIFIER_MISMATCH: Status = Status(-12213);
pub const MULTI_PASS_STORAGE_INVALID: Status = Status(-12214);
pub const FRAME_SILO_INVALID_TIME_STAMP: Status = Status(-12215);
pub const FRAME_SILO_INVALID_TIME_RANGE: Status = Status(-12216);
pub const COULD_NOT_FIND_TEMPORAL_FILTER: Status = Status(-12217);
pub const PIXEL_TRANSFER_NOT_PERMITTED: Status = Status(-12218);
pub const COLOR_CORRECTION_IMAGE_ROTATION_FAILED: Status = Status(-12219);
pub const VIDEO_DECODER_REMOVED: Status = Status(-17690);
pub const SESSION_MALFUNCTION: Status = Status(-17691);
pub const VIDEO_DECODER_NEEDS_ROSETTA: Status = Status(-17692);
pub const VIDEO_ENCODER_NEEDS_ROSETTA: Status = Status(-17693);
pub const VIDEO_DECODER_REFERENCE_MISSING: Status = Status(-17694);
pub const VIDEO_DECODER_CALLBACK_MESSAGING: Status = Status(-17695);

define_options!(pub DecodeFrameFlags(u32));

impl DecodeFrameFlags {
    pub const ENABLE_ASYNCHRONOUS_DECOMPRESSION: Self = Self(1 << 0);
    pub const DO_NOT_OUTPUT_FRAME: Self = Self(1 << 1);
    pub const _1X_REAL_TIME_PLAYBACK: Self = Self(1 << 2);
    pub const ENABLE_TEMPORAL_PROCESSING: Self = Self(1 << 3);
}

define_options!(pub DecodeInfoFlags(u32));

impl DecodeInfoFlags {
    pub const ASYNCHRONOUS: Self = Self(1u32 << 0);
    pub const FRAME_DROPPED: Self = Self(1u32 << 1);
    pub const IMAGE_BUFFER_MODIFIABLE: Self = Self(1u32 << 2);
    pub const SKIPPED_LEADING_FRAME_DROPPED: Self = Self(1u32 << 3);
}

define_options!(pub EncodeInfoFlags(u32));

/// Flags that indicate encoder state.
impl EncodeInfoFlags {
    /// A flag that indicates that an encode operation ran asynchronously.
    pub const ASYNCHRONOUS: Self = Self(1u32 << 0);

    /// A flag that indicates that a frame dropped during encoding.
    pub const FRAME_DROPPED: Self = Self(1u32 << 1);
}
