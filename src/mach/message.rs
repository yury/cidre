use std::ffi::{c_uint, c_void};

use crate::define_options;

use super::{Boolean, Integer, KernReturn, Natural, Port, PortName};

define_options!(HeaderBits(u32));
impl HeaderBits {
    pub const ZERO: Self = Self(0);

    /// The value of MACH_MSGH_BITS_REMOTE determines the interpretation
    /// of the msgh_remote_port field.  It is handled like a msgt_name,
    /// but must result in a send or send-once type right.
    pub const REMOTE_MASK: Self = Self(0x0000001f);

    /// The value of MACH_MSGH_BITS_LOCAL determines the interpretation
    /// of the msgh_local_port field.  It is handled like a msgt_name,
    /// and also must result in a send or send-once type right.
    pub const LOCAL_MASK: Self = Self(0x00001f00);

    /// The value of MACH_MSGH_BITS_VOUCHER determines the interpretation
    /// of the msgh_voucher_port field.  It is handled like a msgt_name,
    /// but must result in a send right (and the msgh_voucher_port field
    ///  must be the name of a send right to a Mach voucher kernel object.)
    pub const VOUCHER_MASK: Self = Self(0x001f0000);

    pub const PORTS_MASK: Self =
        Self(Self::REMOTE_MASK.0 | Self::LOCAL_MASK.0 | Self::VOUCHER_MASK.0);

    /// The kernel uses MACH_MSGH_BITS_COMPLEX as a hint.  If it isn't on, it
    /// assumes the body of the message doesn't contain port rights or OOL
    /// data.  The field is set in received messages.  A user task must
    /// use caution in interpreting the body of a message if the bit isn't
    /// on, because the mach_msg_type's in the body might "lie" about the
    /// contents.  If the bit isn't on, but the mach_msg_types
    /// in the body specify rights or OOL data, the behavior is undefined.
    /// (Ie, an error may or may not be produced.)
    pub const COMPLEX: Self = Self(0x80000000);
    pub const USER: Self = Self(0x801f1f1f);
    pub const RAISEIMP: Self = Self(0x20000000);
    pub const DENAP: Self = Self::RAISEIMP;
    pub const IMPHOLDASRT: Self = Self(0x10000000);
    pub const DENAPHOLDASRT: Self = Self::IMPHOLDASRT;

    /// should be zero; is is used internally.
    pub const CIRCULAR: Self = Self(0x10000000);
    pub const USED: Self = Self(0xb01f1f1f);

    pub fn with_ports(remote: TypeName, local: TypeName, voucher: TypeName) -> Self {
        Self(
            (remote as u32 & Self::REMOTE_MASK.0)
                | (((local as u32) << 8) & Self::LOCAL_MASK.0)
                | (((voucher as u32) << 16) & Self::VOUCHER_MASK.0),
        )
    }

    pub fn with(remote: TypeName, local: TypeName, voucher: TypeName, other: Self) -> Self {
        Self(Self::with_ports(remote, local, voucher).0 | (other.0 & (!Self::PORTS_MASK.0)))
    }
}

pub type Size = Natural;

pub type Id = Integer;

pub type Priority = c_uint;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum TypeName {
    None = 0,
    PortName = 15,
    MoveRecieve = 16,
    MoveSend = 17,
    MoveSendOnce = 18,
    CopySend = 19,
    MakeSend = 20,
    MakeSendOnce = 21,
    CopyReceive = 22,
    DisposeReceive = 24,
    DisposeSend = 25,
    DisposeSendOnce = 26,
}

impl TypeName {
    pub const PORT_RECEIVE: Self = Self::MoveRecieve;
    pub const PORT_SEND: Self = Self::MoveSend;
    pub const PORT_SEND_ONCE: Self = Self::MoveSendOnce;
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum CopyOptions {
    PhysicalCopy = 0,
    VirtualCopy = 1,
    Allocate = 2,
    Overwrite = 3,
    KallocCopy = 4,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u16)]
pub enum GuardFlags {
    None,
    ImmovableReceive,
    UnguardedOnSend,
}

