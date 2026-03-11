use std::{
    ffi::{CStr, c_char, c_int},
    mem::MaybeUninit,
    num::NonZeroU32,
    os::fd::RawFd,
};

#[cfg(all(feature = "blocks", feature = "dispatch"))]
use crate::{blocks, dispatch};
use crate::{define_opts, mach};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Error(pub NonZeroU32);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Status(pub u32);

pub type Result<Ok = ()> = std::result::Result<Ok, Error>;

impl Error {
    #[inline]
    pub const fn new_unchecked(val: u32) -> Self {
        debug_assert!(val != 0);
        Self(unsafe { NonZeroU32::new_unchecked(val) })
    }

    #[inline]
    pub const fn status(self) -> Status {
        Status(self.0.get())
    }

    #[doc(alias = "NOTIFY_STATUS_INVALID_NAME")]
    pub const INVALID_NAME: Self = Self::new_unchecked(1);

    #[doc(alias = "NOTIFY_STATUS_INVALID_TOKEN")]
    pub const INVALID_TOKEN: Self = Self::new_unchecked(2);

    #[doc(alias = "NOTIFY_STATUS_INVALID_PORT")]
    pub const INVALID_PORT: Self = Self::new_unchecked(3);

    #[doc(alias = "NOTIFY_STATUS_INVALID_FILE")]
    pub const INVALID_FILE: Self = Self::new_unchecked(4);

    #[doc(alias = "NOTIFY_STATUS_INVALID_SIGNAL")]
    pub const INVALID_SIGNAL: Self = Self::new_unchecked(5);

    #[doc(alias = "NOTIFY_STATUS_INVALID_REQUEST")]
    pub const INVALID_REQUEST: Self = Self::new_unchecked(6);

    #[doc(alias = "NOTIFY_STATUS_NOT_AUTHORIZED")]
    pub const NOT_AUTHORIZED: Self = Self::new_unchecked(7);

    #[doc(alias = "NOTIFY_STATUS_OPT_DISABLE")]
    pub const OPT_DISABLE: Self = Self::new_unchecked(8);

    #[doc(alias = "NOTIFY_STATUS_SERVER_NOT_FOUND")]
    pub const SERVER_NOT_FOUND: Self = Self::new_unchecked(9);

    #[doc(alias = "NOTIFY_STATUS_NULL_INPUT")]
    pub const NULL_INPUT: Self = Self::new_unchecked(10);

    #[doc(alias = "NOTIFY_STATUS_FAILED")]
    pub const FAILED: Self = Self::new_unchecked(1_000_000);
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "notify status {}", self.0)
    }
}

impl std::error::Error for Error {}

impl Status {
    #[doc(alias = "NOTIFY_STATUS_OK")]
    pub const OK: Self = Self(0);

    #[inline]
    pub fn is_ok(self) -> bool {
        self == Self::OK
    }

    #[inline]
    pub fn is_err(self) -> bool {
        !self.is_ok()
    }

    #[inline]
    pub fn result(self) -> Result {
        match NonZeroU32::new(self.0) {
            Some(err) => Err(Error(err)),
            None => Ok(()),
        }
    }

    #[inline]
    pub fn error(self) -> Option<Error> {
        NonZeroU32::new(self.0).map(Error)
    }
}

impl From<Error> for Status {
    #[inline]
    fn from(value: Error) -> Self {
        value.status()
    }
}

impl From<Status> for Result {
    #[inline]
    fn from(value: Status) -> Self {
        value.result()
    }
}

define_opts!(
    /// Flag bits used for notification registration.
    pub RegFlags(c_int)
);

impl RegFlags {
    #[doc(alias = "NOTIFY_REUSE")]
    pub const REUSE: Self = Self(0x0000_0001);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Token(pub c_int);

impl Default for Token {
    fn default() -> Self {
        Self::INVALID
    }
}

impl Token {
    #[doc(alias = "NOTIFY_TOKEN_INVALID")]
    pub const INVALID: Self = Self(-1);

    #[inline]
    pub fn is_invalid(self) -> bool {
        self == Self::INVALID
    }

    #[doc(alias = "notify_check")]
    #[inline]
    pub fn check(self) -> Result<bool> {
        check(self)
    }

