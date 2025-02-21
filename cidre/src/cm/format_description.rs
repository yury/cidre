use std::{ffi::c_void, mem::transmute};

use crate::{
    FourCharCode, api, arc,
    cf::{self, Allocator},
    define_cf_type, os,
};

#[cfg(feature = "cv")]
use crate::cv;

#[cfg(feature = "cat")]
use crate::cat;

#[doc(alias = "CMPixelFormatType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct PixelFormat(pub FourCharCode);

impl PixelFormat {
    #[doc(alias = "kCMPixelFormat_32ARGB")]
    pub const _32_ARGB: Self = Self(32);
    #[doc(alias = "kCMPixelFormat_32BGRA")]
    pub const _32_BGRA: Self = Self::from_be_bytes(b"BGRA");
    #[doc(alias = "kCMPixelFormat_24RGB")]
    pub const _24_RGB: Self = Self(24);

    /// Y'CbCr 8-bit 4:2:2 ordered Cb Y'0 Cr Y'1
    #[doc(alias = "kCMPixelFormat_422YpCbCr8")]
    pub const _422_YP_CB_CR_8: Self = Self::from_be_bytes(b"2vuy");

    /// Y'CbCr 8-bit 4:2:2 ordered Y'0 Cb Y'1 Cr
    #[doc(alias = "kCMPixelFormat_422YpCbCr8_yuvs")]
    pub const _422_YP_CB_CR_8_YUVS: Self = Self::from_be_bytes(b"yuvs");

    /// Component Y'CbCr 8-bit 4:4:4
    #[doc(alias = "kCMPixelFormat_444YpCbCr8")]
    pub const _444_YP_CB_CR_8: Self = Self::from_be_bytes(b"v308");

    /// Component Y'CbCrA 8-bit 4:4:4:4
    #[doc(alias = "kCMPixelFormat_4444YpCbCrA8")]
    pub const _4444_YP_CB_CR_A_8: Self = Self::from_be_bytes(b"v408");

    /// Component Y'CbCr 10,12,14,16-bit 4:2:2
    #[doc(alias = "kCMPixelFormat_422YpCbCr16")]
    pub const _422_YP_CB_CR_16: Self = Self::from_be_bytes(b"v216");

    /// Y'CbCr 10-bit 4:2:2
    #[doc(alias = "kCMPixelFormat_422YpCbCr10")]
    pub const _422_YP_CB_CR_10: Self = Self::from_be_bytes(b"v210");
    /// Y'CbCr 10-bit 4:4:4
    #[doc(alias = "kCMPixelFormat_444YpCbCr10")]
    pub const _444_YP_CB_CR_10: Self = Self::from_be_bytes(b"v410");

    const fn from_be_bytes(bytes: &[u8; 4]) -> Self {
        Self(FourCharCode::from_be_bytes(*bytes))
    }
}

#[doc(alias = "CMVideoDimensions")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct VideoDimensions {
    pub width: i32,
    pub height: i32,
}

#[doc(alias = "CMMediaType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub struct MediaType(pub FourCharCode);

impl MediaType {
    #[doc(alias = "kCMMediaType_Video")]
    pub const VIDEO: Self = Self::from_be_bytes(b"vide");

    #[doc(alias = "kCMMediaType_Audio")]
    pub const AUDIO: Self = Self::from_be_bytes(b"soun");

    #[doc(alias = "kCMMediaType_Muxed")]
    pub const MUXED: Self = Self::from_be_bytes(b"muxx");

    #[doc(alias = "kCMMediaType_Text")]
    pub const TEXT: Self = Self::from_be_bytes(b"text");

    #[doc(alias = "kCMMediaType_ClosedCaption")]
    pub const CLOSED_CAPTION: Self = Self::from_be_bytes(b"clcp");

    #[doc(alias = "kCMMediaType_Subtitle")]
    pub const SUBTITLE: Self = Self::from_be_bytes(b"sbtl");

    #[doc(alias = "kCMMediaType_TimeCode")]
    pub const TIME_CODE: Self = Self::from_be_bytes(b"tmcd");

    #[doc(alias = "kCMMediaType_Metadata")]
    pub const METADATA: Self = Self::from_be_bytes(b"meta");

    #[doc(alias = "kCMMediaType_AuxiliaryPicture")]
    pub const AUXILIARY_PICTURE: Self = Self::from_be_bytes(b"auxv");

    #[doc(alias = "kCMMediaType_TaggedBufferGroup")]
    pub const TAGGED_BUFFER_GROUP: Self = Self::from_be_bytes(b"tbgr");

    const fn from_be_bytes(bytes: &[u8; 4]) -> Self {
        Self(FourCharCode::from_be_bytes(*bytes))
    }
}

