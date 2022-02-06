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

    pub const INVALID_DISPLAY: Self = Self(-6670);
    pub const DISPLAY_LINK_ALREADY_RUNNING: Self = Self(-6671);

    // TODO: ...

    #[inline]
    pub unsafe fn to_result<T>(&self, option: Option<T>) -> Result<T, Self> {
        if self.0 == Self::SUCCESS.0 {
            Ok(option.unwrap_unchecked())
        } else {
            Err(*self)
        }
    }
}
