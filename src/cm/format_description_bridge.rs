use crate::{cf, cm, define_cf_type, os};

define_cf_type!(Flavor(cf::String));

impl Flavor {
    /// Chooses the QuickTime Movie Image Description format.
    ///
    /// Passing None is equivalent to passing this constant.
    pub fn quick_time_movie() -> &'static Self {
        unsafe { kCMImageDescriptionFlavor_QuickTimeMovie }
    }

    /// Chooses the ISO family sample description format, used in MP4
    pub fn iso_family() -> &'static Self {
        unsafe { kCMImageDescriptionFlavor_ISOFamily }
    }

    /// Chooses the 3GP family sample description format.
    ///
    /// This implies kCMImageDescriptionFlavor_ISOFamily and adds additional rules specific to the 3GP family.
    pub fn _3gp_family() -> &'static Self {
        unsafe { kCMImageDescriptionFlavor_3GPFamily }
    }

    /// Chooses the ISO family sample description format with use of Apple extensions where appropriate for M4V and M4A.
    ///
    /// This implies kCMImageDescriptionFlavor_ISOFamily and adds additional rules specific to the .m4a, .m4b, and .m4v file formats.
    pub fn iso_family_with_apple_extensions() -> &'static Self {
        unsafe { kCMImageDescriptionFlavor_ISOFamilyWithAppleExtensions }
    }
}

impl cm::VideoFormatDescription {
    /// Copies the contents of a cm::VideoFormatDescription to a cm::BlockBuffer in big-endian byte ordering.
    pub fn as_big_endian_cm_buffer_in(
        &self,
        string_encoding: Option<&cf::StringEncoding>,
        flavor: Option<&Flavor>,
        allocator: Option<&cf::Allocator>,
    ) -> Result<cf::Retained<cm::BlockBuffer>, os::Status> {
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

    pub fn as_big_endian_cm_buffer(
        &self,
        flavor: Option<&Flavor>,
    ) -> Result<cf::Retained<cm::BlockBuffer>, os::Status> {
        Self::as_big_endian_cm_buffer_in(self, None, flavor, None)
    }

    pub fn from_big_endian_image_description_in(
        image_description_block_buffer: &cm::BlockBuffer,
        string_encoding: cf::StringEncoding,
        flavor: Option<&Flavor>,
        allocator: Option<&cf::Allocator>,
    ) -> Result<cf::Retained<Self>, os::Status> {
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

    pub fn from_big_endian_image_description(
        image_description_block_buffer: &cm::BlockBuffer,
        flavor: Option<&Flavor>,
    ) -> Result<cf::Retained<Self>, os::Status> {
        Self::from_big_endian_image_description_in(
            image_description_block_buffer,
            cf::StringEncoding::system_encoding(),
            flavor,
            None,
        )
    }
}

#[link(name = "CoreMedia", kind = "framework")]
extern "C" {
    static kCMImageDescriptionFlavor_QuickTimeMovie: &'static Flavor;
    static kCMImageDescriptionFlavor_ISOFamily: &'static Flavor;
    static kCMImageDescriptionFlavor_3GPFamily: &'static Flavor;
    static kCMImageDescriptionFlavor_ISOFamilyWithAppleExtensions: &'static Flavor;

    fn CMVideoFormatDescriptionCopyAsBigEndianImageDescriptionBlockBuffer(
        allocator: Option<&cf::Allocator>,
        video_format_description: &cm::VideoFormatDescription,
        string_encoding: Option<&cf::StringEncoding>,
        flavor: Option<&Flavor>,
        block_buffer_out: &mut Option<cf::Retained<cm::BlockBuffer>>,
    ) -> os::Status;

    fn CMVideoFormatDescriptionCreateFromBigEndianImageDescriptionBlockBuffer(
        allocator: Option<&cf::Allocator>,
        image_description_block_buffer: &cm::BlockBuffer,
        string_encoding: cf::StringEncoding,
        flavor: Option<&cm::ImageDescriptionFlavor>,
        format_description_out: &mut Option<cf::Retained<cm::FormatDescription>>,
    ) -> os::Status;
}