#[doc(alias = "CMVideoCodecType")]
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(transparent)]
pub struct VideoCodec(FourCharCode);

impl VideoCodec {
    #[doc(alias = "kCMVideoCodecType_422YpCbCr8")]
    pub const _422_YP_CB_CR_8: Self = Self(PixelFormat::_422_YP_CB_CR_8.0);

    #[doc(alias = "kCMVideoCodecType_JPEG")]
    pub const JPEG: Self = Self::from_be_bytes(b"jpeg");

    #[doc(alias = "kCMVideoCodecType_H264")]
    pub const H264: Self = Self::from_be_bytes(b"avc1");

    #[doc(alias = "kCMVideoCodecType_HEVC")]
    pub const HEVC: Self = Self::from_be_bytes(b"hvc1");

    #[doc(alias = "kCMVideoCodecType_HEVCWithAlpha")]
    pub const HEVC_WITH_ALPHA: Self = Self::from_be_bytes(b"muxa");

    #[doc(alias = "kCMVideoCodecType_DolbyVisionHEVC")]
    pub const DOLBY_VISION_HEVC: Self = Self::from_be_bytes(b"dvh1");

    #[doc(alias = "kCMVideoCodecType_VP9")]
    pub const VP9: Self = Self::from_be_bytes(b"vp09");

    #[doc(alias = "kCMVideoCodecType_AppleProRes4444XQ")]
    pub const APPLE_PRO_RES4444_XQ: Self = Self::from_be_bytes(b"ap4x");

    #[doc(alias = "kCMVideoCodecType_AppleProRes4444")]
    pub const APPLE_PRO_RES4444: Self = Self::from_be_bytes(b"ap4h");

    #[doc(alias = "kCMVideoCodecType_AppleProRes422HQ")]
    pub const APPLE_PRO_RES422_HQ: Self = Self::from_be_bytes(b"apch");

    #[doc(alias = "kCMVideoCodecType_AppleProRes422")]
    pub const APPLE_PRO_RES422: Self = Self::from_be_bytes(b"apcn");

    #[doc(alias = "kCMVideoCodecType_AppleProRes422LT")]
    pub const APPLE_PRO_RES422_LT: Self = Self::from_be_bytes(b"apcs");

    #[doc(alias = "kCMVideoCodecType_AppleProRes422Proxy")]
    pub const APPLE_PRO_RES422_PROXY: Self = Self::from_be_bytes(b"apco");

    #[doc(alias = "kCMVideoCodecType_AppleProResRAW")]
    pub const APPLE_PRO_RES_RAW: Self = Self::from_be_bytes(b"aprn");

    #[doc(alias = "kCMVideoCodecType_AppleProResRAWHQ")]
    pub const APPLE_PRO_RES_RAWHQ: Self = Self::from_be_bytes(b"aprh");

    #[doc(alias = "kCMVideoCodecType_DisparityHEVC")]
    pub const DISPARITY_HEVC: Self = Self::from_be_bytes(b"dish");

    #[doc(alias = "kCMVideoCodecType_DepthHEVC")]
    pub const DEPTH_HEVC: Self = Self::from_be_bytes(b"deph");

    #[doc(alias = "kCMVideoCodecType_AV1")]
    pub const AV1: Self = Self::from_be_bytes(b"av01");

    const fn from_be_bytes(bytes: &[u8; 4]) -> Self {
        Self(FourCharCode::from_be_bytes(*bytes))
    }
}

define_cf_type!(
    #[doc(alias = "CMFormatDescriptionRef")]
    FormatDesc(cf::Type)
);

unsafe impl Send for FormatDesc {}

