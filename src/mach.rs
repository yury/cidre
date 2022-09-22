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
pub use message::Base as MsgBase;
pub use message::Body as MsgBody;
pub use message::CopyOptions as MsgCopyOptions;
pub use message::DescriptorType as MsgDescriptorType;
pub use message::GuardFlags as MsgGuardFlags;
pub use message::Header as MsgHeader;
pub use message::MsgSize;
pub use message::PortDescriptor as MsgPortDescriptor;
pub use message::Return as MsgReturn;
pub use message::TypeDescriptor as MsgTypeDescriptor;

pub fn msg_receive(header: &mut MsgHeader) -> MsgReturn {
    unsafe { mach_msg_receive(header) }
}

pub fn msg_send(header: &MsgHeader) -> MsgReturn {
    unsafe { mach_msg_send(header) }
}

extern "C" {
    fn mach_msg_receive(header: &mut MsgHeader) -> MsgReturn;
    fn mach_msg_send(header: &MsgHeader) -> MsgReturn;
}
