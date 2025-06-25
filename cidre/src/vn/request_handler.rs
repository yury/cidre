use crate::{arc, cg, cm, cv, define_cls, define_obj_type, ns, objc, vn};

define_obj_type!(pub ImageRequestHandler(ns::Id));

impl arc::A<ImageRequestHandler> {
    #[objc::msg_send(initWithURL:options:)]
    pub fn init_with_url_options(
        self,
        url: &ns::Url,
        options: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> arc::R<ImageRequestHandler>;

    #[objc::msg_send(initWithURL:orientation:options:)]
    pub fn init_with_url_orientation_options(
        self,
        url: &ns::Url,
        orientation: cg::ImagePropOrientation,
        options: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> arc::R<ImageRequestHandler>;

    #[objc::msg_send(initWithCVPixelBuffer:options:)]
    pub fn init_with_cv_pixel_buf_options(
        self,
        pb: &cv::PixelBuf,
        options: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> Option<arc::R<ImageRequestHandler>>;

    #[objc::msg_send(initWithCVPixelBuffer:orientation:options:)]
    pub fn init_with_cv_pixel_buf_orientaion_options(
        self,
        pb: &cv::PixelBuf,
        orientation: cg::ImagePropOrientation,
        options: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> Option<arc::R<ImageRequestHandler>>;
}

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
    define_cls!(VN_IMAGE_REQUEST_HANDLER);
    /// Creates a vn::ImageRequestHandler to be used for performing requests against an image
    /// specified by it's URL
    pub fn with_url(
        url: &ns::Url,
        options: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_url_options(url, options)
    }

    pub fn with_url_and_orientation(
        url: &ns::Url,
        orientation: cg::ImagePropOrientation,
        options: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_url_orientation_options(url, orientation, options)
    }

    /// Creates a vn::ImageRequestHandler to be used for performing requests against the image passed in as buffer.
    ///
    /// # Example
    ///
    /// ```
    /// use cidre::{ns, cv, cg, vn};
    ///
    /// let pixel_buf = cv::PixelBuf::new(200, 100, cv::PixelFormat::_32_BGRA, None).unwrap();
    /// let handler = vn::ImageRequestHandler::with_cv_pixel_buf(&pixel_buf, None).unwrap();
    /// let requests = ns::Array::new();
    /// handler.perform(&requests).unwrap();
    ///
    /// ````
    #[inline]
    pub fn with_cv_pixel_buf(
        pb: &cv::PixelBuf,
        options: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_cv_pixel_buf_options(pb, options)
    }

    #[inline]
    pub fn with_cv_pixel_buf_and_orientation(
        pb: &cv::PixelBuf,
        orientation: cg::ImagePropOrientation,
        options: Option<&ns::Dictionary<ns::Id, ns::Id>>,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_cv_pixel_buf_orientaion_options(pb, orientation, options)
    }

    #[objc::msg_send(performRequests:error:)]
    pub unsafe fn perform_request_err<'ear>(
        &self,
        requests: &ns::Array<vn::Request>,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    #[inline]
    pub fn perform<'ear>(&self, requests: &ns::Array<vn::Request>) -> ns::Result<'ear> {
        unsafe { ns::if_false(|err| self.perform_request_err(requests, err)) }
    }
}

define_obj_type!(pub SequenceRequestHandler(ns::Id), VN_SEQUENCE_REQUEST_HANDLER);

/// Performs requests on a sequence of images.
///
/// # Example
/// ```no_run
/// use cidre::{ns, cv, cg, vn};
///
/// let pixel_buf = cv::PixelBuf::new(200, 100, cv::PixelFormat::_32_BGRA, None).unwrap();
/// let handler = vn::SequenceRequestHandler::new();
/// let requests = ns::Array::new();
/// handler.perform_on_cv_pixel_buf(&requests, &pixel_buf).unwrap();
///
/// ````
impl SequenceRequestHandler {
    #[objc::msg_send(performRequests:onCVPixelBuffer:error:)]
    pub unsafe fn perform_requests_on_cv_pixel_buf_err<'ear>(
        &self,
        requests: &ns::Array<vn::Request>,
        pixel_buf: &cv::PixelBuf,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    #[inline]
    pub fn perform_on_cv_pixel_buf<'ear>(
        &self,
        requests: &ns::Array<vn::Request>,
        pixel_buf: &cv::PixelBuf,
    ) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe {
            self.perform_requests_on_cv_pixel_buf_err(requests, pixel_buf, err)
        })
    }

    #[objc::msg_send(performRequests:onCMSampleBuffer:error:)]
    pub unsafe fn perform_requests_on_cm_sample_buf_err<'ear>(
        &self,
        requests: &ns::Array<vn::Request>,
        sample_buf: &cm::SampleBuf,
        error: *mut Option<&'ear ns::Error>,
    ) -> bool;

    #[inline]
    pub fn perform_on_cm_sample_buf<'ear>(
        &self,
        requests: &ns::Array<vn::Request>,
        sample_buf: &cm::SampleBuf,
    ) -> ns::Result<'ear> {
        ns::if_false(|err| unsafe {
            self.perform_requests_on_cm_sample_buf_err(requests, sample_buf, err)
        })
    }
}

#[link(name = "vn", kind = "static")]
unsafe extern "C" {
    static VN_IMAGE_REQUEST_HANDLER: &'static objc::Class<ImageRequestHandler>;
    static VN_SEQUENCE_REQUEST_HANDLER: &'static objc::Class<SequenceRequestHandler>;
}

#[cfg(test)]
mod tests {
    use crate::{ns, vn};
    #[test]
    fn basics() {
        let url = ns::Url::with_str("file:///tmp/some.jpg").unwrap();
        let request = vn::DetectFaceRectanglesRequest::new();
        let handler = vn::ImageRequestHandler::with_url(&url, None);

        let requests = ns::Array::<vn::Request>::from_slice(&[&request]);
        let error = handler.perform(&requests).expect_err("should be error");

        assert!(error.domain().is_equal(ns::ErrorDomain::vision()));
        assert_eq!(vn::ErrorCode::InvalidImage, error.code());
    }
}
