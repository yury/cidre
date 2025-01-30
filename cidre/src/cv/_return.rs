/// Check error values with cv::err module
pub type Return = crate::os::Status;

pub mod err {
    use crate::os::Error;

    /// Placeholder to mark the beginning of the range of cv::err codes.
    #[doc(alias = "kCVReturnFirst")]
    pub const FIRST: Error = Error::new_unchecked(-6660);

    #[doc(alias = "kCVReturnError")]
    pub const ERROR: Error = FIRST;

    /// At least one of the arguments passed in is not valid. Either out of range or the wrong type.
    #[doc(alias = "kCVReturnInvalidArgument")]
    pub const INVALID_ARG: Error = Error::new_unchecked(-6661);
    /// The allocation for a buffer or buffer pool failed. Most likely because of lack of resources.
    #[doc(alias = "kCVReturnAllocationFailed")]
    pub const ALLOCATION_FAILED: Error = Error::new_unchecked(-6662);

    #[doc(alias = "kCVReturnUnsupported")]
    pub const UNSUPPORTED: Error = Error::new_unchecked(-6663);

    // DisplayLink related errors

    /// A cv::DisplayLink cannot be created for the given DisplayRef.
    #[doc(alias = "kCVReturnInvalidDisplay")]
    pub const INVALID_DISPLAY: Error = Error::new_unchecked(-6670);

    /// The cv::DisplayLink is already started and running.
    #[doc(alias = "kCVReturnDisplayLinkAlreadyRunning")]
    pub const DISPLAY_LINK_ALREADY_RUNNING: Error = Error::new_unchecked(-6671);

    /// The cv::DisplayLink has not been started.
    #[doc(alias = "kCVReturnDisplayLinkNotRunning")]
    pub const DISPLAY_LINK_NOT_RUNNING: Error = Error::new_unchecked(-6672);

    /// The output callback is not set.
    #[doc(alias = "kCVReturnDisplayLinkCallbacksNotSet")]
    pub const DISPLAY_LINK_CALLBACKS_NOT_SET: Error = Error::new_unchecked(-6673);

    // Buffer related errors

    /// The requested pixelformat is not supported for the cv::Buf type.
    #[doc(alias = "kCVReturnInvalidPixelFormat")]
    pub const INVALID_PIXEL_FORMAT: Error = Error::new_unchecked(-6680);

    ///  The requested size (most likely too big) is not supported for the cv::Buf type.
    #[doc(alias = "kCVReturnInvalidSize")]
    pub const INVALID_SIZE: Error = Error::new_unchecked(-6681);

    /// A cv::Buffer cannot be created with the given attributes.
    #[doc(alias = "kCVReturnInvalidPixelBufferAttributes")]
    pub const INVALID_PIXEL_BUF_ATTRS: Error = Error::new_unchecked(-6682);

    /// The Buffer cannot be used with OpenGL as either its size, pixelformat or attributes are not supported by OpenGL.
    #[doc(alias = "kCVReturnPixelBufferNotOpenGLCompatible")]
    pub const PIXEL_BUF_NOT_OPEN_GL_COMPATIBLE: Error = Error::new_unchecked(-6683);

    /// The Buffer cannot be used with Metal as either its size, pixelformat or attributes are not supported by Metal.
    #[doc(alias = "kCVReturnPixelBufferNotMetalCompatible")]
    pub const PIXEL_BUF_NOT_METAL_COMPATIBLE: Error = Error::new_unchecked(-6684);

    // Buffer pool related errors

    /// The allocation request failed because it would have exceeded a specified allocation threshold (see kCVPixelBufferPoolAllocationThresholdKey).
    #[doc(alias = "kCVReturnWouldExceedAllocationThreshold")]
    pub const WOULD_EXCEED_ALLOCATION_THRESHOLD: Error = Error::new_unchecked(-6689);

    /// The allocation for the buffer pool failed. Most likely because of lack of resources. Check if your parameters are in range.
    #[doc(alias = "kCVReturnPoolAllocationFailed")]
    pub const POOL_ALLOCATION_FAILED: Error = Error::new_unchecked(-6690);

    /// A cv::BufPool cannot be created with the given attributes.
    #[doc(alias = "kCVReturnInvalidPoolAttributes")]
    pub const INVALID_POOL_ATTRS: Error = Error::new_unchecked(-6691);

    /// A scan hasn't completely traversed the CVBufferPool due to a concurrent operation. The client can retry the scan.
    #[doc(alias = "kCVReturnRetry")]
    pub const RETRY: Error = Error::new_unchecked(-6692);

    /// Placeholder to mark the end of the range of cv::err codes.
    #[doc(alias = "kCVReturnLast")]
    pub const LAST: Error = Error::new_unchecked(-6699);
}
