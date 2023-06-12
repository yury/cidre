use crate::{arc, cf, cv};

/// ```no_run
/// use cidre::cv;
///
/// let format = cv::PixelFormat::LOSSY_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE;
/// assert_eq!(false, cv::compressed_pixel_format_available(format));
///
/// ```
pub fn avaiable_compressed(pixel_format: cv::PixelFormat) -> bool {
    unsafe { CVIsCompressedPixelFormatAvailable(pixel_format) }
}

/// ```
/// use cidre::cv;
///
/// let format = cv::PixelFormat::_32_BGRA;
/// let format = cv::pixel_format_description::create(format).unwrap();
/// let format = cv::PixelFormat::LOSSY_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE;
/// let format = cv::pixel_format_description::create(format).is_none();
///
/// ```
pub fn create(pixel_format: cv::PixelFormat) -> Option<arc::R<cf::Dictionary>> {
    unsafe { CVPixelFormatDescriptionCreateWithPixelFormatType(None, pixel_format) }
}

pub fn all_pixel_formats() -> Option<arc::R<cf::ArrayOf<cf::Number>>> {
    unsafe { CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes(None) }
}

extern "C" {
    fn CVIsCompressedPixelFormatAvailable(pixel_format: cv::PixelFormat) -> bool;
    fn CVPixelFormatDescriptionCreateWithPixelFormatType(
        allocator: Option<&cf::Allocator>,
        pixel_format: cv::PixelFormat,
    ) -> Option<arc::R<cf::Dictionary>>;

    fn CVPixelFormatDescriptionArrayCreateWithAllPixelFormatTypes(
        alloc: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::ArrayOf<cf::Number>>>;
}

pub mod keys {
    use crate::cf;

    /// The canonical name for the format.  This should bethe same as the codec name you'd use in QT
    #[inline]
    pub fn name() -> &'static cf::String {
        unsafe { kCVPixelFormatName }
    }

    /// QuickTime/QuickDraw Pixel Format Type constant (os::Type)
    #[inline]
    pub fn constant() -> &'static cf::String {
        unsafe { kCVPixelFormatConstant }
    }

    /// This is the codec type constant, i.e. '2vuy' or k422YpCbCr8CodecType
    #[inline]
    pub fn codec_type() -> &'static cf::String {
        unsafe { kCVPixelFormatCodecType }
    }

    /// This is the equivalent Microsoft FourCC code for this pixel format
    #[inline]
    pub fn four_cc() -> &'static cf::String {
        unsafe { kCVPixelFormatFourCC }
    }

    #[inline]
    pub fn contains_alpha() -> &'static cf::String {
        unsafe { kCVPixelFormatContainsAlpha }
    }

    #[inline]
    pub fn contains_y_cb_cr() -> &'static cf::String {
        unsafe { kCVPixelFormatContainsYCbCr }
    }

    #[inline]
    pub fn contains_rgb() -> &'static cf::String {
        unsafe { kCVPixelFormatContainsRGB }
    }

    #[inline]
    pub fn contains_grayscale() -> &'static cf::String {
        unsafe { kCVPixelFormatContainsGrayscale }
    }

    #[inline]
    pub fn component_range() -> &'static cf::String {
        unsafe { kCVPixelFormatComponentRange }
    }

    extern "C" {
        static kCVPixelFormatName: &'static cf::String;
        static kCVPixelFormatConstant: &'static cf::String;
        static kCVPixelFormatCodecType: &'static cf::String;
        static kCVPixelFormatFourCC: &'static cf::String;
        static kCVPixelFormatContainsAlpha: &'static cf::String;
        static kCVPixelFormatContainsYCbCr: &'static cf::String;
        static kCVPixelFormatContainsRGB: &'static cf::String;
        static kCVPixelFormatContainsGrayscale: &'static cf::String;
        static kCVPixelFormatComponentRange: &'static cf::String;
    }
}

pub mod component_range {
    use crate::cf;

    #[inline]
    pub fn component_range_video_range() -> &'static cf::String {
        unsafe { kCVPixelFormatComponentRange_VideoRange }
    }

    extern "C" {
        static kCVPixelFormatComponentRange_VideoRange: &'static cf::String;
    }
}

#[cfg(test)]
mod tests {
    use crate::cv;
    #[test]
    fn all_pixel_formats() {
        let all = super::all_pixel_formats().unwrap();
        all.show();

        for f in all.iter() {
            let num = cv::PixelFormat::from_cf_number(f);
            let desc = num.to_description();
            assert!(desc.is_some())
        }
    }
}
