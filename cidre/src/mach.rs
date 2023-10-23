pub mod boolean;
pub use boolean::Boolean;

pub mod time;
pub use time::absolute_time;
pub use time::approximate_time;
pub use time::continuous_approximate_time;
pub use time::continuous_time;
pub use time::TimeBaseInfo;

pub mod kern_return;
pub use kern_return::KernReturn;

pub mod port;
pub use port::Name as PortName;
pub use port::Port;
pub use port::Right as PortRight;
pub use port::Type as PortType;

pub mod message;
pub use message::msg;
pub use message::Base as MsgBase;
pub use message::Body as MsgBody;
pub use message::CopyOptions as MsgCopyOptions;
pub use message::DescType as MsgDescType;
pub use message::GuardFlags as MsgGuardFlags;
pub use message::Header as MsgHeader;
pub use message::HeaderBits as MsgHBits;
pub use message::Id as MsgId;
pub use message::MsgOption;
pub use message::OOLDesc as MsgOOLDesc;
pub use message::PortDesc as MsgPortDesc;
pub use message::Priority as MsgPriority;
pub use message::Return as MsgReturn;
pub use message::Size as MsgSize;
pub use message::Timeout as MsgTimeout;
pub use message::Trailer as MsgTrailer;
pub use message::TrailerSize as MsgTrailerSize;
pub use message::TrailerType as MsgTrailerType;
pub use message::TypeDesc as MsgTypeDesc;
pub use message::TypeName as MsgTypeName;

pub mod vm_types;
pub use vm_types::Address as VMAddress;
pub use vm_types::Integer;
pub use vm_types::Map as VMMap;
pub use vm_types::MapInspect as VMMapInspect;
pub use vm_types::MapRead as VMMapRead;
pub use vm_types::Natural;
pub use vm_types::Offset as VMOffset;
pub use vm_types::Size as VMSize;

pub mod vm_map;

pub mod vm_statistics;
pub use vm_statistics::VMAllocationFlags;

pub mod task;

pub mod task_info;
pub use task_info::TaskBasicInfo;
pub use task_info::TaskFlavor;

pub mod time_value;
pub use time_value::TimeValue;

pub mod policy;
pub use policy::Policy;

#[inline]
pub fn msg_receive(header: &mut MsgHeader) -> MsgReturn {
    unsafe { mach_msg_receive(header) }
}

#[inline]
pub fn msg_send(header: &mut MsgHeader) -> MsgReturn {
    unsafe { mach_msg_send(header) }
}

extern "C" {
    fn mach_msg_receive(header: &mut MsgHeader) -> MsgReturn;
    fn mach_msg_send(header: &mut MsgHeader) -> MsgReturn;
}
