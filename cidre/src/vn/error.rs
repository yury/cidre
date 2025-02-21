use crate::ns;

impl ns::ErrorDomain {
    pub fn vision() -> &'static Self {
        unsafe { VNErrorDomain }
    }
}

#[link(name = "Vision", kind = "framework")]
unsafe extern "C" {
    static VNErrorDomain: &'static ns::ErrorDomain;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum Code {
    #[doc(alias = "VNErrorTuriCoreErrorCode")]
    TuriCoreErrorCode = -1,

    /// The operation finished without error.
    #[doc(alias = "VNErrorOK")]
    Ok = 0,

    /// An app canceled the request.
    #[doc(alias = "VNErrorRequestCancelled")]
    RequestCancelled,

    /// The format of the image is invalid.
    #[doc(alias = "VNErrorInvalidFormat")]
    InvalidFormat,

    /// The requested operation failed.
    #[doc(alias = "VNErrorOperationFailed")]
    OperationFailed,

    /// An app attempted to access data that’s out-of-bounds.
    #[doc(alias = "VNErrorOutOfBoundsError")]
    OutOfBoundsError,

    /// An app specified an invalid option on a request.
    #[doc(alias = "VNErrorInvalidOption")]
    InvalidOption,

    /// An I/O error for an image, image sequence, or Core ML model.
    #[doc(alias = "VNErrorIOError")]
    IOError,

    /// A request is missing a required option.
    #[doc(alias = "VNErrorMissingOption")]
    MissingOption,

    /// The method isn’t implemented in the underlying model.
    #[doc(alias = "VNErrorNotImplemented")]
    NotImplemented,

    /// An internal error occurred within the framework.
    #[doc(alias = "VNErrorInternalError")]
    InternalError,

    /// The system doesn’t have enough memory to complete the request.
    #[doc(alias = "VNErrorOutOfMemory")]
    OutOfMemory,

    /// An unidentified error occurred.
    #[doc(alias = "VNErrorUnknownError")]
    UnknownError,

    /// An app requested an unsupported operation.
    #[doc(alias = "VNErrorInvalidOperation")]
    InvalidOperation,

    /// The image is invalid.
    ///
    /// This error occurs when you pass an invalid image to an operation, such as passing an image with no dimensions.
    #[doc(alias = "VNErrorInvalidImage")]
    InvalidImage,

    /// An app passed an invalid parameter to a request.
    #[doc(alias = "VNErrorInvalidArgument")]
    InvalidArgument,

    /// The Core ML model is incompatible with the request.
    #[doc(alias = "VNErrorInvalidModel")]
    InvalidModel,

    /// An app specified an unsupported request revision.
    #[doc(alias = "VNErrorUnsupportedRevision")]
    UnsupportedRevision,

    /// The data isn’t available.
    #[doc(alias = "VNErrorDataUnavailable")]
    DataUnavailable,

    /// The system can’t find a timestamp.
    #[doc(alias = "VNErrorTimeStampNotFound")]
    TimeStampNotFound,

    /// An app attempted an unsupported request.
    #[doc(alias = "VNErrorUnsupportedRequest")]
    UnsupportedRequest,

    #[doc(alias = "VNErrorTimeout")]
    Timeout,
}

impl PartialEq<isize> for Code {
    fn eq(&self, other: &isize) -> bool {
        *self as isize == *other
    }
}