impl GuardFlags {
    pub const MASK: u16 = 3;
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
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

#[repr(C, align(4))]
pub struct TypeDescriptor {
    pub pad1: Natural,
    pub pad2: Size,
    pub pad3: [u8; 3],
    pub type_: DescriptorType,
}

#[repr(C, align(4))]
pub struct PortDescriptor {
    pub name: Port,
    pub pad1: Size,
    pub pad2: u16,
    pub disposition: TypeName,
    pub type_: DescriptorType,
}

#[repr(C, align(4))]
pub struct OOLDescriptor32 {
    pub address: u32,
    pub size: Size,
    pub deallocate: Boolean,
    pub pad1: u32,
    pub type_: DescriptorType,
}

#[repr(C, align(4))]
pub struct OOLDescriptor64 {
    pub address: u64,
    pub size: Size,
    pub deallocate: Boolean,
    pub pad1: u32,
    pub type_: DescriptorType,
}

#[repr(C, align(4))]
pub struct OOLDescriptor {
    pub address: *mut c_void,
    pub deallocate: Boolean,
    pub copy: CopyOptions,
    pub pad1: u32,
    pub type_: DescriptorType,
    pub size: Size,
}

#[repr(C, align(4))]
pub struct Body {
    pub descriptor_count: Size,
}

#[repr(C, align(4))]
pub struct Header {
    pub bits: HeaderBits,
    pub size: Size,
    pub remote_port: Port,
    pub local_port: Port,
    pub voucher_port: PortName,
    pub id: Id,
}

#[repr(C, align(4))]
pub struct Base {
    pub header: Header,
    pub body: Body,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Return(pub KernReturn);

impl Return {
    pub const SUCCESS: Self = Self(KernReturn(0));

    pub const MASK: Self = Self(KernReturn(0x00003e00));

    /// No room in IPC name space for another capability name.
    pub const IPC_SPACE: Self = Self(KernReturn(0x00002000));

    ///  No room in VM address space for out-of-line memory.
    pub const VM_SPACE: Self = Self(KernReturn(0x00001000));

    /// Kernel resource shortage handling an IPC capability.
    pub const IPC_KERNEL: Self = Self(KernReturn(0x00000800));

    /// Kernel resource shortage handling out-of-line memory.
    pub const VM_KERNEL: Self = Self(KernReturn(0x00000400));

    /// Thread is waiting to send.  (Internal use only.)
    pub const SEND_IN_PROGRESS: Self = Self(KernReturn(0x10000001));

    /// Bogus in-line data.
    pub const SEND_INVALID_DATA: Self = Self(KernReturn(0x10000002));

    /// Bogus destination port.
    pub const SEND_INVALID_DEST: Self = Self(KernReturn(0x10000003));

    /// Message not sent before timeout expired.
    pub const SEND_TIMED_OUT: Self = Self(KernReturn(0x10000004));

    /// Bogus voucher port.
    pub const SEND_INVALID_VOUCHER: Self = Self(KernReturn(0x10000005));

    /// Software interrupt.
    pub const SEND_INTERRUPTED: Self = Self(KernReturn(0x10000007));

    /// Data doesn't contain a complete message.
    pub const SEND_SEND_MSG_TOO_SMALL: Self = Self(KernReturn(0x10000008));

    ///  Bogus reply port.
    pub const SEND_INVALID_REPLY: Self = Self(KernReturn(0x10000009));

    /// Bogus port rights in the message body.
    pub const SEND_INVALID_RIGHT: Self = Self(KernReturn(0x1000000a));

    /// Bogus notify port argument.
    pub const SEND_INVALID_NOTIFY: Self = Self(KernReturn(0x1000000b));

    /// Invalid out-of-line memory pointer.
    pub const SEND_INVALID_MEMORY: Self = Self(KernReturn(0x1000000c));

    /// No message buffer is available.
    pub const SEND_NO_BUFFER: Self = Self(KernReturn(0x1000000d));

    /// Send is too large for port
    pub const SEND_TOO_LARGE: Self = Self(KernReturn(0x1000000e));

