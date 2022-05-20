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

pub fn all_pixel_formats<'a>() -> Option<cf::Retained<'a, cf::ArrayOf<cf::Number>>> {
    unsafe { CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes(None) }
}

extern "C" {
    fn CVIsCompressedPixelFormatAvailable(pixel_format: cv::PixelFormatType) -> bool;
    fn CVPixelFormatDescriptionCreateWithPixelFormatType<'a>(
        allocator: Option<&cf::Allocator>,
        pixel_format: cv::PixelFormatType,
    ) -> Option<cf::Retained<'a, cf::Dictionary>>;

    fn CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes<'a>(
        alloc: Option<&cf::Allocator>,
    ) -> Option<cf::Retained<'a, cf::ArrayOf<cf::Number>>>;
}

#[cfg(test)]
mod tests {
    use crate::cv;
    #[test]
    fn all_pixel_formats() {
        let all = super::all_pixel_formats().unwrap();
        all.show();

        for f in all.iter() {
            let num = cv::PixelFormatType::from_cf_number(f);
            let desc = num.to_description();
            assert!(desc.is_some())
        }
    }
}
