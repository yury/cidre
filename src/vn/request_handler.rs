use crate::{cf, cg, cv, define_obj_type, objc};

define_obj_type!(ImageRequestHandler(objc::Id));

impl ImageRequestHandler {
    /// ```
    /// use cidre::{cf, cv, cg, vn};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32_BGRA, None).unwrap();
    /// let options = cf::Dictionary::new().unwrap();
    /// let handler = vn::ImageRequestHandler::with_cv_pixel_buffer(&pixel_buffer, &options).unwrap();
    ///
    /// ````
    pub fn with_cv_pixel_buffer(
        pb: &cv::PixelBuffer,
        options: &cf::Dictionary,
    ) -> Option<cf::Retained<Self>> {
        unsafe { VNImageRequestHandler_initWithCVPixelBuffer_options(pb, options) }
    }

    pub fn with_cv_pixel_buffer_and_orientation(
        pb: &cv::PixelBuffer,
        orientation: cg::ImagePropertyOrientation,
        options: &cf::Dictionary,
    ) -> Option<cf::Retained<Self>> {
        unsafe {
            VNImageRequestHandler_initWithCVPixelBuffer_orientation_options(
                pb,
                orientation,
                options,
            )
        }
    }
}

define_obj_type!(SequenceRequestHandler(objc::Id));

#[link(name = "vn", kind = "static")]
extern "C" {
    fn VNImageRequestHandler_initWithCVPixelBuffer_options(
        pb: &cv::PixelBuffer,
        options: &cf::Dictionary,
    ) -> Option<cf::Retained<ImageRequestHandler>>;

    fn VNImageRequestHandler_initWithCVPixelBuffer_orientation_options(
        pb: &cv::PixelBuffer,
        orientation: cg::ImagePropertyOrientation,
        options: &cf::Dictionary,
    ) -> Option<cf::Retained<ImageRequestHandler>>;
}
