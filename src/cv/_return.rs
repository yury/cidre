#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Return(i32);

impl Return {
    pub const SUCCESS: Self = Self(0);
    pub const FIRST: Self = Self(-6660);
    pub const ERROR: Self = Self::FIRST;
    pub const INVALID_ARGUMENT: Self = Self(-6661);
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

    // TODO: ...

    pub const LAST: Self = Self(-6699);

    #[inline]
    pub unsafe fn to_result<T>(self, option: Option<T>) -> Result<T, Self> {
        if self == Self::SUCCESS {
            Ok(option.unwrap_unchecked())
        } else {
            Err(self)
        }
    }
}