    #[doc(alias = "notify_cancel")]
    #[inline]
    pub fn cancel(self) -> Result {
        cancel(self)
    }

    #[doc(alias = "notify_suspend")]
    #[inline]
    pub fn suspend(self) -> Result {
        suspend(self)
    }

    #[doc(alias = "notify_resume")]
    #[inline]
    pub fn resume(self) -> Result {
        resume(self)
    }

    #[doc(alias = "notify_set_state")]
    #[inline]
    pub fn set_state(self, state: u64) -> Result {
        set_state(self, state)
    }

    #[doc(alias = "notify_get_state")]
    #[inline]
    pub fn state(self) -> Result<u64> {
        state(self)
    }

    #[doc(alias = "notify_is_valid_token")]
    #[inline]
    pub fn is_valid(self) -> bool {
        is_valid_token(self)
    }
}

#[cfg(all(feature = "blocks", feature = "dispatch"))]
#[doc(alias = "notify_handler_t")]
pub type Handler = blocks::EscBlock<fn(token: Token)>;

#[inline]
fn result_init<T>(op: impl FnOnce(*mut T) -> Status) -> Result<T> {
    let mut val = MaybeUninit::<T>::uninit();
    op(val.as_mut_ptr()).result()?;
    Ok(unsafe { val.assume_init() })
}

/// Post a notification for a name.
#[doc(alias = "notify_post")]
#[inline]
pub fn post(name: impl AsRef<CStr>) -> Result {
    unsafe { notify_post(name.as_ref().as_ptr()) }.result()
}

/// Create a token for use with [`Token::check`].
#[doc(alias = "notify_register_check")]
#[inline]
pub fn register_check(name: impl AsRef<CStr>) -> Result<Token> {
    result_init(|out_token| unsafe { notify_register_check(name.as_ref().as_ptr(), out_token) })
}

/// Request notification delivery by UNIX signal.
#[doc(alias = "notify_register_signal")]
#[inline]
pub fn register_signal(name: impl AsRef<CStr>, sig: c_int) -> Result<Token> {
    result_init(|out_token| unsafe {
        notify_register_signal(name.as_ref().as_ptr(), sig, out_token)
    })
}

/// Request notification delivery by Mach message.
#[doc(alias = "notify_register_mach_port")]
#[inline]
pub fn register_mach_port(
    name: impl AsRef<CStr>,
    notify_port: &mut mach::Port,
    flags: RegFlags,
) -> Result<Token> {
    result_init(|out_token| unsafe {
        notify_register_mach_port(name.as_ref().as_ptr(), notify_port, flags.0, out_token)
    })
}

/// Request notification delivery by a write to a file descriptor.
#[doc(alias = "notify_register_file_descriptor")]
#[inline]
pub fn register_file_descriptor(
    name: impl AsRef<CStr>,
    notify_fd: &mut RawFd,
    flags: RegFlags,
) -> Result<Token> {
    result_init(|out_token| unsafe {
        notify_register_file_descriptor(name.as_ref().as_ptr(), notify_fd, flags.0, out_token)
    })
}

#[cfg(all(feature = "blocks", feature = "dispatch"))]
#[doc(alias = "notify_register_dispatch")]
#[inline]
pub fn register_dispatch_block(
    name: impl AsRef<CStr>,
    queue: &dispatch::Queue,
    handler: &mut Handler,
) -> Result<Token> {
    result_init(|out_token| {
        register_dispatch_available(name.as_ref().as_ptr(), out_token, queue, handler)
    })
}

#[cfg(all(feature = "blocks", feature = "dispatch"))]
#[inline]
pub fn register_dispatch(
    name: impl AsRef<CStr>,
    queue: &dispatch::Queue,
    handler: impl FnMut(Token) + 'static,
) -> Result<Token> {
    let mut handler = Handler::new1(handler);
    register_dispatch_block(name, queue, &mut handler)
}

/// Check if any notifications have been posted for a token.
#[doc(alias = "notify_check")]
#[inline]
pub fn check(token: Token) -> Result<bool> {
    let mut check = 0;
    unsafe { notify_check(token.0, &mut check) }.result()?;
    Ok(check != 0)
}

/// Cancel notification delivery and free resources associated with a token.
#[doc(alias = "notify_cancel")]
#[inline]
pub fn cancel(token: Token) -> Result {
    unsafe { notify_cancel(token.0) }.result()
}

/// Suspend notification delivery for a token.
#[doc(alias = "notify_suspend")]
#[inline]
pub fn suspend(token: Token) -> Result {
    suspend_available(token.0).result()
}

/// Resume notification delivery for a token.
#[doc(alias = "notify_resume")]
#[inline]
pub fn resume(token: Token) -> Result {
    resume_available(token.0).result()
}

/// Set the 64-bit state value associated with a token.
#[doc(alias = "notify_set_state")]
#[inline]
pub fn set_state(token: Token, state: u64) -> Result {
    set_state_available(token.0, state).result()
}

/// Get the 64-bit state value associated with a token.
#[doc(alias = "notify_get_state")]
#[inline]
pub fn state(token: Token) -> Result<u64> {
    result_init(|state| get_state_available(token.0, state))
}

/// Determine whether a token is currently registered.
#[doc(alias = "notify_is_valid_token")]
#[inline]
pub fn is_valid_token(val: Token) -> bool {
    is_valid_token_available(val.0)
}

#[cfg(all(feature = "blocks", feature = "dispatch"))]
#[inline]
fn register_dispatch_available(
    name: *const c_char,
    out_token: *mut Token,
    queue: &dispatch::Queue,
    handler: &mut Handler,
) -> Status {
    unsafe { notify_register_dispatch(name, out_token, queue, handler) }
}

#[inline]
fn suspend_available(token: c_int) -> Status {
    unsafe { notify_suspend(token) }
}

#[inline]
fn resume_available(token: c_int) -> Status {
    unsafe { notify_resume(token) }
}

#[inline]
fn set_state_available(token: c_int, state64: u64) -> Status {
    unsafe { notify_set_state(token, state64) }
}

#[inline]
fn get_state_available(token: c_int, state64: *mut u64) -> Status {
    unsafe { notify_get_state(token, state64) }
}

#[inline]
fn is_valid_token_available(val: c_int) -> bool {
    unsafe { notify_is_valid_token(val) }
}

unsafe extern "C-unwind" {
    fn notify_post(name: *const c_char) -> Status;

    fn notify_register_check(name: *const c_char, out_token: *mut Token) -> Status;

    fn notify_register_signal(name: *const c_char, sig: c_int, out_token: *mut Token) -> Status;

    fn notify_register_mach_port(
        name: *const c_char,
        notify_port: *mut mach::Port,
        flags: c_int,
        out_token: *mut Token,
    ) -> Status;

    fn notify_register_file_descriptor(
        name: *const c_char,
        notify_fd: *mut RawFd,
        flags: c_int,
        out_token: *mut Token,
    ) -> Status;

    fn notify_check(token: c_int, check: *mut c_int) -> Status;
    fn notify_cancel(token: c_int) -> Status;

    fn notify_suspend(token: c_int) -> Status;

    fn notify_resume(token: c_int) -> Status;

    fn notify_set_state(token: c_int, state64: u64) -> Status;

    fn notify_get_state(token: c_int, state64: *mut u64) -> Status;

    fn notify_is_valid_token(val: c_int) -> bool;
}

#[cfg(all(feature = "blocks", feature = "dispatch"))]
unsafe extern "C-unwind" {
    fn notify_register_dispatch(
        name: *const c_char,
        out_token: *mut Token,
        queue: &dispatch::Queue,
        handler: &mut Handler,
    ) -> Status;
}

#[cfg(test)]
mod tests {
    use std::{ffi::CString, time::SystemTime};

    use crate::notify;

    #[test]
    fn basics() {
        let suffix = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let name = CString::new(format!("cidre.notify.{}.{}", std::process::id(), suffix)).unwrap();

        let token = notify::register_check(&name).unwrap();
        assert!(!token.is_invalid());

        token.set_state(42).unwrap();
        assert_eq!(token.state().unwrap(), 42);

        notify::post(&name).unwrap();

        token.cancel().unwrap();
        assert!(!token.is_valid());
    }
}
