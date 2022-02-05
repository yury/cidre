use crate::{
    cf::{self, Retained},
    cm, define_cf_type, os, FourCharCode,
};

#[repr(transparent)]
pub struct MediaType(FourCharCode);

impl MediaType {
    pub const VIDEO: Self = Self::from_be_bytes(b"vide");
    pub const AUDIO: Self = Self::from_be_bytes(b"soun");

    const fn from_be_bytes(bytes: &[u8; 4]) -> Self {
        Self(FourCharCode::from_be_bytes(*bytes))
    }
}

#[repr(transparent)]
pub struct VideoCodecType(FourCharCode);

impl VideoCodecType {
    pub const H264: Self = Self::from_be_bytes(b"avc1");

    const fn from_be_bytes(bytes: &[u8; 4]) -> Self {
        Self(FourCharCode::from_be_bytes(*bytes))
    }
}

define_cf_type!(FormatDescription(cf::Type));

impl FormatDescription {}

pub type VideoFormatDescription = FormatDescription;

impl VideoFormatDescription {
  pub fn create() {
    
  }
}

extern "C" {
    fn CMVideoFormatDescriptionCreate(
        allocator: Option<&cf::Allocator>,
        codec_type: VideoCodecType,
        width: i32,
        height: i32,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<Retained<VideoFormatDescription>>,
    ) -> os::Status;
}
