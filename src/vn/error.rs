use crate::cf;

pub type Domain = cf::ErrorDomain;

impl Domain {
    pub fn vision() -> &'static Domain {
        unsafe { VNErrorDomain }
    }
}

#[link(name = "Vision", kind = "framework")]
extern "C" {
    static VNErrorDomain: &'static Domain;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(isize)]
pub enum Code {
    TuriCoreErrorCode = -1,
    Ok = 0,
    RequestCancelled,
    InvalidFormat,
    OperationFailed,
    OutOfBoundsError,
    InvalidOption,
    IOError,
    MissingOption,
    NotImplemented,
    InternalError,
    OutOfMemory,
    UnknownError,
    InvalidOperation,
    InvalidImage,
    InvalidArgument,
    InvalidModel,
    UnsupportedRevision,
    DataUnavailable,
    TimeStampNotFound,
    UnsupportedRequest,
    Timeout,
}

impl PartialEq<isize> for Code {
    fn eq(&self, other: &isize) -> bool {
        *self as isize == *other
    }
}
