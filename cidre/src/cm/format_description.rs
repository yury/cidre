use std::{ffi::c_void, mem::transmute};

use crate::{
    arc, cat,
    cf::{self, Allocator},
    cv, define_cf_type, os, FourCharCode,
};

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct VideoDimensions {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

    pub fn extensions(&self) -> Option<&cf::DictionaryOf<cf::String, cf::Type>> {
        unsafe { CMFormatDescriptionGetExtensions(self) }
    }

    pub fn extension<'a>(
        &'a self,
        key: &FormatDescriptionExtensionKey,
    ) -> Option<&'a cf::PropertyList> {
        unsafe { CMFormatDescriptionGetExtension(self, key) }
    }

    pub fn original_compression_session_settings(
        &self,
    ) -> Option<&cf::DictionaryOf<cf::String, cf::PropertyList>> {
        unsafe {
            let key = FormatDescriptionExtensionKey::original_compression_settings();
            transmute(self.extension(key))
        }
    }

    pub fn extension_atoms(&self) -> Option<&cf::DictionaryOf<cf::String, cf::PropertyList>> {
        unsafe {
            let key = FormatDescriptionExtensionKey::sample_description_extension_atoms();
            transmute(self.extension(key))
        }
    }

    fn video_configuration(&self, key: &str) -> Option<Vec<u8>> {
        let Some(dict) = self.extension_atoms() else {
            return None;
        };
        let key = cf::String::from_str(key);
        let Some(value) = dict.get(&key) else {
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
        self.video_configuration("avcC")
    }

    pub fn hvcc(&self) -> Option<Vec<u8>> {
        self.video_configuration("hvcC")
    }

    pub fn verbatim_sample_description(&self) -> Option<&cf::Data> {
        unsafe {
            let key = FormatDescriptionExtensionKey::verbatim_sample_description();
            transmute(self.extension(key))
        }
    }

    pub fn verbatim_iso_sample_entry(&self) -> Option<&cf::Data> {
        unsafe {
            let key = FormatDescriptionExtensionKey::verbatim_iso_sample_entry();
            transmute(self.extension(key))
        }
    }

    pub fn create(
        allocator: Option<&cf::Allocator>,
        media_type: MediaType,
        media_sub_type: FourCharCode,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<arc::R<FormatDescription>>,
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
    /// let desc = cm::FormatDescription::new(
    ///     cm::MediaType::VIDEO,
    ///     FourCharCode::from_be_bytes(*b"avc1"),
    ///     None
    /// ).unwrap();
    /// ```
    pub fn new(
        media_type: MediaType,
        media_sub_type: FourCharCode,
        extensions: Option<&cf::Dictionary>,
    ) -> Result<arc::R<Self>, os::Status> {
        let mut format_desc = None;
        let res = Self::create(
            None,
            media_type,
            media_sub_type,
            extensions,
            &mut format_desc,
        );
        unsafe { res.to_result_unchecked(format_desc) }
    }
}

pub type VideoFormatDescription = FormatDescription;

impl VideoFormatDescription {
    /// ```
    /// use cidre::cm;
    ///
    /// let desc = cm::VideoFormatDescription::video(cm::VideoCodecType::H264, 1920, 1080, None).unwrap();
    /// ```
    pub fn video(
        codec_type: VideoCodecType,
        width: i32,
        height: i32,
        extensions: Option<&cf::Dictionary>,
    ) -> Result<arc::R<Self>, os::Status> {
        let mut format_desc = None;
        let res = Self::create_video(
            None,
            codec_type,
            width,
            height,
            extensions,
            &mut format_desc,
        );
        unsafe { res.to_result_unchecked(format_desc) }
    }

    pub fn create_video(
        allocator: Option<&Allocator>,
        codec_type: VideoCodecType,
        width: i32,
        height: i32,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<arc::R<VideoFormatDescription>>,
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

    #[inline]
    pub fn dimensions(&self) -> VideoDimensions {
        unsafe { CMVideoFormatDescriptionGetDimensions(self) }
    }

    #[inline]
    pub fn matches_image_buffer(&self, image_buffer: &cv::ImageBuffer) -> bool {
        unsafe { CMVideoFormatDescriptionMatchesImageBuffer(self, image_buffer) }
    }

    #[inline]
    pub fn create_from_h264_parameter_sets<const N: usize>(
        pointers: &[*const u8; N],
        sizes: &[usize; N],
        nal_unit_header_length: i32,
    ) -> Result<arc::R<VideoFormatDescription>, os::Status> {
        let mut result = None;

        unsafe {
            CMVideoFormatDescriptionCreateFromH264ParameterSets(
                None,
                N,
                pointers.as_ptr(),
                sizes.as_ptr(),
                nal_unit_header_length,
                &mut result,
            )
            .to_result_unchecked(result)
        }
    }

    #[inline]
    pub fn create_from_hevc_parameter_sets(
        count: usize,
        pointers: &[*const u8],
        sizes: &[usize],
        nal_unit_header_length: i32,
        extensions: Option<&cf::Dictionary>,
    ) -> Result<arc::R<VideoFormatDescription>, os::Status> {
        let mut result = None;

        unsafe {
            CMVideoFormatDescriptionCreateFromHEVCParameterSets(
                None,
                count,
                pointers.as_ptr(),
                sizes.as_ptr(),
                nal_unit_header_length,
                extensions,
                &mut result,
            )
            .to_result_unchecked(result)
        }
    }

    #[inline]
    pub fn h264_parameters_count_and_header_length(&self) -> Result<(usize, i32), os::Status> {
        unsafe {
            let mut parameters_count_out = 0;
            let mut nal_unit_header_length_out = 0;

            CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
                self,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut parameters_count_out,
                &mut nal_unit_header_length_out,
            )
            .to_result_unchecked(Some((parameters_count_out, nal_unit_header_length_out)))
        }
    }

    #[inline]
    pub unsafe fn hevc_paramater_set_at_index(
        &self,
        parameter_set_index: usize,
        parameter_set_pointer_out: *mut *const u8,
        parameter_set_size_out: *mut usize,
        parameter_set_count_out: *mut usize,
        nal_unit_header_length_out: *mut i32,
    ) -> os::Status {
        CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(
            self,
            parameter_set_index,
            parameter_set_pointer_out,
            parameter_set_size_out,
            parameter_set_count_out,
            nal_unit_header_length_out,
        )
    }

    #[inline]
    pub fn hevc_parameters_count_and_header_length(&self) -> Result<(usize, i32), os::Status> {
        unsafe {
            let mut parameters_count_out = 0;
            let mut nal_unit_header_length_out = 0;

            CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(
                self,
                0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut parameters_count_out,
                &mut nal_unit_header_length_out,
            )
            .to_result_unchecked(Some((parameters_count_out, nal_unit_header_length_out)))
        }
    }

    #[inline]
    pub fn h264_parameter_set_at(&self, index: usize) -> Result<&[u8], os::Status> {
        unsafe {
            let mut size = 0;
            let mut bytes = std::ptr::null();
            let res = CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
                self,
                index,
                &mut bytes,
                &mut size,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            if res.is_ok() {
                let slice = std::ptr::slice_from_raw_parts(bytes, size);
                Ok(&*slice)
            } else {
                Err(res)
            }
        }
    }

    #[inline]
    pub fn hevc_parameter_set_at(&self, index: usize) -> Result<&[u8], os::Status> {
        unsafe {
            let mut size = 0;
            let mut bytes = std::ptr::null();
            let res = CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(
                self,
                index,
                &mut bytes,
                &mut size,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            if res.is_ok() {
                let slice = std::ptr::slice_from_raw_parts(bytes, size);
                Ok(&*slice)
            } else {
                Err(res)
            }
        }
    }
}

pub type AudioFormatDescription = FormatDescription;

impl AudioFormatDescription {
    pub fn with_asbd(
        asbd: &cat::audio::StreamBasicDescription,
    ) -> Result<arc::R<Self>, os::Status> {
        let mut res = None;
        unsafe {
            Self::audio_in(asbd, 0, None, 0, None, None, &mut res, None).to_result_unchecked(res)
        }
    }

    pub fn audio_in(
        asbd: &cat::audio::StreamBasicDescription,
        layout_size: usize,
        layout: Option<&cat::AudioChannelLayout<1>>,
        magic_cookie_size: usize,
        magic_cookie: Option<&c_void>,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<arc::R<Self>>,
        allocator: Option<&cf::Allocator>,
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

    /// Returns a read-only pointer to the audio stream description in an audio format description.
    ///
    /// This API is specific to audio format descriptions, and returns `None` if used with a non-audio format descriptions.
    pub fn stream_basic_description(&self) -> Option<&cat::audio::StreamBasicDescription> {
        unsafe { CMAudioFormatDescriptionGetStreamBasicDescription(self) }
    }
}

define_cf_type!(FormatDescriptionExtensionKey(cf::String));

impl FormatDescriptionExtensionKey {
    /// This extension contains a media-type-specific dictionary of settings used to produce a compressed media buffer.
    ///
    /// This extension is valid for format descriptions of all media types, but the contents of the dictionary are defined
    /// in a media-specific way.  The dictionary and its contents are valid property list objects. This means that
    /// dictionary keys are all cf::Strings, and the values are all either cf::Number, cf::String, cf::Boolean, cf::Array,
    /// cf::Dictionary, cf::Date, or cf::Data.
    ///
    /// cf::Dictionary
    pub fn original_compression_settings() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_OriginalCompressionSettings }
    }

    /// Sample description extension atoms that were not translated into other entries in the extensions dictionary.
    ///
    /// This key is used by sample description bridges to hold sample description
    /// extension atoms that they do not recognize.
    /// The extension is a cf::Dictionary mapping cf::Strings of the four-char-code atom types
    /// to either cf::Datas containing the atom payload or (to represent multiple atoms of a
    /// specific type) to cf::Arrays of cf::Data containing those payloads.
    ///
    /// cf::Dictionary of cf::String (four-char-code, atom type) -> ( cf::Data (atom payload) or cf::Array of cf::Data (atom payload) )
    pub fn sample_description_extension_atoms() -> &'static Self {
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
    pub fn verbatim_sample_description() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_VerbatimSampleDescription }
    }

    /// Preserves the original ISOSampleEntry data.
    ///
    /// This extension is used to ensure that roundtrips from ISO Sample Entry (ie. AudioSampleEntry or VisualSampleEntry)
    /// to cm::FormatDescriptions back to ISO Sample Entry preserve the exact original
    /// sample descriptions.
    ///
    /// IMPORTANT: If you make a modified clone of a CMFormatDescription, you must
    /// delete this extension from the clone, or your modifications could be lost.
    ///
    /// cf::Data
    pub fn verbatim_iso_sample_entry() -> &'static Self {
        unsafe { kCMFormatDescriptionExtension_VerbatimISOSampleEntry }
    }
}

