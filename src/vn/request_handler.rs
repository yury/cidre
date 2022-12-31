use std::mem::transmute;

use crate::{cf, cg, cm, cv, define_obj_type, ns, vn};

define_obj_type!(ImageRequestHandler(ns::Id));

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
    /// Creates a vn::ImageRequestHandler to be used for performing requests against an image
    /// specified by it's URL
    pub fn with_url(url: &cf::URL, options: Option<&cf::Dictionary>) -> cf::Retained<Self> {
        unsafe { VNImageRequestHandler_initWithURL_options(url, options) }
    }

    pub fn with_url_and_orientation(
        url: &cf::URL,
        orientation: cg::ImagePropertyOrientation,
        options: Option<&cf::Dictionary>,
    ) -> cf::Retained<Self> {
        unsafe { VNImageRequestHandler_initWithURL_orientation_options(url, orientation, options) }
    }

    /// Creates a vn::ImageRequestHandler to be used for performing requests against the image passed in as buffer.
    ///
    /// # Example
    /// ```
    /// use cidre::{cf, cv, cg, vn};
    ///
    /// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32_BGRA, None).unwrap();
    /// let handler = vn::ImageRequestHandler::with_cv_pixel_buffer(&pixel_buffer, None).unwrap();
    /// let requests = cf::ArrayOf::new();
    /// handler.perform(&requests).unwrap();
    ///
    /// ````
    #[inline]
    pub fn with_cv_pixel_buffer(
        pb: &cv::PixelBuffer,
        options: Option<&cf::Dictionary>,
    ) -> Option<cf::Retained<Self>> {
        unsafe { VNImageRequestHandler_initWithCVPixelBuffer_options(pb, options) }
    }

    #[inline]
    pub fn with_cv_pixel_buffer_and_orientation(
        pb: &cv::PixelBuffer,
        orientation: cg::ImagePropertyOrientation,
        options: Option<&cf::Dictionary>,
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
    pub fn perform<'ar>(&self, requests: &cf::ArrayOf<vn::Request>) -> Result<(), &'ar cf::Error> {
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

define_obj_type!(SequenceRequestHandler(ns::Id));

/// Performs requests on a sequence of images.
///
/// # Example
/// ```
/// use cidre::{cf, cv, cg, vn};
///
/// let pixel_buffer = cv::PixelBuffer::new(200, 100, cv::PixelFormatType::_32_BGRA, None).unwrap();
/// let handler = vn::SequenceRequestHandler::new();
/// let requests = cf::ArrayOf::new();
/// handler.perform_on_cv_pixel_buffer(&requests, &pixel_buffer).unwrap();
///
/// ````
impl SequenceRequestHandler {
    /// # Example
    ///
    /// ```
    /// use cidre::{vn};
    ///
    /// let handler = vn::SequenceRequestHandler::new();
    /// ```
    #[inline]
    pub fn new() -> cf::Retained<Self> {
        unsafe { VNSequenceRequestHandler_new() }
    }

    #[inline]
    pub fn perform_on_cv_pixel_buffer<'ar>(
        &self,
        requests: &cf::ArrayOf<vn::Request>,
        pixel_buffer: &cv::PixelBuffer,
    ) -> Result<(), &'ar cf::Error> {
        unsafe {
            let mut error = None;
            let res = rsel_performRequests_onCVPixelBuffer_error(
                self,
                requests,
                pixel_buffer,
                &mut error,
            );

            if res {
                Ok(())
            } else {
                Err(transmute(error))
            }
        }
    }

    #[inline]
    pub fn perform_on_cm_sample_buffer<'ar>(
        &self,
        requests: &cf::ArrayOf<vn::Request>,
        sample_buffer: &cm::SampleBuffer,
    ) -> Result<(), &'ar cf::Error> {
        unsafe {
            let mut error = None;
            let res = rsel_performRequests_onCMSampleBuffer_error(
                self,
                requests,
                sample_buffer,
                &mut error,
            );

            if res {
                Ok(())
            } else {
                Err(transmute(error))
            }
        }
    }
}

#[link(name = "vn", kind = "static")]
extern "C" {

    fn VNImageRequestHandler_initWithURL_options(
        url: &cf::URL,
        options: Option<&cf::Dictionary>,
    ) -> cf::Retained<ImageRequestHandler>;

    fn VNImageRequestHandler_initWithURL_orientation_options(
        url: &cf::URL,
        orientation: cg::ImagePropertyOrientation,
        options: Option<&cf::Dictionary>,
    ) -> cf::Retained<ImageRequestHandler>;

    fn VNImageRequestHandler_initWithCVPixelBuffer_options(
        pb: &cv::PixelBuffer,
        options: Option<&cf::Dictionary>,
    ) -> Option<cf::Retained<ImageRequestHandler>>;

    fn VNImageRequestHandler_initWithCVPixelBuffer_orientation_options(
        pb: &cv::PixelBuffer,
        orientation: cg::ImagePropertyOrientation,
        options: Option<&cf::Dictionary>,
    ) -> Option<cf::Retained<ImageRequestHandler>>;

    fn rsel_performRequests_error<'ar>(
        id: &ns::Id,
        requests: &cf::ArrayOf<vn::Request>,
        error: &mut Option<&'ar cf::Error>,
    ) -> bool;

    fn VNSequenceRequestHandler_new() -> cf::Retained<SequenceRequestHandler>;

    fn rsel_performRequests_onCVPixelBuffer_error<'ar>(
        id: &ns::Id,
        requests: &cf::ArrayOf<vn::Request>,
        pixel_buffer: &cv::PixelBuffer,
        error: &mut Option<&'ar cf::Error>,
    ) -> bool;

    fn rsel_performRequests_onCMSampleBuffer_error<'ar>(
        id: &ns::Id,
        requests: &cf::ArrayOf<vn::Request>,
        sample_buffer: &cm::SampleBuffer,
        error: &mut Option<&'ar cf::Error>,
    ) -> bool;

}

#[cfg(test)]
mod tests {
    use crate::{cf, vn};
    #[test]
    fn basics() {
        let url = cf::URL::from_str("file:///tmp/some.jpg").unwrap();
        let request = vn::DetectFaceRectanglesRequest::new();
        let handler = vn::ImageRequestHandler::with_url(&url, None);

        let requests = cf::ArrayOf::<vn::Request>::from_slice(&[&request]);
        let error = handler.perform(&requests).expect_err("should be error");

        assert!(error.domain().equal(vn::ErrorDomain::vision()));
        assert_eq!(vn::ErrorCode::InvalidImage, error.code());
        error.show();
    }
}