    /// Invalid msg-type specification.
    pub const SEND_INVALID_TYPE: Self = Self(KernReturn(0x1000000f));

    ///  A field in the header had a bad value.
    pub const SEND_INVALID_HEADER: Self = Self(KernReturn(0x10000010));

    /// The trailer to be sent does not match kernel format.
    pub const SEND_INVALID_TRAILER: Self = Self(KernReturn(0x10000011));

    /// The sending thread context did not match the context on the dest port
    pub const SEND_INVALID_CONTEXT: Self = Self(KernReturn(0x10000012));

    ///  Send options are invalid.
    pub const SEND_INVALID_OPTIONS: Self = Self(KernReturn(0x10000013));

    ///  compatibility: no longer a returned error
    pub const SEND_INVALID_RT_OOL_SIZE: Self = Self(KernReturn(0x10000015));

    /// The destination port doesn't accept ports in body
    pub const SEND_NO_GRANT_DEST: Self = Self(KernReturn(0x10000016));

    /// Message send was rejected by message filter
    pub const SEND_MSG_FILTERED: Self = Self(KernReturn(0x10000017));

    /// Message auxiliary data is too small
    pub const SEND_AUX_TOO_SMALL: Self = Self(KernReturn(0x10000018));

    /// Message auxiliary data is too large
    pub const SEND_SEND_AUX_TOO_LARGE: Self = Self(KernReturn(0x10000019));

    /// Thread is waiting for receive.  (Internal use only.)
    pub const RCV_IN_PROGRESS: Self = Self(KernReturn(0x10004001));

    /// Bogus name for receive port/port-set.
    pub const RCV_INVALID_NAME: Self = Self(KernReturn(0x10004002));

    /// Didn't get a message within the timeout value.
    pub const RCV_TIMED_OUT: Self = Self(KernReturn(0x10004003));

    /// Message buffer is not large enough for inline data.
    pub const RCV_TOO_LARGE: Self = Self(KernReturn(0x10004004));

    /// Software interrupt.    
    pub const RCV_INTERRUPTED: Self = Self(KernReturn(0x10004005));

    /// compatibility: no longer a returned error
    pub const RCV_PORT_CHANGED: Self = Self(KernReturn(0x10004006));

    /// Bogus notify port argument.
    pub const RCV_INVALID_NOTIFY: Self = Self(KernReturn(0x10004007));

    /// Bogus message buffer for inline data.
    pub const RCV_INVALID_DATA: Self = Self(KernReturn(0x10004008));

    /// Port/set was sent away/died during receive.
    pub const RCV_PORT_DIED: Self = Self(KernReturn(0x10004009));

    /// compatibility: no longer a returned error
    pub const RCV_IN_SET: Self = Self(KernReturn(0x1000400a));

    /// Error receiving message header.  See special bits.
    pub const RCV_HEADER_ERROR: Self = Self(KernReturn(0x1000400b));

    /// Error receiving message body.  See special bits.
    pub const RCV_BODY_ERROR: Self = Self(KernReturn(0x1000400c));

    /// Invalid msg-type specification in scatter list.
    pub const RCV_INVALID_TYPE: Self = Self(KernReturn(0x1000400d));

    /// Out-of-line overwrite region is not large enough
    pub const RCV_SCATTER_SMALL: Self = Self(KernReturn(0x1000400e));

    /// trailer type or number of trailer elements not supported
    pub const RCV_INVALID_TRAILER: Self = Self(KernReturn(0x1000400f));

    /// Waiting for receive with timeout. (Internal use only.)
    pub const RCV_IN_PROGRESS_TIMED: Self = Self(KernReturn(0x10004011));

    /// invalid reply port used in a STRICT_REPLY message    
    pub const RCV_INVALID_REPLY: Self = Self(KernReturn(0x10004012));

    /// invalid receive arguments, receive has not started
    pub const RCV_INVALID_ARGUMENTS: Self = Self(KernReturn(0x10004013));

