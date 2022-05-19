use crate::cv;

/// ```
/// use cidre::cv;
/// 
/// let format = cv::PixelFormatType::LOSSY_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE;
/// assert_eq!(false, cv::compressed_pixel_format_available(format));
/// 
/// ```
pub fn compressed_pixel_format_available(pixel_format: cv::PixelFormatType) -> bool {
    unsafe { CVIsCompressedPixelFormatAvailable(pixel_format) }
}

extern "C" {
    fn CVIsCompressedPixelFormatAvailable(pixel_format: cv::PixelFormatType) -> bool;
}
