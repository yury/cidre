use crate::{cf, cv};

/// ```
/// use cidre::cv;
///
/// let format = cv::PixelFormatType::LOSSY_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE;
/// assert_eq!(false, cv::compressed_pixel_format_available(format));
///
/// ```
pub fn avaiable_compressed(pixel_format: cv::PixelFormatType) -> bool {
    unsafe { CVIsCompressedPixelFormatAvailable(pixel_format) }
}

/// ```
/// use cidre::cv;
///
/// let format = cv::PixelFormatType::_32_BGRA;
/// let format = cv::pixel_format_description::create(format).unwrap();
/// let format = cv::PixelFormatType::LOSSY_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE;
/// let format = cv::pixel_format_description::create(format).is_none();
///
/// ```
pub fn create<'a>(pixel_format: cv::PixelFormatType) -> Option<cf::Retained<'a, cf::Dictionary>> {
    unsafe { CVPixelFormatDescriptionCreateWithPixelFormatType(None, pixel_format) }
}

extern "C" {
    fn CVIsCompressedPixelFormatAvailable(pixel_format: cv::PixelFormatType) -> bool;
    fn CVPixelFormatDescriptionCreateWithPixelFormatType<'a>(
        allocator: Option<&cf::Allocator>,
        pixel_format: cv::PixelFormatType,
    ) -> Option<cf::Retained<'a, cf::Dictionary>>;
}

/* Create a description of a pixel format from a provided OSType */
