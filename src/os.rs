use crate::mac_types::FourCharCode;

pub type Err = i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Status(i32);
pub type Type = FourCharCode;

pub mod lock;

impl Status {
    #[inline]
    pub fn is_ok(&self) -> bool {
        self.0 == 0
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
