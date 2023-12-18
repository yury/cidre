use crate::{arc, cf, cm, define_cf_type, os};

pub mod errors {
    use crate::os::Status;

    /// Invalid parameter.
    #[doc(alias = "kCMFormatDescriptionBridgeError_InvalidParameter")]
    pub const INVALID_PARAMETER: Status = Status(-12712);

    /// Returned when an allocation fails.
    #[doc(alias = "kCMFormatDescriptionBridgeError_AllocationFailed")]
    pub const ALLOCATION_FAILED: Status = Status(-12713);

    /// Returned when the sample description is invalid (e.g. invalid size).
    #[doc(alias = "kCMFormatDescriptionBridgeError_InvalidSerializedSampleDescription")]
    pub const INVALID_SERIALIZED_SAMPLE_DESCRIPTION: Status = Status(-12714);

    /// Returned when the format description is invalid (e.g. invalid size).
    #[doc(alias = "kCMFormatDescriptionBridgeError_InvalidFormatDescription")]
    pub const INVALID_FORMAT_DESCRIPTION: Status = Status(-12715);

    /// Returned when the format description has an incompatible format (e.g. unknown format / incompatible atom).
    #[doc(alias = "kCMFormatDescriptionBridgeError_IncompatibleFormatDescription")]
    pub const INCOMPATIBLE_FORMAT_DESCRIPTION: Status = Status(-12716);

    /// Returned when the sample description is unsupported for the specified format flavor.
    #[doc(alias = "kCMFormatDescriptionBridgeError_UnsupportedSampleDescriptionFlavor")]
    pub const UNSUPPORTED_SAMPLE_DESCRIPTION_FLAVOR: Status = Status(-12717);

    /// Returned when the slice has an invalid value.
    #[doc(alias = "kCMFormatDescriptionBridgeError_InvalidSlice")]
    pub const INVALID_SLICE: Status = Status(-12719);
}

define_cf_type!(
    #[doc(alias = "CMImageDescriptionFlavor")]
    ImageDescFlavor(cf::String)
);

impl ImageDescFlavor {
    /// Chooses the QuickTime Movie Image Description format.
    ///
    /// Passing None is equivalent to passing this constant.
    #[doc(alias = "kCMImageDescriptionFlavor_QuickTimeMovie")]
    pub fn quick_time_movie() -> &'static Self {
        unsafe { kCMImageDescriptionFlavor_QuickTimeMovie }
    }

    /// Chooses the ISO family sample description format, used in MP4
    #[doc(alias = "kCMImageDescriptionFlavor_ISOFamily")]
    pub fn iso_family() -> &'static Self {
        unsafe { kCMImageDescriptionFlavor_ISOFamily }
    }

    /// Chooses the 3GP family sample description format.
    ///
    /// This implies [`iso_family()`] and adds additional rules specific to the 3GP family.
    #[doc(alias = "kCMImageDescriptionFlavor_3GPFamily")]
    pub fn _3gp_family() -> &'static Self {
        unsafe { kCMImageDescriptionFlavor_3GPFamily }
    }

    /// Chooses the ISO family sample description format with use of Apple extensions where appropriate for M4V and M4A.
    ///
    /// This implies [`iso_family()`] and adds additional rules specific to the .m4a, .m4b, and .m4v file formats.
    #[doc(alias = "kCMImageDescriptionFlavor_ISOFamilyWithAppleExtensions")]
    pub fn iso_family_with_apple_exts() -> &'static Self {
        unsafe { kCMImageDescriptionFlavor_ISOFamilyWithAppleExtensions }
    }
}