impl FormatDesc {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CMFormatDescriptionGetTypeID() }
    }

    #[inline]
    pub fn media_type(&self) -> MediaType {
        unsafe { CMFormatDescriptionGetMediaType(self) }
    }

    #[inline]
    pub fn media_sub_type(&self) -> FourCharCode {
        unsafe { CMFormatDescriptionGetMediaSubType(self) }
    }

    #[inline]
    pub fn exts(&self) -> Option<&cf::DictionaryOf<cf::String, cf::Plist>> {
        unsafe { CMFormatDescriptionGetExtensions(self) }
    }

    pub fn ext<'a>(&'a self, key: &FormatDescExtKey) -> Option<&'a cf::Plist> {
        unsafe { CMFormatDescriptionGetExtension(self, key) }
    }

    pub fn original_compression_session_settings(
        &self,
    ) -> Option<&cf::DictionaryOf<cf::String, cf::Plist>> {
        unsafe {
            let key = FormatDescExtKey::original_compression_settings();
            transmute(self.ext(key))
        }
    }

    pub fn ext_atoms(&self) -> Option<&cf::DictionaryOf<cf::String, cf::Plist>> {
        unsafe {
            let key = FormatDescExtKey::sample_desc_ext_atoms();
            transmute(self.ext(key))
        }
    }

    fn video_cfg(&self, key: &cf::String) -> Option<Vec<u8>> {
        let Some(dict) = self.ext_atoms() else {
            return None;
        };
        let Some(value) = dict.get(key) else {
            return None;
        };

        let Some(data) = value.try_as_data() else {
            return None;
        };

        let mut res = vec![0u8; data.len()];
        data.copy_bytes(&mut res);
        Some(res)
    }

    pub fn avcc(&self) -> Option<Vec<u8>> {
        self.video_cfg(cf::str!(c"avcC"))
    }

    pub fn hvcc(&self) -> Option<Vec<u8>> {
        self.video_cfg(cf::str!(c"hvcC"))
    }

    pub fn verbatim_sample_desc(&self) -> Option<&cf::Data> {
        unsafe {
            let key = FormatDescExtKey::verbatim_sample_desc();
            transmute(self.ext(key))
        }
    }

    pub fn verbatim_iso_sample_entry(&self) -> Option<&cf::Data> {
        unsafe {
            let key = FormatDescExtKey::verbatim_iso_sample_entry();
            transmute(self.ext(key))
        }
    }

    pub fn create_in(
        media_type: MediaType,
        media_sub_type: FourCharCode,
        extensions: Option<&cf::DictionaryOf<FormatDescExtKey, cf::Type>>,
        format_description_out: *mut Option<arc::R<FormatDesc>>,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result {
        unsafe {
            CMFormatDescriptionCreate(
                allocator,
                media_type,
                media_sub_type,
                extensions,
                format_description_out,
            )
            .result()
        }
    }

    /// ```
    /// use cidre::{cm, mac_types::FourCharCode};
    ///
    /// let desc = cm::FormatDesc::new(
    ///     cm::MediaType::VIDEO,
    ///     FourCharCode::from_be_bytes(*b"avc1"),
    ///     None
    /// ).unwrap();
    /// ```
    pub fn new(
        media_type: MediaType,
        media_sub_type: FourCharCode,
        extensions: Option<&cf::DictionaryOf<FormatDescExtKey, cf::Type>>,
    ) -> os::Result<arc::R<Self>> {
        unsafe {
            os::result_unchecked(|val| {
                Self::create_in(media_type, media_sub_type, extensions, val, None)
            })
        }
    }
}

pub type VideoFormatDesc = FormatDesc;

impl VideoFormatDesc {
    /// ```
    /// use cidre::cm;
    ///
    /// let desc = cm::VideoFormatDesc::video(cm::VideoCodec::H264, 1920, 1080, None).unwrap();
    /// ```
    pub fn video(
        codec_type: VideoCodec,
        width: i32,
        height: i32,
        extensions: Option<&cf::DictionaryOf<FormatDescExtKey, cf::Type>>,
    ) -> os::Result<arc::R<Self>> {
        unsafe {
            os::result_unchecked(|res| {
                Self::create_video_in(codec_type, width, height, extensions, res, None)
            })
        }
    }

    #[doc(alias = "CMVideoFormatDescriptionCreate")]
    pub fn create_video_in(
        codec_type: VideoCodec,
        width: i32,
        height: i32,
        extensions: Option<&cf::DictionaryOf<FormatDescExtKey, cf::Type>>,
        format_description_out: *mut Option<arc::R<VideoFormatDesc>>,
        allocator: Option<&Allocator>,
    ) -> os::Result {
        unsafe {
            CMVideoFormatDescriptionCreate(
                allocator,
                codec_type,
                width,
                height,
                extensions,
                format_description_out,
            )
            .result()
        }
    }

    /// Returns the dimensions (in encoded pixels)
    ///
    /// This does not take into account pixel aspect ratio or clean aperture tags.
    #[doc(alias = "CMVideoFormatDescriptionGetDimensions")]
    #[inline]
    pub fn dimensions(&self) -> VideoDimensions {
        unsafe { CMVideoFormatDescriptionGetDimensions(self) }
    }

    #[cfg(feature = "cv")]
    #[inline]
    pub fn matches_image_buf(&self, image_buffer: &cv::ImageBuf) -> bool {
        unsafe { CMVideoFormatDescriptionMatchesImageBuffer(self, image_buffer) }
    }

