#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Return(i32);

impl Return {
    /// Function executed successfully without errors.
    pub const SUCCESS: Self = Self(0);
    /// Placeholder to mark the beginning of the range of cv::Return codes.
    pub const FIRST: Self = Self(-6660);

    pub const ERROR: Self = Self::FIRST;

    /// At least one of the arguments passed in is not valid. Either out of range or the wrong type.
    pub const INVALID_ARGUMENT: Self = Self(-6661);
    /// The allocation for a buffer or buffer pool failed. Most likely because of lack of resources.
    pub const ALLOCATION_FAILED: Self = Self(-6662);

    pub const UNSUPPORTED: Self = Self(-6663);

    // DisplayLink related errors

    pub const INVALID_DISPLAY: Self = Self(-6670);
    pub const DISPLAY_LINK_ALREADY_RUNNING: Self = Self(-6671);
    pub const DISPLAY_LINK_NOT_RUNNING: Self = Self(-6672);
    pub const DISPLAY_LINK_CALLBACKS_NOT_SET: Self = Self(-6673);

    // Buffer related errors

    pub const INVALID_PIXEL_FORMAT: Self = Self(-6680);
    pub const INVALID_SIZE: Self = Self(-6681);
    pub const INVALID_PIXEL_BUFFER_ATTRIBUTES: Self = Self(-6682);
    pub const PIXEL_BUFFER_NOT_OPEN_GLCOMPATIBLE: Self = Self(-6683);
    pub const PIXEL_BUFFER_NOT_METAL_COMPATIBLE: Self = Self(-6684);

    // Buffer pool related errors

    pub const WOULD_EXCEED_ALLOCATION_THRESHOLD: Self = Self(-6689);
    pub const POOL_ALLOCATION_FAILED: Self = Self(-6690);
    pub const INVALID_POOL_ATTRIBUTES: Self = Self(-6691);
    pub const RETRY: Self = Self(-6692);

    /// Placeholder to mark the end of the range of cv::Return codes.
    pub const LAST: Self = Self(-6699);

    #[inline]
    pub fn is_ok(&self) -> bool {
        *self == Self::SUCCESS
    }

    #[inline]
    pub unsafe fn to_result<T>(self, option: Option<T>) -> Result<T, Self> {
        if self.is_ok() {
            Ok(option.unwrap_unchecked())
        } else {
            Err(self)
        }
    }

    #[inline]
    pub fn result(self) -> Result<(), Self> {
        if self.is_ok() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

// impl Into<Result<(), Return>> for Return {
//     #[inline]
//     fn into(self) -> Result<(), Return> {
//         self.result()
//     }
// }

impl From<Return> for Result<(), Return> {
    #[inline]
    fn from(r: Return) -> Self {
        r.result()
    }
}
