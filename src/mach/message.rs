use super::{Boolean, Port};

pub type MsgBits = u32;
pub type MsgSize = i32;
pub type MsgId = i32;
pub type MsgPriority = u32;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum TypeName {
    MoveRecieve = 16,
    MoveSend = 17,
    MoveSendOnce = 18,
    MoveCopySend = 19,
    MakeSend = 20,
    MakeSendOnce = 21,
    CopyReceive = 22,
    DisposeReceive = 24,
    DisposeSend = 25,
    DisposeSendOnce = 26,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum CopyOptions {
    PhysicalCopy = 0,
    VirtualCopy = 1,
    Allocate = 2,
    Overwrite = 3,
    KallocCopy = 4,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum GuardFlags {
    None,
    ImmovableReceive,
    UnguardedOnSend,
}

impl GuardFlags {
    pub const MASK: u32 = 3;
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum DescriptorType {
    Port,

    /// Out of line
    OOL,
    OOLPorts,
    OOLVolatile,
    GuardedPort,
}

impl DescriptorType {
    pub const MAX: Self = Self::GuardedPort;
}

#[repr(C)]
pub struct TypeDescriptor {
    pub pad1: u32,
    pub pad2: MsgSize,
    pub pad3: u32,
    pub type_: DescriptorType,
}

#[repr(C)]
pub struct PortDescriptor {
    pub name: Port,
    pub pad1: MsgSize,
    pub pad2: u32,
    pub disposition: TypeName,
    pub type_: DescriptorType,
}

#[repr(C)]
pub struct OOLDescriptor32 {
    pub address: u32,
    pub size: MsgSize,
    pub deallocate: Boolean,
    pub pad1: u32,
    pub type_: DescriptorType,
}

#[repr(C)]
pub struct OOLDescriptor64 {
    pub address: u64,
    pub size: MsgSize,
    pub deallocate: Boolean,
    pub pad1: u32,
    pub type_: DescriptorType,
}

pub struct Header {}
