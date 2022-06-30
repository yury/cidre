use std::ffi::c_void;

use crate::{
    av, cat,
    cf::{self, Allocator, Retained},
    cm, cv, define_cf_type, os, FourCharCode,
};

#[repr(transparent)]
pub struct PixelFormatType(pub FourCharCode);

impl PixelFormatType {
    pub const _32_ARGB: Self = Self(32);
    pub const _32_BGRA: Self = Self::from_be_bytes(b"BGRA");
    pub const _24_RGB: Self = Self(24);
    pub const _422_YP_CB_CR_8: Self = Self::from_be_bytes(b"2vuy");
    pub const _422_YP_CB_CR_8_YUVS: Self = Self::from_be_bytes(b"yuvs");

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
    pub const MUXED: Self = Self::from_be_bytes(b"muxx");
    pub const TEXT: Self = Self::from_be_bytes(b"text");
    pub const CLOSED_CAPTION: Self = Self::from_be_bytes(b"clcp");
    pub const SUBTITLE: Self = Self::from_be_bytes(b"sbtl");
    pub const TIME_CODE: Self = Self::from_be_bytes(b"tmcd");
    pub const METADATA: Self = Self::from_be_bytes(b"meta");

    const fn from_be_bytes(bytes: &[u8; 4]) -> Self {
        Self(FourCharCode::from_be_bytes(*bytes))
    }
}

#[repr(transparent)]
pub struct VideoCodecType(FourCharCode);

impl VideoCodecType {
    pub const _422_YP_CB_CR_8: Self = Self(PixelFormatType::_422_YP_CB_CR_8.0);
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

    pub fn create(
        allocator: Option<&cf::Allocator>,
        media_type: MediaType,
        media_sub_type: FourCharCode,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<Retained<FormatDescription>>,
    ) -> os::Status {
        unsafe {
            CMFormatDescriptionCreate(
                allocator,
                media_type,
                media_sub_type,
                extensions,
                format_description_out,
            )
        }
    }

    /// ```
    /// use cidre::{cm, mac_types::FourCharCode};
    ///
    /// let desc = cm::FormatDescription::new(cm::MediaType::VIDEO, FourCharCode::from_be_bytes(*b"avc1"), None).unwrap();
    /// ```
    pub fn new<'a>(
        media_type: MediaType,
        media_sub_type: FourCharCode,
        extensions: Option<&cf::Dictionary>,
    ) -> Result<Retained<'a, Self>, os::Status> {
        let mut format_desc = None;
        let res = Self::create(
            None,
            media_type,
            media_sub_type,
            extensions,
            &mut format_desc,
        );
        unsafe { res.to_result(format_desc) }
    }
}

pub type VideoFormatDescription = FormatDescription;

impl VideoFormatDescription {
    /// ```
    /// use cidre::cm;
    ///
    /// let desc = cm::VideoFormatDescription::new_video(cm::VideoCodecType::H264, 1920, 1080, None).unwrap();
    /// ```
    pub fn new_video<'a>(
        codec_type: VideoCodecType,
        width: i32,
        height: i32,
        extensions: Option<&cf::Dictionary>,
    ) -> Result<Retained<'a, Self>, os::Status> {
        let mut format_desc = None;
        let res = Self::create_video(
            None,
            codec_type,
            width,
            height,
            extensions,
            &mut format_desc,
        );
        unsafe { res.to_result(format_desc) }
    }

    pub fn create_video(
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

impl AudioFormatDescription {
    pub fn create_audio(
        allocator: Option<&cf::Allocator>,
        asbd: &cat::audio::StreamBasicDescription,
        layout_size: usize,
        layout: Option<&cat::AudioChannelLayout<1>>,
        magic_cookie_size: usize,
        magic_cookie: Option<&c_void>,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<Retained<AudioFormatDescription>>,
    ) -> os::Status {
        unsafe {
            CMAudioFormatDescriptionCreate(
                allocator,
                asbd,
                layout_size,
                layout,
                magic_cookie_size,
                magic_cookie,
                extensions,
                format_description_out,
            )
        }
    }
}

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

    fn CMAudioFormatDescriptionCreate(
        allocator: Option<&cf::Allocator>,
        asbd: &cat::audio::StreamBasicDescription,
        layout_size: usize,
        layout: Option<&cat::AudioChannelLayout<1>>,
        magic_cookie_size: usize,
        magic_cookie: Option<&c_void>,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<Retained<AudioFormatDescription>>,
    ) -> os::Status;

    fn CMFormatDescriptionCreate(
        allocator: Option<&cf::Allocator>,
        media_type: MediaType,
        media_sub_type: FourCharCode,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<Retained<FormatDescription>>,
    ) -> os::Status;
}
