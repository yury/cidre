use std::num::NonZeroI32;

#[doc(alias = "CGError")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Error(pub NonZeroI32);

#[doc(alias = "CGError")]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(transparent)]
pub struct Status(pub i32);

impl Status {
    #[inline]
    pub fn result(self) -> Result<(), Error> {
        if self == Self::SUCCESS {
            Ok(())
        } else {
            Err(Error(unsafe { NonZeroI32::new_unchecked(self.0) }))
        }
    }

    #[doc(alias = "kCGErrorSuccess")]
    pub const SUCCESS: Self = Self(0);
}

impl Error {
    #[doc(alias = "kCGErrorFailure")]
    pub const FAILURE: Self = Self(unsafe { NonZeroI32::new_unchecked(1000) });

    #[doc(alias = "kCGErrorIllegalArgument")]
    pub const ILLEGAL_ARGUMENT: Self = Self(unsafe { NonZeroI32::new_unchecked(1001) });

    #[doc(alias = "kCGErrorInvalidConnection")]
    pub const INVALID_CONNECTION: Self = Self(unsafe { NonZeroI32::new_unchecked(1002) });

    #[doc(alias = "kCGErrorInvalidContext")]
    pub const INVALID_CONTEXT: Self = Self(unsafe { NonZeroI32::new_unchecked(1003) });

    #[doc(alias = "kCGErrorCannotComplete")]
    pub const CANNOT_COMPLETE: Self = Self(unsafe { NonZeroI32::new_unchecked(1004) });

    #[doc(alias = "kCGErrorNotImplemented")]
    pub const NOT_IMPLEMENTED: Self = Self(unsafe { NonZeroI32::new_unchecked(1006) });

    #[doc(alias = "kCGErrorRangeCheck")]
    pub const RANGE_CHECK: Self = Self(unsafe { NonZeroI32::new_unchecked(1007) });

    #[doc(alias = "kCGErrorTypeCheck")]
    pub const TYPE_CHECK: Self = Self(unsafe { NonZeroI32::new_unchecked(1008) });

    #[doc(alias = "kCGErrorInvalidOperation")]
    pub const INVALID_OPERATION: Self = Self(unsafe { NonZeroI32::new_unchecked(1010) });

    #[doc(alias = "kCGErrorNoneAvailable")]
    pub const NONE_AVAILABLE: Self = Self(unsafe { NonZeroI32::new_unchecked(1011) });

    /// Set a callback for easier detection of error conditions
    /// causing CoreGraphics to raise an error.
    /// Pass None to reset the callback.
    pub fn set_cb(callback: Option<Cb>) {
        unsafe { CGErrorSetCallback(callback) }
    }
}

pub type Cb = extern "C" fn();

extern "C" {
    fn CGErrorSetCallback(callback: Option<Cb>);
}
