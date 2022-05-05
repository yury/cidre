use crate::{
    cf::{self, Allocator, Retained},
    cv, define_cf_type, os, FourCharCode,
};

#[repr(transparent)]
pub struct PixelFormatType(pub FourCharCode);

impl PixelFormatType {
    pub const _32ARGB: Self = Self(32);
    pub const _32BGRA: Self = Self::from_be_bytes(b"BGRA");
    pub const _24RGB: Self = Self(24);
    pub const _422YPCBCR8: Self = Self::from_be_bytes(b"2vuy");
    pub const _422YPCBCR8_YUVS: Self = Self::from_be_bytes(b"yuvs");

    const fn from_be_bytes(bytes: &[u8; 4]) -> Self {
        Self(FourCharCode::from_be_bytes(*bytes))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C)]
pub struct VideoDimensions {
    pub width: i32,
    pub height: i32,
}

#[repr(transparent)]
pub struct MediaType(pub FourCharCode);

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
    pub const _422YPCBCR8: Self = Self(PixelFormatType::_422YPCBCR8.0);
    pub const H264: Self = Self::from_be_bytes(b"avc1");
    pub const HEVC: Self = Self::from_be_bytes(b"hvc1");

    const fn from_be_bytes(bytes: &[u8; 4]) -> Self {
        Self(FourCharCode::from_be_bytes(*bytes))
    }
}

define_cf_type!(FormatDescription(cf::Type));

impl FormatDescription {
    pub fn type_id() -> cf::TypeId {
        unsafe { CMFormatDescriptionGetTypeID() }
    }

    pub fn media_type(&self) -> MediaType {
        unsafe { CMFormatDescriptionGetMediaType(self) }
    }

    pub fn media_sub_type(&self) -> FourCharCode {
        unsafe { CMFormatDescriptionGetMediaSubType(self) }
    }

    pub fn extensions(&self) -> Option<&cf::Dictionary> {
        unsafe { CMFormatDescriptionGetExtensions(self) }
    }
}

pub type VideoFormatDescription = FormatDescription;

impl VideoFormatDescription {
    pub fn create(
        allocator: Option<&Allocator>,
        codec_type: VideoCodecType,
        width: i32,
        height: i32,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<Retained<VideoFormatDescription>>,
    ) -> os::Status {
        unsafe {
            CMVideoFormatDescriptionCreate(
                allocator,
                codec_type,
                width,
                height,
                extensions,
                format_description_out,
            )
        }
    }

    pub fn dimensions(&self) -> VideoDimensions {
        unsafe { CMVideoFormatDescriptionGetDimensions(self) }
    }

    pub fn matches_image_buffer(&self, image_buffer: &cv::ImageBuffer) -> bool {
        unsafe { CMVideoFormatDescriptionMatchesImageBuffer(self, image_buffer) }
    }
}

pub type AudioFormatDescription = FormatDescription;

extern "C" {
    fn CMFormatDescriptionGetTypeID() -> cf::TypeId;
    fn CMFormatDescriptionGetMediaType(desc: &FormatDescription) -> MediaType;

    fn CMVideoFormatDescriptionCreate(
        allocator: Option<&cf::Allocator>,
        codec_type: VideoCodecType,
        width: i32,
        height: i32,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<Retained<VideoFormatDescription>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionGetDimensions(
        video_desc: &VideoFormatDescription,
    ) -> VideoDimensions;

    fn CMVideoFormatDescriptionMatchesImageBuffer(
        video_desc: &VideoFormatDescription,
        image_buffer: &cv::ImageBuffer,
    ) -> bool;

    fn CMFormatDescriptionGetMediaSubType(desc: &FormatDescription) -> FourCharCode;

    fn CMFormatDescriptionGetExtensions(desc: &FormatDescription) -> Option<&cf::Dictionary>;
}
