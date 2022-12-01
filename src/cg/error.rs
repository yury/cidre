#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Error(pub i32);

impl Error {
    pub const SUCCESS: Self = Self(0);
    pub const FAILURE: Self = Self(1000);
    pub const ILLEGAL_ARGUMENT: Self = Self(1001);
    pub const INVALID_CONNECTION: Self = Self(1002);
    pub const INVALID_CONTEXT: Self = Self(1003);
    pub const CANNOT_COMPLETE: Self = Self(1004);
    pub const NOT_IMPLEMENTED: Self = Self(1006);
    pub const RANGE_CHECK: Self = Self(1007);
    pub const TYPE_CHECK: Self = Self(1008);
    pub const INVALID_OPERATION: Self = Self(1010);
    pub const NONE_AVAILABLE: Self = Self(1010);

    /// Set a callback for easier detection of error conditions
    /// causing CoreGraphics to raise an error.
    /// Pass None to reset the callback.
    pub fn set_callback(callback: Option<Callback>) {
        unsafe { CGErrorSetCallback(callback) }
    }
}

pub type Callback = extern "C" fn();

extern "C" {
    fn CGErrorSetCallback(callback: Option<Callback>);
}
