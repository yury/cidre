use std::os::raw::c_int;

use crate::define_opts;

use super::vm_map::{vm_allocate, vm_deallocate};
use super::{KernReturn, VmAddr, VmAllocationFlags, VmSize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Name(pub u32);

impl Name {
    pub const NULL: Self = Self(0);
    pub const DEAD: Self = Self(!0);
}

#[doc(alias = "mach_port_t")]
pub type Port = Name;

impl Port {
    pub fn task_self_deallocate(self) -> c_int {
        unsafe { mach_port_deallocate(mach_task_self_, self) }
    }

    #[inline]
    pub fn current_task() -> Self {
        unsafe { mach_task_self_ }
    }

    #[inline]
    pub fn allocate(
        self,
        address: *mut VmAddr,
        size: VmSize,
        flags: VmAllocationFlags,
    ) -> KernReturn {
        unsafe { vm_allocate(self, address, size, flags) }
    }

    #[inline]
    pub fn deallocate(self, address: VmAddr, size: VmSize) -> KernReturn {
        unsafe { vm_deallocate(self, address, size) }
    }
}

/// A send right this task holds, released on drop.
///
/// [`Name`] is a bare `u32` that copies freely, which is right for naming a
/// right but says nothing about who has to release one. The calls that hand a
/// send right *over* — [`crate::io::Surf::create_mach_port`],
/// [`crate::xpc::Dictionary::copy_mach_send`] — return this instead, so the
/// release is the compiler's to remember rather than the caller's.
///
/// Send rights only: [`Drop`] releases through `mach_port_deallocate`, which is
/// the send-side release. A receive right or a port set is freed differently and
/// does not belong in here.
///
/// One reference each, not one name each: a task's name space maps a name to a
/// port, so taking another right to a port you already name yields *that* name
/// again with the user reference count bumped. Two [`SendRight`]s can therefore
/// compare equal while each still owes exactly one release — which is why this
/// is not [`Copy`] and why [`Self::name`] borrows.
///
/// Nothing here is a security boundary — [`Self::into_name`] hands the raw name
/// back and [`Self::from_name`] takes one on the caller's word. The point is
/// that the ordinary path cannot leak by accident.
#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct SendRight(Name);

impl SendRight {
    /// Takes ownership of the send right `name` denotes.
    ///
    /// # Safety
    /// `name` must be a send right this task holds, and nothing else may
    /// release it — this will, exactly once.
    #[inline]
    pub const unsafe fn from_name(name: Name) -> Self {
        Self(name)
    }

    /// As [`Self::from_name`], but [`None`] for a name that holds nothing:
    /// [`Name::NULL`] or [`Name::DEAD`].
    ///
    /// # Safety
    /// As [`Self::from_name`], for any name that is neither of those.
    #[inline]
    pub unsafe fn try_from_name(name: Name) -> Option<Self> {
        match name {
            Name::NULL | Name::DEAD => None,
            name => Some(Self(name)),
        }
    }

    /// The right's name, borrowed for as long as `self` lives.
    ///
    /// For a call that reads the right or copies it — `MACH_MSG_TYPE_COPY_SEND`
    /// and friends. One that *moves* it wants [`Self::into_name`], or the right
    /// is released twice.
    #[inline]
    pub const fn name(&self) -> Name {
        self.0
    }

    /// Gives the right up without releasing it, for a call that moves it —
    /// `MACH_MSG_TYPE_MOVE_SEND`.
    #[inline]
    pub fn into_name(self) -> Name {
        let name = self.0;
        std::mem::forget(self);
        name
    }
}

impl Drop for SendRight {
    #[inline]
    fn drop(&mut self) {
        self.0.task_self_deallocate();
    }
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

define_opts!(pub Type(u32));

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

unsafe extern "C-unwind" {
    static mach_task_self_: Port;

    fn mach_port_deallocate(task: Port, port: Port) -> c_int;
}