extern "C" {
    static kCMFormatDescriptionExtension_OriginalCompressionSettings:
        &'static FormatDescriptionExtensionKey;
    static kCMFormatDescriptionExtension_SampleDescriptionExtensionAtoms:
        &'static FormatDescriptionExtensionKey;
    static kCMFormatDescriptionExtension_VerbatimSampleDescription:
        &'static FormatDescriptionExtensionKey;
    static kCMFormatDescriptionExtension_VerbatimISOSampleEntry:
        &'static FormatDescriptionExtensionKey;

    fn CMFormatDescriptionGetTypeID() -> cf::TypeId;
    fn CMFormatDescriptionGetMediaType(desc: &FormatDescription) -> MediaType;

    fn CMVideoFormatDescriptionCreate(
        allocator: Option<&cf::Allocator>,
        codec_type: VideoCodecType,
        width: i32,
        height: i32,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<arc::R<VideoFormatDescription>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionGetDimensions(
        video_desc: &VideoFormatDescription,
    ) -> VideoDimensions;

    fn CMVideoFormatDescriptionMatchesImageBuffer(
        video_desc: &VideoFormatDescription,
        image_buffer: &cv::ImageBuffer,
    ) -> bool;

    fn CMFormatDescriptionGetMediaSubType(desc: &FormatDescription) -> FourCharCode;

    fn CMFormatDescriptionGetExtensions(
        desc: &FormatDescription,
    ) -> Option<&cf::DictionaryOf<cf::String, cf::Type>>;
    fn CMFormatDescriptionGetExtension<'a>(
        desc: &'a FormatDescription,
        extension_key: &FormatDescriptionExtensionKey,
    ) -> Option<&'a cf::PropertyList>;

    fn CMAudioFormatDescriptionCreate(
        allocator: Option<&cf::Allocator>,
        asbd: &cat::audio::StreamBasicDescription,
        layout_size: usize,
        layout: Option<&cat::AudioChannelLayout<1>>,
        magic_cookie_size: usize,
        magic_cookie: Option<&c_void>,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<arc::R<AudioFormatDescription>>,
    ) -> os::Status;

    fn CMAudioFormatDescriptionGetStreamBasicDescription(
        desc: &AudioFormatDescription,
    ) -> Option<&cat::audio::StreamBasicDescription>;

    fn CMFormatDescriptionCreate(
        allocator: Option<&cf::Allocator>,
        media_type: MediaType,
        media_sub_type: FourCharCode,
        extensions: Option<&cf::Dictionary>,
        format_description_out: &mut Option<arc::R<FormatDescription>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionCreateFromH264ParameterSets(
        allocator: Option<&cf::Allocator>,
        parameter_set_count: usize,
        parameter_set_pointers: *const *const u8,
        parameter_set_sizes: *const usize,
        nal_unit_header_length: i32,
        format_descirption: &mut Option<arc::R<FormatDescription>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionCreateFromHEVCParameterSets(
        allocator: Option<&cf::Allocator>,
        parameter_set_count: usize,
        parameter_set_pointers: *const *const u8,
        parameter_set_sizes: *const usize,
        nal_unit_header_length: i32,
        extensions: Option<&cf::Dictionary>,
        format_descirption: &mut Option<arc::R<FormatDescription>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionGetH264ParameterSetAtIndex(
        video_desc: &VideoFormatDescription,
        parameter_set_index: usize,
        parameter_set_pointer_out: *mut *const u8,
        parameter_set_size_out: *mut usize,
        parameter_set_count_out: *mut usize,
        nal_unit_header_length_out: *mut i32,
    ) -> os::Status;

    fn CMVideoFormatDescriptionGetHEVCParameterSetAtIndex(
        video_desc: &VideoFormatDescription,
        parameter_set_index: usize,
        parameter_set_pointer_out: *mut *const u8,
        parameter_set_size_out: *mut usize,
        parameter_set_count_out: *mut usize,
        nal_unit_header_length_out: *mut i32,
    ) -> os::Status;
}