    #[doc(alias = "CMVideoFormatDescriptionCreateForImageBuffer")]
    #[cfg(feature = "cv")]
    #[inline]
    pub fn with_image_buf_in(
        image_buffer: &cv::ImageBuf,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<arc::R<VideoFormatDesc>> {
        unsafe {
            os::result_unchecked(|res| {
                CMVideoFormatDescriptionCreateForImageBuffer(allocator, image_buffer, res)
            })
        }
    }

    #[doc(alias = "CMVideoFormatDescriptionCreateForImageBuffer")]
    #[cfg(feature = "cv")]
    #[inline]
    pub fn with_image_buf(image_buffer: &cv::ImageBuf) -> os::Result<arc::R<VideoFormatDesc>> {
        Self::with_image_buf_in(image_buffer, None)
    }

    #[doc(alias = "CMVideoFormatDescriptionCreateFromH264ParameterSets")]
    #[inline]
    pub fn with_h264_param_sets<const N: usize>(
        pointers: &[*const u8; N],
        sizes: &[usize; N],
        nal_unit_header_length: i32,
    ) -> os::Result<arc::R<VideoFormatDesc>> {
        Self::with_h264_param_sets_in(pointers, sizes, nal_unit_header_length, None)
    }

    #[doc(alias = "CMVideoFormatDescriptionCreateFromH264ParameterSets")]
    #[inline]
    pub fn with_h264_param_sets_in<const N: usize>(
        pointers: &[*const u8; N],
        sizes: &[usize; N],
        nal_unit_header_length: i32,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<arc::R<VideoFormatDesc>> {
        unsafe {
            os::result_unchecked(|res| {
                CMVideoFormatDescriptionCreateFromH264ParameterSets(
                    allocator,
                    N,
                    pointers.as_ptr(),
                    sizes.as_ptr(),
                    nal_unit_header_length,
                    res,
                )
            })
        }
    }

    #[doc(alias = "CMVideoFormatDescriptionCreateFromHEVCParameterSets")]
    #[inline]
    pub fn with_hevc_param_sets(
        count: usize,
        pointers: &[*const u8],
        sizes: &[usize],
        nal_unit_header_length: i32,
        extensions: Option<&cf::DictionaryOf<FormatDescExtKey, cf::Type>>,
    ) -> os::Result<arc::R<VideoFormatDesc>> {
        Self::with_hevc_param_sets_in(
            count,
            pointers,
            sizes,
            nal_unit_header_length,
            extensions,
            None,
        )
    }

    #[doc(alias = "CMVideoFormatDescriptionCreateFromHEVCParameterSets")]
    #[inline]
    pub fn with_hevc_param_sets_in(
        count: usize,
        pointers: &[*const u8],
        sizes: &[usize],
        nal_unit_header_length: i32,
        extensions: Option<&cf::DictionaryOf<FormatDescExtKey, cf::Type>>,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result<arc::R<VideoFormatDesc>> {
        unsafe {
            os::result_unchecked(|res| {
                CMVideoFormatDescriptionCreateFromHEVCParameterSets(
                    allocator,
                    count,
                    pointers.as_ptr(),
                    sizes.as_ptr(),
                    nal_unit_header_length,
                    extensions,
                    res,
                )
            })
        }
    }

    #[doc(alias = "CMVideoFormatDescriptionGetH264ParameterSetAtIndex")]
    #[inline]
    pub fn h264_params_count_and_header_len(&self) -> os::Result<(usize, i32)> {
        let mut parameters_count_out = 0;
        let mut nal_unit_header_length_out = 0;

        unsafe {
            CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
                self,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut parameters_count_out,
                &mut nal_unit_header_length_out,
            )
            .result()?;
        }
        Ok((parameters_count_out, nal_unit_header_length_out))
    }

    #[doc(alias = "CMVideoFormatDescriptionGetHEVCParameterSetAtIndex")]
    #[inline]
    pub unsafe fn hevc_param_set_at_index(
        &self,
        parameter_set_index: usize,
        parameter_set_pointer_out: *mut *const u8,
        parameter_set_size_out: *mut usize,
        parameter_set_count_out: *mut usize,
        nal_unit_header_length_out: *mut i32,
    ) -> os::Result {
        unsafe {
            CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(
                self,
                parameter_set_index,
                parameter_set_pointer_out,
                parameter_set_size_out,
                parameter_set_count_out,
                nal_unit_header_length_out,
            )
            .result()
        }
    }

    #[doc(alias = "CMVideoFormatDescriptionGetHEVCParameterSetAtIndex")]
    #[inline]
    pub fn hevc_params_count_and_header_len(&self) -> os::Result<(usize, i32)> {
        let mut parameters_count_out = 0;
        let mut nal_unit_header_length_out = 0;
        unsafe {
            CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(
                self,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut parameters_count_out,
                &mut nal_unit_header_length_out,
            )
            .result()?;
        }

        Ok((parameters_count_out, nal_unit_header_length_out))
    }

    #[doc(alias = "CMVideoFormatDescriptionGetH264ParameterSetAtIndex")]
    #[inline]
    pub fn h264_param_set_at(&self, index: usize) -> os::Result<&[u8]> {
        let mut size = 0;
        let mut bytes = std::ptr::null();
        unsafe {
            CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
                self,
                index,
                &mut bytes,
                &mut size,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
            .result()?;
            let slice = std::ptr::slice_from_raw_parts(bytes, size);
            Ok(&*slice)
        }
    }

    #[doc(alias = "CMVideoFormatDescriptionGetHEVCParameterSetAtIndex")]
    #[inline]
    pub fn hevc_param_set_at(&self, index: usize) -> os::Result<&[u8]> {
        let mut size = 0;
        let mut bytes = std::ptr::null();
        unsafe {
            CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(
                self,
                index,
                &mut bytes,
                &mut size,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
            .result()?;
            let slice = std::ptr::slice_from_raw_parts(bytes, size);
            Ok(&*slice)
        }
    }

    /// Returns an array of the keys that are used both as [`cm::VideoFormatDesc`] extensions
    /// and [`cv::ImageBuf`] attachments and attributes.
    #[doc(alias = "CMVideoFormatDescriptionGetExtensionKeysCommonWithImageBuffers")]
    pub fn common_image_buf_ext_keys() -> &'static cf::ArrayOf<FormatDescExtKey> {
        unsafe { CMVideoFormatDescriptionGetExtensionKeysCommonWithImageBuffers() }
    }
}

pub type AudioFormatDesc = FormatDesc;

#[cfg(feature = "cat")]
impl AudioFormatDesc {
    pub fn with_asbd(asbd: &cat::audio::StreamBasicDesc) -> os::Result<arc::R<Self>> {
        unsafe {
            os::result_unchecked(|res| Self::audio_in(asbd, 0, None, 0, None, None, res, None))
        }
    }

    pub fn audio_in(
        asbd: &cat::audio::StreamBasicDesc,
        layout_size: usize,
        layout: Option<&cat::AudioChannelLayout<1>>,
        magic_cookie_size: usize,
        magic_cookie: Option<&c_void>,
        extensions: Option<&cf::DictionaryOf<FormatDescExtKey, cf::Type>>,
        format_description_out: *mut Option<arc::R<Self>>,
        allocator: Option<&cf::Allocator>,
    ) -> os::Result {
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
            .result()
        }
    }

    /// Returns a read-only pointer to the audio stream description in an audio format description.
    ///
    /// This API is specific to audio format descriptions, and returns `None` if used with a non-audio format descriptions.
    pub fn stream_basic_desc(&self) -> Option<&cat::audio::StreamBasicDesc> {
        unsafe { CMAudioFormatDescriptionGetStreamBasicDescription(self) }
    }
}

define_cf_type!(
    #[doc(alias = "CMFormatDescription.Extensions.Key")]
    #[doc(alias = "CMFormatDescriptionExtension")]
    FormatDescExtKey(cf::String)
);

impl FormatDescExtKey {
    /// [`cf::String`]
    #[doc(alias = "kCMFormatDescriptionExtension_FormatName")]
    #[inline]
    pub fn format_name() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_FormatName }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_Depth")]
    #[inline]
    pub fn depth() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_Depth }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_CleanAperture")]
    #[inline]
    pub fn clean_aperture() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_CleanAperture }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_FieldCount")]
    #[inline]
    pub fn field_count() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_FieldCount }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_FieldDetail")]
    #[inline]
    pub fn field_detail() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_FieldDetail }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_PixelAspectRatio")]
    #[inline]
    pub fn pixel_aspect_ratio() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_PixelAspectRatio }
    }
    /// This extension contains a media-type-specific dictionary of settings used to produce a compressed media buffer.
    ///
    /// This extension is valid for format descriptions of all media types, but the contents of the dictionary are defined
    /// in a media-specific way.  The dictionary and its contents are valid property list objects. This means that
    /// dictionary keys are all [`cf::String`]s, and the values are all either [`cf::Number`], [`cf::String`], [`cf::Boolean`],
    /// [`cf::Array`], [`cf::Dictionary`], [`cf::Date`], or [`cf::Data`].
    ///
    /// cf::Dictionary
    #[doc(alias = "kCMFormatDescriptionExtension_OriginalCompressionSettings")]
    #[inline]
    pub fn original_compression_settings() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_OriginalCompressionSettings }
    }

    /// Sample description extension atoms that were not translated into other entries in the extensions dictionary.
    ///
    /// This key is used by sample description bridges to hold sample description
    /// extension atoms that they do not recognize.
    /// The extension is a [`cf::Dictionary`] mapping [`cf::String`]s of the four-char-code atom types
    /// to either [`cf::Data`]s containing the atom payload or (to represent multiple atoms of a
    /// specific type) to [`cf::Array`]s of [`cf::Data`] containing those payloads.
    ///
    /// cf::Dictionary of cf::String (four-char-code, atom type) -> ( cf::Data (atom payload) or cf::Array of cf::Data (atom payload) )
    #[doc(alias = "kCMFormatDescriptionExtension_SampleDescriptionExtensionAtoms")]
    #[inline]
    pub fn sample_desc_ext_atoms() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_SampleDescriptionExtensionAtoms }
    }

    /// Preserves the original SampleDescription data.
    ///
    /// This extension is used to ensure that roundtrips from sample descriptions
    /// to cm::FormatDescriptions back to sample descriptions preserve the exact original
    /// sample descriptions.
    ///
    /// IMPORTANT: If you make a modified clone of a cm::FormatDescription, you must
    /// delete this extension from the clone, or your modifications could be lost.
    ///
    /// cf::Data
    #[doc(alias = "kCMFormatDescriptionExtension_VerbatimSampleDescription")]
    #[inline]
    pub fn verbatim_sample_desc() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_VerbatimSampleDescription }
    }

    /// Preserves the original ISOSampleEntry data.
    ///
    /// This extension is used to ensure that roundtrips from ISO Sample Entry (ie. AudioSampleEntry or VisualSampleEntry)
    /// to cm::FormatDescriptions back to ISO Sample Entry preserve the exact original
    /// sample descriptions.
    ///
    /// IMPORTANT: If you make a modified clone of a [`cm::FormatDesc`], you must
    /// delete this extension from the clone, or your modifications could be lost.
    ///
    /// [`cf::Data`]
    #[doc(alias = "kCMFormatDescriptionExtension_VerbatimISOSampleEntry")]
    #[inline]
    pub fn verbatim_iso_sample_entry() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_VerbatimISOSampleEntry }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_ColorPrimaries")]
    #[inline]
    pub fn color_primaries() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_ColorPrimaries }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_TransferFunction")]
    #[inline]
    pub fn transfer_fn() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_TransferFunction }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_GammaLevel")]
    #[inline]
    pub fn gamma_level() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_GammaLevel }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_YCbCrMatrix")]
    #[inline]
    pub fn ycbcr_matrix() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_YCbCrMatrix }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_FullRangeVideo")]
    #[inline]
    pub fn full_range_video() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_FullRangeVideo }
    }

    /// cf::Data
    #[doc(alias = "kCMFormatDescriptionExtension_ICCProfile")]
    #[inline]
    pub fn icc_profile() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_ICCProfile }
    }

    /// cf::Number
    #[doc(alias = "kCMFormatDescriptionExtension_BytesPerRow")]
    #[inline]
    pub fn bytes_per_row() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_BytesPerRow }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_ChromaLocationTopField")]
    #[inline]
    pub fn chroma_location_top_field() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_ChromaLocationTopField }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_ProtectedContentOriginalFormat")]
    #[inline]
    pub fn protected_content_oridinal_format() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_ProtectedContentOriginalFormat }
    }

    /// cf::Number
    #[doc(alias = "kCMFormatDescriptionExtension_TemporalQuality")]
    #[inline]
    pub fn temporal_quality() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_TemporalQuality }
    }

    /// cf::Number
    #[doc(alias = "kCMFormatDescriptionExtension_SpatialQuality")]
    #[inline]
    pub fn spatial_quality() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_SpatialQuality }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_VerbatimImageDescription")]
    #[inline]
    pub fn verbatim_image_desc() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_VerbatimImageDescription }
    }

    /// [`cf::Number`]
    #[doc(alias = "kCMFormatDescriptionExtension_Version")]
    #[inline]
    pub fn version() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_Version }
    }

    /// [`cf::Number`]
    #[doc(alias = "kCMFormatDescriptionExtension_RevisionLevel")]
    #[inline]
    pub fn revision_level() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_RevisionLevel }
    }

    /// [`cf::String`] of fourCC
    #[doc(alias = "kCMFormatDescriptionExtension_Vendor")]
    #[inline]
    pub fn vendor() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_Vendor }
    }

    /// [`cf::Data`] (24 bytes); big-endian structure; same as kCVImageBufferMasteringDisplayColorVolumeKey;
    /// matches payload of ISO/IEC 23008-2:2015(E), D.2.28 Mastering display colour volume SEI message
    #[doc(alias = "kCMFormatDescriptionExtension_MasteringDisplayColorVolume")]
    #[inline]
    pub fn mastering_display_color_volume() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_MasteringDisplayColorVolume }
    }

    /// [`cf::Data`] (4 bytes); big-endian structure; same as kCVImageBufferContentLightLevelInfoKey
    #[doc(alias = "kCMFormatDescriptionExtension_ContentLightLevelInfo")]
    #[inline]
    pub fn content_light_level_info() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_ContentLightLevelInfo }
    }

    /// [`cf::Data`]; big-endian structure; same as kCVImageBufferContentColorVolumeKey
    /// (to be added); matches payload of ITU-T-H.265:11/2019, D.2.40 Content Colour
    /// Volume SEI message
    #[doc(alias = "kCMFormatDescriptionExtension_ContentColorVolume")]
    #[inline]
    pub fn content_color_volume() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_ContentColorVolume }
    }

    /// kCMFormatDescriptionTransferFunction_ITU_R_2100_HLG when used); corresponds to D.2.38 Alternative Transfer Characteristics SEI message
    #[doc(alias = "kCMFormatDescriptionExtension_AlternativeTransferCharacteristics")]
    #[inline]
    pub fn alternate_transfer_characterisitcs() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_AlternativeTransferCharacteristics }
    }

    /// [`cf::String`] (Auxiliary type URN)
    #[doc(alias = "kCMFormatDescriptionExtension_AuxiliaryTypeInfo")]
    #[inline]
    pub fn auxiliary_type_info() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_AuxiliaryTypeInfo }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_AlphaChannelMode")]
    #[inline]
    pub fn alpha_channel_mode() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_AlphaChannelMode }
    }

    #[doc(alias = "kCMFormatDescriptionExtension_ContainsAlphaChannel")]
    #[inline]
    pub fn contains_alpha_channel() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_ContainsAlphaChannel }
    }

    /// [`cf::Number`] (such as 8, 10, 12, 16, etc). Bit-depth per component
    /// if there are components with different bit depths this should be the deepest.
    /// Do not rely on this extension always being present, as it often isn't.
    #[doc(alias = "kCMFormatDescriptionExtension_BitsPerComponent")]
    #[inline]
    pub fn bits_per_component() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_BitsPerComponent }
    }

    /// [`cf::Number`]; horizontal field of view in thousandths of a degree
    /// (i.e., 123456 is 123.456 degrees).
    #[doc(alias = "kCMFormatDescriptionExtension_HorizontalFieldOfView")]
    #[inline]
    pub fn horizontal_field_of_view() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_HorizontalFieldOfView }
    }

    /// Indicates that the transfer function or gamma of the content is a log format and identifies the specific log curve.
    ///
    /// The value is a CFString holding fully specified reverse DNS identifier.
    /// Content captured in Apple Log will have this key set to kCMFormatDescriptionLogTransferFunction_AppleLog.
    #[doc(alias = "kCMFormatDescriptionExtension_LogTransferFunction")]
    #[api::available(macos = 14.2, ios = 17.2, tvos = 17.2, watchos = 10.2, visionos = 1.1)]
    pub fn log_transfer_fn() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_LogTransferFunction }
    }
}