impl cm::VideoFormatDesc {
    /// Copies the contents of a cm::VideoFormatDescription to a cm::BlockBuffer in big-endian byte ordering.
    #[doc(alias = "CMVideoFormatDescriptionCopyAsBigEndianImageDescriptionBlockBuffer")]
    pub fn as_be_image_desc_cm_buf_in(
        &self,
        string_encoding: cf::StringEncoding,
        flavor: Option<&ImageDescFlavor>,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<cm::BlockBuf>, os::Status> {
        unsafe {
            let mut buffer_out = None;
            CMVideoFormatDescriptionCopyAsBigEndianImageDescriptionBlockBuffer(
                allocator,
                self,
                string_encoding,
                flavor,
                &mut buffer_out,
            )
            .to_result_unchecked(buffer_out)
        }
    }

    #[doc(alias = "CMVideoFormatDescriptionCopyAsBigEndianImageDescriptionBlockBuffer")]
    pub fn as_be_image_desc_cm_buf(
        &self,
        flavor: Option<&ImageDescFlavor>,
    ) -> Result<arc::R<cm::BlockBuf>, os::Status> {
        Self::as_be_image_desc_cm_buf_in(self, cf::StringEncoding::system_encoding(), flavor, None)
    }

    pub fn from_be_image_desc_buf_in(
        image_description_block_buffer: &cm::BlockBuf,
        string_encoding: cf::StringEncoding,
        flavor: Option<&ImageDescFlavor>,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<Self>, os::Status> {
        let mut result = None;
        unsafe {
            CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionBlockBuffer(
                allocator,
                image_description_block_buffer,
                string_encoding,
                flavor,
                &mut result,
            )
            .to_result_unchecked(result)
        }
    }

    pub fn from_be_image_desc_buf(
        image_description_block_buffer: &cm::BlockBuf,
        flavor: Option<&ImageDescFlavor>,
    ) -> Result<arc::R<Self>, os::Status> {
        Self::from_be_image_desc_buf_in(
            image_description_block_buffer,
            cf::StringEncoding::system_encoding(),
            flavor,
            None,
        )
    }

    pub fn from_be_image_desc_data_in(
        data: &[u8],
        string_encoding: cf::StringEncoding,
        flavor: Option<&ImageDescFlavor>,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<Self>, os::Status> {
        let mut result = None;
        unsafe {
            CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionData(
                allocator,
                data.as_ptr(),
                data.len(),
                string_encoding,
                flavor,
                &mut result,
            )
            .to_result_unchecked(result)
        }
    }

    pub fn from_be_image_desc_data(
        data: &[u8],
        string_encoding: cf::StringEncoding,
        flavor: Option<&ImageDescFlavor>,
    ) -> Result<arc::R<Self>, os::Status> {
        Self::from_be_image_desc_data_in(data, string_encoding, flavor, None)
    }
}

/// Converts a ImageDescription data structure from big-endian to host-endian in place.
#[inline]
pub fn swap_be_image_desc_to_host(desc: &mut [u8]) -> os::Status {
    unsafe { CMSwapBigEndianImageDescriptionToHost(desc.as_mut_ptr(), desc.len()) }
}

/// Converts an ImageDescription data structure from host-endian to big-endian in place.
#[inline]
pub fn swap_host_image_desc_to_be(desc: &mut [u8]) -> os::Status {
    unsafe { CMSwapHostEndianImageDescriptionToBig(desc.as_mut_ptr(), desc.len()) }
}

/// Converts a SoundDescription data structure from big-endian to host-endian in place.
#[inline]
pub fn swap_be_sound_desc_to_host(desc: &mut [u8]) -> os::Status {
    unsafe { CMSwapBigEndianSoundDescriptionToHost(desc.as_mut_ptr(), desc.len()) }
}

/// Converts a SoundDescription data structure from host-endian to big-endian in place.
#[inline]
pub fn swap_host_sound_desc_to_be(desc: &mut [u8]) -> os::Status {
    unsafe { CMSwapHostEndianSoundDescriptionToBig(desc.as_mut_ptr(), desc.len()) }
}

define_cf_type!(
    #[doc(alias = "CMSoundDescriptionFlavor")]
    SoundDescFlavor(cf::String)
);

impl SoundDescFlavor {
    /// Chooses the most backwards-compatible QuickTime Movie Sound Description format.
    /// A V1 sound description will be written if possible.
    /// If a V1 sound description is written for CBR or PCM audio, the sample tables will need to use the legacy CBR layout.
    #[doc(alias = "kCMSoundDescriptionFlavor_QuickTimeMovie")]
    pub fn quick_time_movie() -> &'static Self {
        unsafe { kCMSoundDescriptionFlavor_QuickTimeMovie }
    }

    /// Chooses the QuickTime Movie Sound Description V2 format.
    /// A V2 sound description will be written.
    /// V2 Sound Descriptions contain no legacy CBR layout, and use 'lpcm' for all flavors of PCM.
    #[doc(alias = "kCMSoundDescriptionFlavor_QuickTimeMovieV2")]
    pub fn quick_time_movie_v2() -> &'static Self {
        unsafe { kCMSoundDescriptionFlavor_QuickTimeMovieV2 }
    }

    /// Chooses the ISO family sample description format, used in MP4, M4A, etc.
    #[doc(alias = "kCMSoundDescriptionFlavor_ISOFamily")]
    pub fn iso_family() -> &'static Self {
        unsafe { kCMSoundDescriptionFlavor_ISOFamily }
    }

