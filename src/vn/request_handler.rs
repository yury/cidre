use std::mem::transmute;

use crate::{cf, cg, cv, define_obj_type, objc, vn};

define_obj_type!(ImageRequestHandler(objc::Id));

/// Performs requests on a single image.
///
/// The VNImageRequestHandler is created with an image that is used
/// to be used for the requests a client might want to schedule. The
/// vn::ImageRequestHandler retains, but never modifies, the image source
/// for its entire lifetime. The client also must not modify the content of
/// the image source once the vn::ImageRequestHandler is created otherwise
/// the results are undefined.
/// The vn::ImageRequestHandler can choose to also cache intermediate representation
/// of the image or other request-specific information for the purposes of runtime performance.
impl ImageRequestHandler {
    /// Creates a vn::ImageRequestHandler to be used for performing requests against the image passed in as buffer.
    ///
    /// # Example
    /// ```
    /// use cidre::{cf, cv, cg, vn};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32_BGRA, None).unwrap();
    /// let options = cf::Dictionary::new().unwrap();
    /// let handler = vn::ImageRequestHandler::with_cv_pixel_buffer(&pixel_buffer, &options).unwrap();
    /// let requests = cf::ArrayOf::new().unwrap();
    /// handler.perform_requests(&requests).unwrap();
    ///
    /// ````
    #[inline]
    pub fn with_cv_pixel_buffer(
        pb: &cv::PixelBuffer,
        options: &cf::Dictionary,
    ) -> Option<cf::Retained<Self>> {
        unsafe { VNImageRequestHandler_initWithCVPixelBuffer_options(pb, options) }
    }

    #[inline]
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

    #[inline]
    pub fn perform_requests<'ar>(
        &self,
        requests: &cf::ArrayOf<vn::Request>,
    ) -> Result<(), &'ar cf::Error> {
        unsafe {
            let mut error = None;
            let res = rsel_performRequests_error(self, requests, &mut error);

            if res {
                Ok(())
            } else {
                Err(transmute(error))
            }
        }
    }
}

define_obj_type!(SequenceRequestHandler(objc::Id));

impl SequenceRequestHandler {
    /// # Example
    ///
    /// ```
    /// use cidre::{vn};
    ///
    /// let sequence_handler = vn::SequenceRequestHandler::new().unwrap();
    /// ```
    #[inline]
    pub fn new() -> Option<cf::Retained<Self>> {
        unsafe { VNSequenceRequestHandler_new() }
    }
}

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

    fn rsel_performRequests_error<'ar>(
        id: &objc::Id,
        requests: &cf::ArrayOf<vn::Request>,
        error: &mut Option<&'ar cf::Error>,
    ) -> bool;

    fn VNSequenceRequestHandler_new() -> Option<cf::Retained<SequenceRequestHandler>>;
}