define_cf_type!(LogTransferFn(cf::String));

impl LogTransferFn {
    #[doc(alias = "kCMFormatDescriptionLogTransferFunction_AppleLog")]
    #[api::available(macos = 14.2, ios = 17.2, tvos = 17.2, watchos = 10.2, visionos = 1.1)]
    pub fn apple_log() -> &'static Self {
        unsafe { kCMFormatDescriptionLogTransferFunction_AppleLog }
    }
}

#[link(name = "CoreMedia", kind = "framework")]
#[api::weak]
unsafe extern "C-unwind" {
    static kCMFormatDescriptionExtension_FormatName: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_Depth: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_CleanAperture: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_FieldCount: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_FieldDetail: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_PixelAspectRatio: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_OriginalCompressionSettings: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_SampleDescriptionExtensionAtoms: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_VerbatimSampleDescription: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_VerbatimISOSampleEntry: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_ColorPrimaries: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_TransferFunction: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_GammaLevel: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_YCbCrMatrix: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_FullRangeVideo: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_ICCProfile: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_BytesPerRow: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_ChromaLocationTopField: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_ProtectedContentOriginalFormat: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_TemporalQuality: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_SpatialQuality: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_VerbatimImageDescription: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_Version: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_RevisionLevel: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_Vendor: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_MasteringDisplayColorVolume: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_ContentLightLevelInfo: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_ContentColorVolume: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_AlternativeTransferCharacteristics:
        &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_AuxiliaryTypeInfo: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_AlphaChannelMode: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_ContainsAlphaChannel: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_BitsPerComponent: &'static FormatDescExtKey;
    static kCMFormatDescriptionExtension_HorizontalFieldOfView: &'static FormatDescExtKey;

    #[api::available(macos = 14.2, ios = 17.2, tvos = 17.2, watchos = 10.2, visionos = 1.1)]
    static kCMFormatDescriptionExtension_LogTransferFunction: &'static FormatDescExtKey;

    #[api::available(macos = 14.2, ios = 17.2, tvos = 17.2, watchos = 10.2, visionos = 1.1)]
    static kCMFormatDescriptionLogTransferFunction_AppleLog: &'static LogTransferFn;

    fn CMFormatDescriptionGetTypeID() -> cf::TypeId;
    fn CMFormatDescriptionGetMediaType(desc: &FormatDesc) -> MediaType;

    fn CMVideoFormatDescriptionCreate(
        allocator: Option<&cf::Allocator>,
        codec_type: VideoCodec,
        width: i32,
        height: i32,
        extensions: Option<&cf::DictionaryOf<FormatDescExtKey, cf::Type>>,
        format_description_out: *mut Option<arc::R<VideoFormatDesc>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionGetDimensions(video_desc: &VideoFormatDesc) -> VideoDimensions;

    #[cfg(feature = "cv")]
    fn CMVideoFormatDescriptionMatchesImageBuffer(
        video_desc: &VideoFormatDesc,
        image_buffer: &cv::ImageBuf,
    ) -> bool;

    fn CMFormatDescriptionGetMediaSubType(desc: &FormatDesc) -> FourCharCode;

    fn CMFormatDescriptionGetExtensions(
        desc: &FormatDesc,
    ) -> Option<&cf::DictionaryOf<cf::String, cf::Plist>>;

    fn CMFormatDescriptionGetExtension<'a>(
        desc: &'a FormatDesc,
        extension_key: &FormatDescExtKey,
    ) -> Option<&'a cf::Plist>;

    #[cfg(feature = "cat")]
    fn CMAudioFormatDescriptionCreate(
        allocator: Option<&cf::Allocator>,
        asbd: &cat::audio::StreamBasicDesc,
        layout_size: usize,
        layout: Option<&cat::AudioChannelLayout<1>>,
        magic_cookie_size: usize,
        magic_cookie: Option<&c_void>,
        extensions: Option<&cf::DictionaryOf<FormatDescExtKey, cf::Type>>,
        format_description_out: *mut Option<arc::R<AudioFormatDesc>>,
    ) -> os::Status;

    #[cfg(feature = "cat")]
    fn CMAudioFormatDescriptionGetStreamBasicDescription(
        desc: &AudioFormatDesc,
    ) -> Option<&cat::audio::StreamBasicDesc>;

    fn CMFormatDescriptionCreate(
        allocator: Option<&cf::Allocator>,
        media_type: MediaType,
        media_sub_type: FourCharCode,
        extensions: Option<&cf::DictionaryOf<FormatDescExtKey, cf::Type>>,
        format_description_out: *mut Option<arc::R<FormatDesc>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionCreateForImageBuffer(
        allocator: Option<&cf::Allocator>,
        image_buffer: &cv::ImageBuf,
        format_description: *mut Option<arc::R<FormatDesc>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionCreateFromH264ParameterSets(
        allocator: Option<&cf::Allocator>,
        parameter_set_count: usize,
        parameter_set_pointers: *const *const u8,
        parameter_set_sizes: *const usize,
        nal_unit_header_length: i32,
        format_description: *mut Option<arc::R<FormatDesc>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionCreateFromHEVCParameterSets(
        allocator: Option<&cf::Allocator>,
        parameter_set_count: usize,
        parameter_set_pointers: *const *const u8,
        parameter_set_sizes: *const usize,
        nal_unit_header_length: i32,
        extensions: Option<&cf::DictionaryOf<FormatDescExtKey, cf::Type>>,
        format_description: *mut Option<arc::R<FormatDesc>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
        video_desc: &VideoFormatDesc,
        parameter_set_index: usize,
        parameter_set_pointer_out: *mut *const u8,
        parameter_set_size_out: *mut usize,
        parameter_set_count_out: *mut usize,
        nal_unit_header_length_out: *mut i32,
    ) -> os::Status;

    fn CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(
        video_desc: &VideoFormatDesc,
        parameter_set_index: usize,
        parameter_set_pointer_out: *mut *const u8,
        parameter_set_size_out: *mut usize,
        parameter_set_count_out: *mut usize,
        nal_unit_header_length_out: *mut i32,
    ) -> os::Status;

    fn CMVideoFormatDescriptionGetExtensionKeysCommonWithImageBuffers()
    -> &'static cf::ArrayOf<FormatDescExtKey>;
}

#[cfg(test)]
mod tests {
    use crate::cm;

    #[test]
    fn basics() {
        let keys = cm::VideoFormatDesc::common_image_buf_ext_keys();
        eprintln!("{keys:?}");
        assert!(!keys.is_empty());
    }
}