    /// Chooses the 3GP family sample description format.
    /// This implies iso_family and adds additional rules specific to the 3GP family.
    #[doc(alias = "kCMSoundDescriptionFlavor_3GPFamily")]
    pub fn _3gp_family() -> &'static Self {
        unsafe { kCMSoundDescriptionFlavor_3GPFamily }
    }
}

impl cm::AudioFormatDesc {
    /// Copies the contents of a CMAudioFormatDescription to a [`cm::BlockBuf`] in big-endian byte ordering.
    ///
    /// Note that the dataRefIndex field of the SampleDescription is intentionally filled with
    /// garbage values (0xFFFF).  The caller must overwrite these values with a valid dataRefIndex
    /// if writing the SampleDescription to a QuickTime/ISO file.
    #[doc(alias = "CMAudioFormatDescriptionCopyAsBigEndianSoundDescriptionBlockBuffer")]
    pub fn as_be_sound_desc_cm_buf_in(
        &self,
        flavor: Option<&SoundDescFlavor>,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<cm::BlockBuf>, os::Status> {
        unsafe {
            let mut buffer_out = None;
            CMAudioFormatDescriptionCopyAsBigEndianSoundDescriptionBlockBuffer(
                allocator,
                self,
                flavor,
                &mut buffer_out,
            )
            .to_result_unchecked(buffer_out)
        }
    }

    #[doc(alias = "CMAudioFormatDescriptionCopyAsBigEndianSoundDescriptionBlockBuffer")]
    pub fn as_be_sound_desc_cm_buf(
        &self,
        flavor: Option<&SoundDescFlavor>,
    ) -> Result<arc::R<cm::BlockBuf>, os::Status> {
        Self::as_be_sound_desc_cm_buf_in(self, flavor, None)
    }

    #[doc(alias = "CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionData")]
    pub fn from_be_sound_desc_data_in(
        data: &[u8],
        flavor: Option<&SoundDescFlavor>,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<Self>, os::Status> {
        let mut result = None;
        unsafe {
            CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionData(
                allocator,
                data.as_ptr(),
                data.len(),
                flavor,
                &mut result,
            )
            .to_result_unchecked(result)
        }
    }

    #[doc(alias = "CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionData")]
    #[inline]
    pub fn from_be_sound_desc_data(
        data: &[u8],
        flavor: Option<&SoundDescFlavor>,
    ) -> Result<arc::R<Self>, os::Status> {
        Self::from_be_sound_desc_data_in(data, flavor, None)
    }

    pub fn from_be_sound_desc_buf_in(
        buffer: &cm::BlockBuf,
        flavor: Option<&SoundDescFlavor>,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<Self>, os::Status> {
        let mut result = None;
        unsafe {
            CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionBlockBuffer(
                allocator,
                buffer,
                flavor,
                &mut result,
            )
            .to_result_unchecked(result)
        }
    }

    #[inline]
    pub fn from_be_sound_desc_buf(
        buffer: &cm::BlockBuf,
        flavor: Option<&SoundDescFlavor>,
    ) -> Result<arc::R<Self>, os::Status> {
        Self::from_be_sound_desc_buf_in(buffer, flavor, None)
    }
}

define_cf_type!(
    #[doc(alias = "CMTextDescriptionFlavor")]
    TextDescFlavor(cf::String)
);

impl TextDescFlavor {}

#[link(name = "CoreMedia", kind = "framework")]
extern "C" {
    static kCMImageDescriptionFlavor_QuickTimeMovie: &'static ImageDescFlavor;
    static kCMImageDescriptionFlavor_ISOFamily: &'static ImageDescFlavor;
    static kCMImageDescriptionFlavor_3GPFamily: &'static ImageDescFlavor;
    static kCMImageDescriptionFlavor_ISOFamilyWithAppleExtensions: &'static ImageDescFlavor;

    static kCMSoundDescriptionFlavor_QuickTimeMovie: &'static SoundDescFlavor;
    static kCMSoundDescriptionFlavor_QuickTimeMovieV2: &'static SoundDescFlavor;
    static kCMSoundDescriptionFlavor_ISOFamily: &'static SoundDescFlavor;
    static kCMSoundDescriptionFlavor_3GPFamily: &'static SoundDescFlavor;

    fn CMVideoFormatDescriptionCopyAsBigEndianImageDescriptionBlockBuffer(
        allocator: Option<&cf::Allocator>,
        video_format_description: &cm::VideoFormatDesc,
        string_encoding: cf::StringEncoding,
        flavor: Option<&ImageDescFlavor>,
        block_buffer_out: &mut Option<arc::R<cm::BlockBuf>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionBlockBuffer(
        allocator: Option<&cf::Allocator>,
        image_description_block_buffer: &cm::BlockBuf,
        string_encoding: cf::StringEncoding,
        flavor: Option<&cm::ImageDescFlavor>,
        format_description_out: &mut Option<arc::R<cm::VideoFormatDesc>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionData(
        allocator: Option<&cf::Allocator>,
        image_description_data: *const u8,
        size: usize,
        string_encoding: cf::StringEncoding,
        flavor: Option<&cm::ImageDescFlavor>,
        format_description_out: &mut Option<arc::R<cm::VideoFormatDesc>>,
    ) -> os::Status;

    fn CMSwapBigEndianImageDescriptionToHost(
        image_description_data: *mut u8,
        image_description_size: usize,
    ) -> os::Status;

    fn CMSwapHostEndianImageDescriptionToBig(
        image_description_data: *mut u8,
        image_description_size: usize,
    ) -> os::Status;

    fn CMSwapBigEndianSoundDescriptionToHost(
        image_description_data: *mut u8,
        image_description_size: usize,
    ) -> os::Status;

    fn CMSwapHostEndianSoundDescriptionToBig(
        image_description_data: *mut u8,
        image_description_size: usize,
    ) -> os::Status;

    fn CMAudioFormatDescriptionCopyAsBigEndianSoundDescriptionBlockBuffer(
        allocator: Option<&cf::Allocator>,
        audio_format_description: &cm::AudioFormatDesc,
        flavor: Option<&SoundDescFlavor>,
        block_buffer_out: &mut Option<arc::R<cm::BlockBuf>>,
    ) -> os::Status;

    fn CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionData(
        allocator: Option<&cf::Allocator>,
        sound_description_data: *const u8,
        size: usize,
        flavor: Option<&SoundDescFlavor>,
        format_description_out: &mut Option<arc::R<cm::AudioFormatDesc>>,
    ) -> os::Status;

    fn CMAudioFormatDescriptionCreateFromBigEndianSoundDescriptionBlockBuffer(
        allocator: Option<&cf::Allocator>,
        block_buffer: &cm::BlockBuf,
        flavor: Option<&SoundDescFlavor>,
        format_description_out: &mut Option<arc::R<cm::AudioFormatDesc>>,
    ) -> os::Status;
}
