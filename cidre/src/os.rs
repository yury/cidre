use std::{mem::MaybeUninit, num::NonZeroI32};

use crate::{four_cc_to_str, mac_types::FourCharCode};

pub type Err = i16;

/// <https://www.osstatus.com>
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Status(pub i32);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub struct Error(pub NonZeroI32);

impl Error {
    #[inline]
    pub const fn new_unchecked(val: i32) -> Self {
        debug_assert!(val != 0);
        Self(unsafe { NonZeroI32::new_unchecked(val) })
    }

    pub const fn from_be_bytes(bytes: [u8; std::mem::size_of::<Self>()]) -> Self {
        let val = i32::from_be_bytes(bytes);
        debug_assert!(val != 0);
        Self(unsafe { NonZeroI32::new_unchecked(val) })
    }

    pub const fn err(val: i32) -> Result {
        debug_assert!(val != 0);
        Err(Error(unsafe { NonZeroI32::new_unchecked(val) }))
    }

    pub const fn status(self) -> Status {
        Status(self.0.get())
    }
}

impl From<Error> for Status {
    fn from(value: Error) -> Self {
        value.status()
    }
}

impl std::fmt::Debug for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.0;
        let mut fcc = val.to_be_bytes();
        f.debug_struct("os::Status")
            .field("raw", &val)
            .field("fcc", &four_cc_to_str(&mut fcc))
            .field("help", &format!("https://www.osstatus.com?search={}", val))
            .finish()
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = self.0.get();
        let mut fcc = val.to_be_bytes();
        f.debug_struct("os::Error")
            .field("raw", &val)
            .field("fcc", &four_cc_to_str(&mut fcc))
            .field("help", &format!("https://www.osstatus.com?search={}", val))
            .finish()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl std::error::Error for Error {}

pub type Result<Ok = ()> = std::result::Result<Ok, Error>;

pub(crate) unsafe fn result_unchecked<T, R>(op: impl FnOnce(&mut Option<T>) -> R) -> Result<T>
where
    R: Into<Result>,
{
    let mut option = None;
    op(&mut option).into()?;
    Ok(unsafe { option.unwrap_unchecked() })
}

pub(crate) fn result_init<T, R>(op: impl FnOnce(*mut T) -> R) -> Result<T>
where
    R: Into<Result>,
{
    let mut val = MaybeUninit::<T>::uninit();
    op(val.as_mut_ptr()).into()?;
    Ok(unsafe { val.assume_init() })
}

pub type Type = FourCharCode;

pub mod lock;

impl PartialEq<i32> for Status {
    fn eq(&self, other: &i32) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<Status> for Error {
    fn eq(&self, other: &Status) -> bool {
        if other.0 == 0 {
            return false;
        }
        self.0.eq(unsafe { &NonZeroI32::new_unchecked(other.0) })
    }
}

impl Status {
    pub const NO_ERR: Self = Self(0);

    #[inline]
    pub fn is_ok(&self) -> bool {
        *self == Self::NO_ERR
    }

    #[inline]
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    #[inline]
    pub unsafe fn to_result_unchecked<T>(self, option: Option<T>) -> Result<T> {
        if self.is_ok() {
            Ok(option.unwrap_unchecked())
        } else {
            Err(Error::new_unchecked(self.0))
        }
    }

    #[inline]
    pub fn to_result_option<T>(self, option: Option<T>) -> Result<Option<T>> {
        if self.is_ok() {
            Ok(option)
        } else {
            Err(Error::new_unchecked(self.0))
        }
    }

    #[inline]
    pub fn result(self) -> Result {
        unsafe { std::mem::transmute(self) }
    }

    #[inline]
    pub fn error(self) -> Option<Error> {
        let code = NonZeroI32::new(self.0)?;
        Some(Error(code))
    }
}

impl From<Status> for Result {
    #[inline]
    fn from(v: Status) -> Self {
        v.result()
    }
}

pub mod proc;
#[cfg(any(target_os = "ios", target_os = "watchos", target_os = "tvos"))]
pub use proc::available_memory as proc_available_memory;