    pub fn is_ok(&self) -> bool {
        *self == Self::SUCCESS
    }
}

define_options!(MsgOption(Integer));

impl MsgOption {
    pub const NONE: Self = Self(0x00000000);

    pub const SEND_MSG: Self = Self(0x00000001);
    pub const RCV_MSG: Self = Self(0x00000002);

    /// report large message sizes
    pub const RCV_LARGE: Self = Self(0x00000004);

    /// identify source of large messages
    pub const RCV_LARGE_IDENTITY: Self = Self(0x00000008);

    /// timeout value applies to send
    pub const SEND_TIMEOUT: Self = Self(0x00000010);

    /// priority override for send
    pub const SEND_OVERRIDE: Self = Self(0x00000020);

    /// don't restart interrupted sends
    pub const SEND_INTERRUPT: Self = Self(0x00000040);

    /// arm send-possible notify
    pub const SEND_NOTIFY: Self = Self(0x00000080);

    /// ignore qlimits - kernel only
    pub const SEND_ALWAYS: Self = Self(0x00010000);

    /// rejection by message filter should return failure - user only
    pub const SEND_FILTER_NONFATA: Self = Self(0x00010000);

    /// sender-provided trailer
    pub const SEND_TRAILER: Self = Self(0x00020000);

    ///  msg won't carry importance
    pub const SEND_NOIMPORTANCE: Self = Self(0x00040000);

    pub const SEND_NODENAP: Self = Self::SEND_NOIMPORTANCE;

    /// msg carries importance - kernel only
    pub const SEND_IMPORTANCE: Self = Self(0x00080000);

    /// msg should do sync IPC override (on legacy kernels)
    pub const SEND_SYNC_OVERRIDE: Self = Self(0x00100000);

    /// IPC should propagate the caller's QoS
    pub const SEND_PROPAGATE_QOS: Self = Self(0x00200000);

    // spub const SEND_SYNC_USE_THRPRI: Self = Self::SEND_PROPAGATE_QOS;

    /// full send from kernel space - kernel only
    pub const SEND_KERNEL: Self = Self(0x00400000);

    /// special reply port should boost thread doing sync bootstrap checkin
    pub const SEND_SYNC_BOOTSTRAP_CHECKIN: Self = Self(0x00800000);

    /// timeout value applies to receive
    pub const RCV_TIMEOUT: Self = Self(0x00000100);

    /// legacy name (value was: 0x00000200)
    pub const RCV_NOTIFY: Self = Self(0x00000000);

    /// don't restart interrupted receive
    pub const RCV_INTERRUPT: Self = Self(0x00000400);

    /// willing to receive voucher port     
    pub const RCV_VOUCHER: Self = Self(0x00000800);

    // scatter receive (deprecated)
    // pub const RCV_OVERWRITE: Self = Self(0x00000000);

    /// Can receive new guarded descriptor
    pub const RCV_GUARDED_DESC: Self = Self(0x00001000);

    /// sync waiter waiting for rcv
    pub const RCV_SYNC_WAIT: Self = Self(0x00004000);

    /// sync waiter waiting to peek
    pub const RCV_SYNC_PEEK: Self = Self(0x00008000);

    /// Enforce specific properties about the reply port, and
    /// the context in which a thread replies to a message
    /// This flag must be passed on both the SEND and RCV
    pub const MSG_STRICT_REPLY: Self = Self(0x00000200);
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct Timeout(pub Natural);

impl Timeout {
    pub const NONE: Self = Self(0);
}

#[inline]
pub fn msg(
    msg: &mut Header,
    option: MsgOption,
    send_size: Size,
    rcv_size: Size,
    rcv_name: PortName,
    timeout: Timeout,
    notify: PortName,
) -> Return {
    unsafe { mach_msg(msg, option, send_size, rcv_size, rcv_name, timeout, notify) }
}

extern "C" {
    fn mach_msg(
        msg: &mut Header,
        option: MsgOption,
        send_size: Size,
        rcv_size: Size,
        rcv_name: PortName,
        timeout: Timeout,
        notify: PortName,
    ) -> Return;
}
