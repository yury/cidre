use crate::{define_opts, os};
use std::{ffi, os::fd::AsRawFd};

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
    pub control_flags: CtrlFlags,

    /// Local flags
    pub local_flags: LocalFlags,

    /// Control chars
    pub ctrl_chars: [Cc; NCCS],

    /// Input speed
    input_speed: BaudRate,

    /// Output speed
    output_speed: BaudRate,
}

define_opts!(
    #[doc(alias = "tcflag_t")]
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
    #[doc(alias = "tcflag_t")]
    pub OutputFlags(ffi::c_ulong)
);

define_opts!(
    #[doc(alias = "tcflag_t")]
    pub CtrlFlags(ffi::c_ulong)
);

impl CtrlFlags {
    /// Ignore control flags
    #[doc(alias = "CIGNORE")]
    pub const IGNORE: Self = Self(0x00000001);
}

define_opts!(
    pub LocalFlags(ffi::c_ulong)
);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[doc(alias = "speed_t")]
#[repr(transparent)]
pub struct BaudRate(pub ffi::c_ulong);

impl BaudRate {
    pub const _0: Self = Self(0);
    pub const _50: Self = Self(50);
    pub const _75: Self = Self(75);
    pub const _110: Self = Self(110);
    pub const _134: Self = Self(134);
    pub const _150: Self = Self(150);
    pub const _200: Self = Self(200);
    pub const _300: Self = Self(300);
    pub const _600: Self = Self(600);
    pub const _1200: Self = Self(1200);
    pub const _1800: Self = Self(1800);
    pub const _2400: Self = Self(2400);
    pub const _4800: Self = Self(4800);
    pub const _7200: Self = Self(7200);
    pub const _9600: Self = Self(9600);
    pub const _14400: Self = Self(14400);
    pub const _19200: Self = Self(19200);
    pub const _28800: Self = Self(28800);
    pub const _38400: Self = Self(38400);
    pub const _57600: Self = Self(57600);
    pub const _76800: Self = Self(76800);
    pub const _115200: Self = Self(115200);
    pub const _230400: Self = Self(230400);
}

impl LocalFlags {
    /// Enable echoing
    pub const ECHO: Self = Self(0x00000008);

    /// echo NL even if ECHO is off
    pub const ECHO_NL: Self = Self(0x00000010);

    /// echo control chars as ^(Char)
    pub const ECHO_CTRL: Self = Self(0x00000040);

    pub fn set_echo(&mut self, on: bool) {
        self.set(Self::ECHO, on)
    }

    pub fn set_echo_nl(&mut self, on: bool) {
        self.set(Self::ECHO_NL, on)
    }

    pub fn set_echo_ctrl(&mut self, on: bool) {
        self.set(Self::ECHO_CTRL, on)
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
    #[doc(alias = "tcgetattr")]
    #[inline]
    pub fn read<Fd: AsRawFd>(fd: &Fd) -> os::Result<Self> {
        os::result_init(|res| unsafe { tcgetattr(fd.as_raw_fd(), res) })
    }

    #[doc(alias = "tcsetattr")]
    #[inline]
    pub fn apply<Fd: AsRawFd>(&self, fd: &mut Fd, action: SetArg) -> os::Result {
        unsafe { tcsetattr(fd.as_raw_fd(), action as _, self).result() }
    }

    #[doc(alias = "tcsetattr")]
    #[inline]
    pub fn apply_now<Fd: AsRawFd>(&self, fd: &mut Fd) -> os::Result {
        self.apply(fd, SetArg::Now)
    }

    #[doc(alias = "cfgetospeed")]
    #[inline]
    pub fn output_speed(&self) -> BaudRate {
        unsafe { cfgetospeed(self) }
    }

    #[doc(alias = "cfgetispeed")]
    #[inline]
    pub fn input_speed(&self) -> BaudRate {
        unsafe { cfgetispeed(self) }
    }

    #[doc(alias = "cfsetospeed")]
    #[inline]
    pub fn set_output_speed(&mut self, val: BaudRate) -> os::Result {
        unsafe { cfsetospeed(self, val).result() }
    }

    #[doc(alias = "cfsetispeed")]
    #[inline]
    pub fn set_input_speed(&mut self, val: BaudRate) -> os::Result {
        unsafe { cfsetispeed(self, val).result() }
    }
}

extern "C-unwind" {
    fn tcgetattr(fd: ffi::c_int, value: *mut Termios) -> os::Status;
    fn tcsetattr(fd: ffi::c_int, action: ffi::c_int, value: *const Termios) -> os::Status;

    fn cfgetispeed(termios: *const Termios) -> BaudRate;
    fn cfgetospeed(termios: *const Termios) -> BaudRate;

    fn cfsetispeed(termios: *mut Termios, val: BaudRate) -> os::Status;
    fn cfsetospeed(termios: *mut Termios, val: BaudRate) -> os::Status;
}

#[cfg(test)]
mod tests {
    use crate::sys::termios as t;

    #[test]
    fn basics() {
        let mut fd = std::io::stdin();
        let mut cfg = t::Termios::read(&fd).unwrap();
        let original_cfg = cfg.clone();
        cfg.local_flags ^= t::LocalFlags::ECHO;
        cfg.apply_now(&mut fd).unwrap();

        cfg.set_input_speed(t::BaudRate::_0).unwrap();
        assert_eq!(cfg.input_speed, t::BaudRate::_0);

        original_cfg.apply_now(&mut fd).unwrap();
    }
}
