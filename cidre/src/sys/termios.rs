use crate::{define_opts, os};
use std::{ffi, os::fd::AsRawFd};

pub type Speed = std::ffi::c_ulong;
pub type Cc = std::ffi::c_uchar;

pub const NCCS: usize = 20;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Termios {
    /// Input flags
    pub input_flags: InputFlags,

    /// Output flags
    pub output_flags: OutputFlags,

    /// Control flags
    pub control_flags: ControlFlags,

    /// Local flags
    pub local_flags: LocalFlags,

    /// Control chars
    pub control_chars: [Cc; NCCS],

    /// Input speed
    pub input_speed: Speed,

    /// Output speed
    pub output_speed: Speed,
}

define_opts!(
    pub InputFlags(ffi::c_ulong)
);

impl InputFlags {
    /// Ignore BREAK condition
    #[doc(alias = "IGNBRK")]
    pub const IGNORE_BREAK: Self = Self(0x00000001);

    /// Map BREAK to SIGINTR
    #[doc(alias = "BRKINT")]
    pub const MAP_BREAK_TO_SIGNIT: Self = Self(0x00000002);

    /// Ignore (discard) parity errors
    #[doc(alias = "IGNPAR")]
    pub const IGNORE_PARITY_ERRORS: Self = Self(0x00000004);

    /// Mark parity and framing errors
    #[doc(alias = "PARMRK")]
    pub const PARITY_MARK: Self = Self(0x00000008);

    /// Enable checking of parity errors
    #[doc(alias = "INPCK")]
    pub const PARITY_CHECK: Self = Self(0x00000010);

    /// Strip 8th bit off chars
    #[doc(alias = "ISTRIP")]
    pub const STRIP: Self = Self(0x00000020);

    /// Map NL into CR
    #[doc(alias = "INLCR")]
    pub const MAP_NL_TO_CR: Self = Self(0x00000040);

    /// Ignore CR
    #[doc(alias = "IGNCR")]
    pub const IGNORE_CR: Self = Self(0x00000080);
}

define_opts!(
    pub OutputFlags(ffi::c_ulong)
);

define_opts!(
    pub ControlFlags(ffi::c_ulong)
);

define_opts!(
    pub LocalFlags(ffi::c_ulong)
);

impl LocalFlags {
    pub const ECHO: Self = Self(0x00000008);

    pub fn set_echo(&mut self, val: bool) {
        if val {
            *self |= Self::ECHO
        } else {
            *self ^= Self::ECHO
        }
    }
}

/// Commands passed to tcsetattr() for setting the termios structure.
#[repr(i32)]
#[non_exhaustive]
pub enum SetArg {
    #[doc(alias = "TCSANOW")]
    Now,

    #[doc(alias = "TCSADRAIN")]
    Drain,

    #[doc(alias = "TCSAFLUSH")]
    Flush,
}

#[repr(i32)]
#[non_exhaustive]
pub enum FlushArg {
    Input,
    Output,
    Io,
}

impl Termios {
    pub fn read<Fd: AsRawFd>(fd: Fd) -> os::Result<Self> {
        os::result_init(|res| unsafe { tcgetattr(fd.as_raw_fd(), res) })
    }

    pub fn apply<Fd: AsRawFd>(&self, fd: Fd, action: SetArg) -> os::Result {
        unsafe { tcsetattr(fd.as_raw_fd(), action as _, self).result() }
    }

    pub fn apply_now<Fd: AsRawFd>(&self, fd: Fd) -> os::Result {
        self.apply(fd, SetArg::Now)
    }
}

extern "C-unwind" {
    fn tcgetattr(fd: ffi::c_int, value: *mut Termios) -> os::Status;
    fn tcsetattr(fd: ffi::c_int, action: ffi::c_int, value: *const Termios) -> os::Status;
}

#[cfg(test)]
mod tests {
    use crate::sys::termios::{LocalFlags, Termios};

    #[test]
    fn basics() {
        let mut cfg = Termios::read(std::io::stdin()).unwrap();
        cfg.local_flags ^= LocalFlags::ECHO;
        cfg.apply_now(std::io::stdin()).unwrap();
    }
}
