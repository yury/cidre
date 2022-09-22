use std::os::raw::{c_int, c_uint};

use crate::define_options;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Port(pub c_uint);

impl Port {
    pub fn task_self_deallocate(self) -> c_int {
        unsafe { mach_port_deallocate(mach_task_self_, self) }
    }

    pub fn current_task() -> Self {
        unsafe { mach_task_self_ }
    }

    pub const NULL: Self = Self(0);
    pub const DEAD: Self = Self(!0);
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct Right(pub u32);

impl Right {
    pub const SEND: Self = Self(0);
    pub const RECEIVE: Self = Self(1);
    pub const SEND_ONCE: Self = Self(2);
    pub const PORT_SET: Self = Self(3);
    pub const DEAD_NAME: Self = Self(4);
    pub const LABEL_H: Self = Self(5);
    pub const NUMBER: Self = Self(6);
}

define_options!(Type(u32));

impl Type {
    pub const NONE: Self = Self(0);
    pub const SEND: Self = Self::new(Right::SEND);
    pub const RECEIVE: Self = Self::new(Right::RECEIVE);
    pub const SEND_ONCE: Self = Self::new(Right::SEND_ONCE);
    pub const PORT_SET: Self = Self::new(Right::PORT_SET);
    pub const DEAD_NAME: Self = Self::new(Right::DEAD_NAME);
    pub const LABELH: Self = Self::new(Right::LABEL_H);

    // Convenient combinations

    pub const SEND_RECEIVE: Self = Self(Self::SEND.0 | Self::RECEIVE.0);
    pub const SEND_RIGHTS: Self = Self(Self::SEND.0 | Self::SEND_ONCE.0);
    pub const PORT_RIGHTS: Self = Self(Self::SEND_RIGHTS.0 | Self::RECEIVE.0);
    pub const PORT_OR_DEAD: Self = Self(Self::PORT_RIGHTS.0 | Self::DEAD_NAME.0);
    pub const ALL_RIGHTS: Self = Self(Self::PORT_OR_DEAD.0 | Self::PORT_SET.0);

    pub const DNREQUEST: Self = Self(0x80000000);
    pub const SPREQUEST: Self = Self(0x40000000);
    pub const SPREQUEST_DELAYED: Self = Self(0x20000000);

    pub const fn new(right: Right) -> Self {
        Self(1 << ((right.0) + 16))
    }
}

extern "C" {
    static mach_task_self_: Port;

    fn mach_port_deallocate(task: Port, port: Port) -> c_int;

}
