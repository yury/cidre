use std::ffi::{c_uint, c_void};

use crate::{
    define_opts,
    mach::{self, Boolean, Integer, KernReturn, Natural, Port, PortName},
    os,
};

pub type Number = Natural;

// https://web.mit.edu/darwin/src/modules/xnu/osfmk/man/mach_msg.html

define_opts!(pub HeaderBits(u32));
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

    pub const fn with_ports(remote: TypeName, local: TypeName, voucher: TypeName) -> Self {
        Self(
            (remote as u32 & Self::REMOTE_MASK.0)
                | (((local as u32) << 8) & Self::LOCAL_MASK.0)
                | (((voucher as u32) << 16) & Self::VOUCHER_MASK.0),
        )
    }

    pub const fn with(remote: TypeName, local: TypeName, voucher: TypeName, other: Self) -> Self {
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

    /// The message will carry a receive right, and the caller must supply a receive right.
    /// The caller loses the supplied receive right, but retains any send rights with the same name.
    /// The make-send count and sequence number of the receive right are reset to zero and
    /// no-more-senders notification requests are cancelled (with a send-once notification
    /// being sent to the no-more-senders notification right), but the port retains other attributes
    /// like queued messages and extant send and send-once rights. If a message carries a send
    /// or send-once right, and the port dies while the message is in transit, then the receiving
    ///  task will get MACH_PORT_DEAD instead of a right.
    MoveRecieve = 16,

    /// The message will carry a send right, and the caller must supply a send right.
    /// The user reference count for the supplied send right is decremented, and
    /// the right is destroyed if the count becomes zero. Unless a receive right remains,
    /// the name becomes available for recycling. The caller may also supply a dead name,
    /// which loses a user reference, and the receiving task will get MACH_PORT_DEAD.
    MoveSend = 17,

    /// The message will carry a send-once right, and the caller must supply a send-once right.
    /// The caller loses the supplied send-once right. The caller may also supply a dead name,
    ///  which loses a user reference, and the receiving task will get MACH_PORT_DEAD.
    MoveSendOnce = 18,

    /// The message will carry a send right, and the caller must supply a send right.
    /// The user reference count for the supplied send right is not changed. The caller may also
    /// supply a dead name and the receiving task will get MACH_PORT_DEAD.
    CopySend = 19,

    /// The message will carry a send right, but the caller must supply a receive right.
    /// The send right is created from the receive right, and the receive right's make-send count
    /// is incremented.
    MakeSend = 20,

    /// The message will carry a send-once right, but the caller must supply a receive right.
    /// The send-once right is created from the receive right. Note that send once rights can
    /// only be created from the receive right.
    MakeSendOnce = 21,
    CopyReceive = 22,
    DisposeReceive = 24,
    DisposeSend = 25,
    DisposeSendOnce = 26,
}

impl TypeName {
    /// This value is an alias for MACH_MSG_TYPE_MOVE_RECEIVE.
    /// The message carried a receive right. If the receiving task already has send rights for the port,
    /// then that name for the port will be reused; otherwise, the right will have a new,
    /// previously unused name.
    pub const PORT_RECEIVE: Self = Self::MoveRecieve;

    /// This value is an alias for MACH_MSG_TYPE_MOVE_SEND.
    /// The message carried a send right. If the receiving task already has send and/ or receive rights
    /// for the port, then that name for the port will be reused. Otherwise, the right will have a new,
    /// previously unused, name. If the task already has send rights, it gains a user reference
    /// for the right (un- less this would cause the user-reference count to overflow).
    /// Otherwise, it acquires send rights, with a user-reference count of one.
    pub const PORT_SEND: Self = Self::MoveSend;

    /// This value is an alias for MACH_MSG_TYPE_MOVE_SEND_ONCE. The message carried a send-once right.
    /// The right will have a new, previously unused, name.
    pub const PORT_SEND_ONCE: Self = Self::MoveSendOnce;
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum CopyOpts {
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
pub enum DescType {
    Port,

    /// Out of line
    Ool,
    OolPorts,
    OolVolatile,
    GuardedPort,
}

impl DescType {
    pub const MAX: Self = Self::GuardedPort;
}

#[doc(alias = "mach_msg_type_descriptor_t")]
#[repr(C, packed(4))]
pub struct TypeDesc {
    pub pad1: Natural,
    pub pad2: Size,
    pub pad3: [u8; 3],
    pub type_: DescType,
}

#[doc(alias = "mach_msg_port_descriptor_t")]
#[repr(C, packed(4))]
pub struct PortDesc {
    pub name: Port,
    pub pad1: Size,
    pub pad2: u16,
    pub disposition: TypeName,
    pub type_: DescType,
}

#[doc(alias = "mach_msg_ool_descriptor32_t")]
#[repr(C, packed(4))]
pub struct OolDesc32 {
    pub address: u32,
    pub size: Size,
    pub deallocate: Boolean,
    pub pad1: u32,
    pub type_: DescType,
}

#[doc(alias = "mach_msg_ool_descriptor64_t")]
#[repr(C, packed(4))]
pub struct OolDesc64 {
    pub address: u64,
    pub size: Size,
    pub deallocate: Boolean,
    pub pad1: u32,
    pub type_: DescType,
}

#[doc(alias = "mach_msg_ool_descriptor_t")]
#[repr(C, packed(4))]
pub struct OolDesc {
    pub address: *mut c_void,
    pub deallocate: u8,
    pub copy: CopyOpts,
    pub pad1: u8,
    pub type_: DescType,
    pub size: Size,
}

#[derive(Debug, Copy, Clone)]
#[doc(alias = "mach_msg_body_t")]
#[repr(C, packed(4))]
pub struct Body {
    pub descriptor_count: Size,
}

#[derive(Debug, Copy, Clone)]
#[doc(alias = "mach_msg_header_t")]
#[repr(C, packed(4))]
pub struct Header {
    pub bits: HeaderBits,
    pub size: Size,
    pub remote_port: Port,
    pub local_port: Port,
    pub voucher_port: PortName,
    pub id: Id,
}

#[doc(alias = "mach_msg_base_t")]
#[repr(C, packed(4))]
pub struct Base {
    pub header: Header,
    pub body: Body,
}

#[doc(alias = "mach_msg_trailer_size_t")]
pub type TrailerSize = u32;

#[derive(Debug, Copy, Clone)]
#[doc(alias = "mach_msg_trailer_type_t")]
#[repr(u32)]
pub enum TrailerType {
    Format0,
}

#[derive(Debug, Copy, Clone)]
#[doc(alias = "mach_msg_trailer_t")]
#[repr(C, packed(4))]
pub struct Trailer {
    pub type_: TrailerType,
    pub size: TrailerSize,
}

/// Check errors with mach::msg_err
pub type Return = KernReturn;

pub mod err {
    use crate::os::Error;

    #[doc(alias = "MACH_MSG_MASK")]
    pub const MASK: Error = Error::new_unchecked(0x00003e00);

    /// No room in IPC name space for another capability name.
    #[doc(alias = "MACH_MSG_IPC_SPACE")]
    pub const IPC_SPACE: Error = Error::new_unchecked(0x00002000);

    ///  No room in VM address space for out-of-line memory.
    #[doc(alias = "MACH_MSG_VM_SPACE")]
    pub const VM_SPACE: Error = Error::new_unchecked(0x00001000);

    /// Kernel resource shortage handling an IPC capability.
    #[doc(alias = "MACH_MSG_IPC_KERNEL")]
    pub const IPC_KERNEL: Error = Error::new_unchecked(0x00000800);

    /// Kernel resource shortage handling out-of-line memory.
    #[doc(alias = "MACH_MSG_VM_KERNEL")]
    pub const VM_KERNEL: Error = Error::new_unchecked(0x00000400);

    /// Thread is waiting to send.  (Internal use only.)
    #[doc(alias = "MACH_SEND_IN_PROGRESS")]
    pub const SEND_IN_PROGRESS: Error = Error::new_unchecked(0x10000001);

    /// Bogus in-line data.
    #[doc(alias = "MACH_SEND_INVALID_DATA")]
    pub const SEND_INVALID_DATA: Error = Error::new_unchecked(0x10000002);

    /// Bogus destination port.
    #[doc(alias = "MACH_SEND_INVALID_DEST")]
    pub const SEND_INVALID_DEST: Error = Error::new_unchecked(0x10000003);

    /// Message not sent before timeout expired.
    #[doc(alias = "MACH_SEND_TIMED_OUT")]
    pub const SEND_TIMED_OUT: Error = Error::new_unchecked(0x10000004);

    /// Bogus voucher port.
    #[doc(alias = "MACH_SEND_INVALID_VOUCHER")]
    pub const SEND_INVALID_VOUCHER: Error = Error::new_unchecked(0x10000005);

    /// Software interrupt.
    #[doc(alias = "MACH_SEND_INTERRUPTED")]
    pub const SEND_INTERRUPTED: Error = Error::new_unchecked(0x10000007);

    /// Data doesn't contain a complete message.
    #[doc(alias = "MACH_SEND_MSG_TOO_SMALL")]
    pub const SEND_SEND_MSG_TOO_SMALL: Error = Error::new_unchecked(0x10000008);

    ///  Bogus reply port.
    #[doc(alias = "MACH_SEND_INVALID_REPLY")]
    pub const SEND_INVALID_REPLY: Error = Error::new_unchecked(0x10000009);

    /// Bogus port rights in the message body.
    #[doc(alias = "MACH_SEND_INVALID_RIGHT")]
    pub const SEND_INVALID_RIGHT: Error = Error::new_unchecked(0x1000000a);

    /// Bogus notify port argument.
    #[doc(alias = "MACH_SEND_INVALID_NOTIFY")]
    pub const SEND_INVALID_NOTIFY: Error = Error::new_unchecked(0x1000000b);

    /// Invalid out-of-line memory pointer.
    #[doc(alias = "MACH_SEND_INVALID_MEMORY")]
    pub const SEND_INVALID_MEMORY: Error = Error::new_unchecked(0x1000000c);

    /// No message buffer is available.
    #[doc(alias = "MACH_SEND_NO_BUFFER")]
    pub const SEND_NO_BUFFER: Error = Error::new_unchecked(0x1000000d);

    /// Send is too large for port
    #[doc(alias = "MACH_SEND_TOO_LARGE")]
    pub const SEND_TOO_LARGE: Error = Error::new_unchecked(0x1000000e);

    /// Invalid msg-type specification.
    #[doc(alias = "MACH_SEND_INVALID_TYPE")]
    pub const SEND_INVALID_TYPE: Error = Error::new_unchecked(0x1000000f);

    /// A field in the header had a bad value.
    #[doc(alias = "MACH_SEND_INVALID_HEADER")]
    pub const SEND_INVALID_HEADER: Error = Error::new_unchecked(0x10000010);

    /// The trailer to be sent does not match kernel format.
    #[doc(alias = "MACH_SEND_INVALID_TRAILER")]
    pub const SEND_INVALID_TRAILER: Error = Error::new_unchecked(0x10000011);

    /// The sending thread context did not match the context on the dest port
    #[doc(alias = "MACH_SEND_INVALID_CONTEXT")]
    pub const SEND_INVALID_CONTEXT: Error = Error::new_unchecked(0x10000012);

    /// Send options are invalid.
    #[doc(alias = "MACH_SEND_INVALID_OPTIONS")]
    pub const SEND_INVALID_OPTIONS: Error = Error::new_unchecked(0x10000013);

    /// Compatibility: no longer a returned error
    #[doc(alias = "MACH_SEND_INVALID_RT_OOL_SIZE")]
    pub const SEND_INVALID_RT_OOL_SIZE: Error = Error::new_unchecked(0x10000015);

    /// The destination port doesn't accept ports in body
    #[doc(alias = "MACH_SEND_NO_GRANT_DEST")]
    pub const SEND_NO_GRANT_DEST: Error = Error::new_unchecked(0x10000016);

    /// Message send was rejected by message filter
    #[doc(alias = "MACH_SEND_MSG_FILTERED")]
    pub const SEND_MSG_FILTERED: Error = Error::new_unchecked(0x10000017);

    /// Message auxiliary data is too small
    #[doc(alias = "MACH_SEND_AUX_TOO_SMALL")]
    pub const SEND_AUX_TOO_SMALL: Error = Error::new_unchecked(0x10000018);

    /// Message auxiliary data is too large
    #[doc(alias = "MACH_SEND_AUX_TOO_LARGE")]
    pub const SEND_SEND_AUX_TOO_LARGE: Error = Error::new_unchecked(0x10000019);

    /// Thread is waiting for receive. (Internal use only.)
    #[doc(alias = "MACH_RCV_IN_PROGRESS")]
    pub const RCV_IN_PROGRESS: Error = Error::new_unchecked(0x10004001);

    /// Bogus name for receive port/port-set.
    #[doc(alias = "MACH_RCV_INVALID_NAME")]
    pub const RCV_INVALID_NAME: Error = Error::new_unchecked(0x10004002);

    /// Didn't get a message within the timeout value.
    #[doc(alias = "MACH_RCV_TIMED_OUT")]
    pub const RCV_TIMED_OUT: Error = Error::new_unchecked(0x10004003);

    /// Message buffer is not large enough for inline data.
    #[doc(alias = "MACH_RCV_TOO_LARGE")]
    pub const RCV_TOO_LARGE: Error = Error::new_unchecked(0x10004004);

    /// Software interrupt.    
    #[doc(alias = "MACH_RCV_INTERRUPTED")]
    pub const RCV_INTERRUPTED: Error = Error::new_unchecked(0x10004005);

    /// compatibility: no longer a returned error
    #[doc(alias = "MACH_RCV_PORT_CHANGED")]
    pub const RCV_PORT_CHANGED: Error = Error::new_unchecked(0x10004006);

    /// Bogus notify port argument.
    #[doc(alias = "MACH_RCV_INVALID_NOTIFY")]
    pub const RCV_INVALID_NOTIFY: Error = Error::new_unchecked(0x10004007);

    /// Bogus message buffer for inline data.
    #[doc(alias = "MACH_RCV_INVALID_DATA")]
    pub const RCV_INVALID_DATA: Error = Error::new_unchecked(0x10004008);

    /// Port/set was sent away/died during receive.
    #[doc(alias = "MACH_RCV_PORT_DIED")]
    pub const RCV_PORT_DIED: Error = Error::new_unchecked(0x10004009);

    /// compatibility: no longer a returned error
    #[doc(alias = "MACH_RCV_IN_SET")]
    pub const RCV_IN_SET: Error = Error::new_unchecked(0x1000400a);

    /// Error receiving message header.  See special bits.
    #[doc(alias = "MACH_RCV_HEADER_ERROR")]
    pub const RCV_HEADER_ERROR: Error = Error::new_unchecked(0x1000400b);

    /// Error receiving message body.  See special bits.
    #[doc(alias = "MACH_RCV_BODY_ERROR")]
    pub const RCV_BODY_ERROR: Error = Error::new_unchecked(0x1000400c);

    /// Invalid msg-type specification in scatter list.
    #[doc(alias = "MACH_RCV_INVALID_TYPE")]
    pub const RCV_INVALID_TYPE: Error = Error::new_unchecked(0x1000400d);

    /// Out-of-line overwrite region is not large enough
    #[doc(alias = "MACH_RCV_SCATTER_SMALL")]
    pub const RCV_SCATTER_SMALL: Error = Error::new_unchecked(0x1000400e);

    /// trailer type or number of trailer elements not supported
    #[doc(alias = "MACH_RCV_INVALID_TRAILER")]
    pub const RCV_INVALID_TRAILER: Error = Error::new_unchecked(0x1000400f);

    /// Waiting for receive with timeout. (Internal use only.)
    #[doc(alias = "MACH_RCV_IN_PROGRESS_TIMED")]
    pub const RCV_IN_PROGRESS_TIMED: Error = Error::new_unchecked(0x10004011);

    /// invalid reply port used in a STRICT_REPLY message    
    #[doc(alias = "MACH_RCV_INVALID_REPLY")]
    pub const RCV_INVALID_REPLY: Error = Error::new_unchecked(0x10004012);

    /// invalid receive arguments, receive has not started
    #[doc(alias = "MACH_RCV_INVALID_ARGUMENTS")]
    pub const RCV_INVALID_ARGUMENTS: Error = Error::new_unchecked(0x10004013);
}

#[doc(alias = "mach_msg_empty_send_t")]
#[derive(Debug, Copy, Clone)]
#[repr(C, packed(4))]
pub struct MsgEmptySend {
    pub header: Header,
}

impl MsgEmptySend {
    pub const fn with_remote_port(remote_port: mach::Port) -> Self {
        Self {
            header: Header {
                bits: HeaderBits::with_ports(TypeName::CopySend, TypeName::None, TypeName::None),
                size: std::mem::size_of::<Header>() as u32,
                remote_port: remote_port,
                local_port: mach::Port::NULL,
                voucher_port: mach::Port::NULL,
                id: 0,
            },
        }
    }

    pub fn send(&mut self) -> os::Result {
        msg(
            &mut self.header,
            MsgOpt::SEND_MSG,
            std::mem::size_of::<Self>() as u32,
            0,
            mach::Port::NULL,
            Timeout::NONE,
            mach::Port::NULL,
        )
        .result()
    }

    pub fn send_to_remote(remote_port: mach::Port) -> os::Result {
        Self::with_remote_port(remote_port).send()
    }
}

#[doc(alias = "mach_msg_empty_rcv_t")]
#[derive(Debug, Copy, Clone)]
#[repr(C, packed(4))]
pub struct MsgEmptyRcv {
    pub header: Header,
    pub trailer: Trailer,
}

#[doc(alias = "mach_msg_empty_t")]
#[repr(C, packed(4))]
pub union MsgEmpty {
    pub send: MsgEmptySend,
    pub rcv: MsgEmptyRcv,
}

define_opts!(
    #[doc(alias = "mach_msg_option_t")]
    pub MsgOpt(Integer)
);

impl MsgOpt {
    #[doc(alias = "MACH_MSG_OPTION_NONE")]
    pub const NONE: Self = Self(0x00000000);

    #[doc(alias = "MACH_SEND_MSG")]
    pub const SEND_MSG: Self = Self(0x00000001);

    #[doc(alias = "MACH_RCV_MSG")]
    pub const RCV_MSG: Self = Self(0x00000002);

    /// report large message sizes
    #[doc(alias = "MACH_RCV_LARGE")]
    pub const RCV_LARGE: Self = Self(0x00000004);

    /// identify source of large messages
    #[doc(alias = "MACH_RCV_LARGE_IDENTITY")]
    pub const RCV_LARGE_IDENTITY: Self = Self(0x00000008);

    /// timeout value applies to send
    #[doc(alias = "MACH_SEND_TIMEOUT")]
    pub const SEND_TIMEOUT: Self = Self(0x00000010);

    /// priority override for send
    #[doc(alias = "MACH_SEND_OVERRIDE")]
    pub const SEND_OVERRIDE: Self = Self(0x00000020);

    /// don't restart interrupted sends
    #[doc(alias = "MACH_SEND_INTERRUPT")]
    pub const SEND_INTERRUPT: Self = Self(0x00000040);

    /// arm send-possible notify
    #[doc(alias = "MACH_SEND_NOTIFY")]
    pub const SEND_NOTIFY: Self = Self(0x00000080);

    /// ignore qlimits - kernel only
    #[doc(alias = "MACH_SEND_ALWAYS")]
    pub const SEND_ALWAYS: Self = Self(0x00010000);

    /// rejection by message filter should return failure - user only
    #[doc(alias = "MACH_SEND_FILTER_NONFATAL")]
    pub const SEND_FILTER_NONFATA: Self = Self(0x00010000);

    /// sender-provided trailer
    #[doc(alias = "MACH_SEND_TRAILER")]
    pub const SEND_TRAILER: Self = Self(0x00020000);

    ///  msg won't carry importance
    #[doc(alias = "MACH_SEND_NOIMPORTANCE")]
    pub const SEND_NOIMPORTANCE: Self = Self(0x00040000);

    #[doc(alias = "MACH_SEND_NODENAP")]
    pub const SEND_NODENAP: Self = Self::SEND_NOIMPORTANCE;

    /// msg carries importance - kernel only
    #[doc(alias = "MACH_SEND_IMPORTANCE")]
    pub const SEND_IMPORTANCE: Self = Self(0x00080000);

    /// msg should do sync IPC override (on legacy kernels)
    #[doc(alias = "MACH_SEND_SYNC_OVERRIDE")]
    pub const SEND_SYNC_OVERRIDE: Self = Self(0x00100000);

    /// IPC should propagate the caller's QoS
    #[doc(alias = "MACH_SEND_PROPAGATE_QOS")]
    pub const SEND_PROPAGATE_QOS: Self = Self(0x00200000);

    // spub const SEND_SYNC_USE_THRPRI: Self = Self::SEND_PROPAGATE_QOS;

    /// full send from kernel space - kernel only
    #[doc(alias = "MACH_SEND_KERNEL")]
    pub const SEND_KERNEL: Self = Self(0x00400000);

    /// special reply port should boost thread doing sync bootstrap checkin
    #[doc(alias = "MACH_SEND_SYNC_BOOTSTRAP_CHECKIN")]
    pub const SEND_SYNC_BOOTSTRAP_CHECKIN: Self = Self(0x00800000);

    /// timeout value applies to receive
    #[doc(alias = "MACH_RCV_TIMEOUT")]
    pub const RCV_TIMEOUT: Self = Self(0x00000100);

    /// legacy name (value was: 0x00000200)
    #[doc(alias = "MACH_RCV_NOTIFY")]
    pub const RCV_NOTIFY: Self = Self(0x00000000);

    /// don't restart interrupted receive
    #[doc(alias = "MACH_RCV_INTERRUPT")]
    pub const RCV_INTERRUPT: Self = Self(0x00000400);

    /// willing to receive voucher port     
    #[doc(alias = "MACH_RCV_VOUCHER")]
    pub const RCV_VOUCHER: Self = Self(0x00000800);

    // scatter receive (deprecated)
    // pub const RCV_OVERWRITE: Self = Self(0x00000000);

    /// Can receive new guarded descriptor
    #[doc(alias = "MACH_RCV_GUARDED_DESC")]
    pub const RCV_GUARDED_DESC: Self = Self(0x00001000);

    /// sync waiter waiting for rcv
    #[doc(alias = "MACH_RCV_SYNC_WAIT")]
    pub const RCV_SYNC_WAIT: Self = Self(0x00004000);

    /// sync waiter waiting to peek
    #[doc = "MACH_RCV_SYNC_PEEK"]
    pub const RCV_SYNC_PEEK: Self = Self(0x00008000);

    /// Enforce specific properties about the reply port, and
    /// the context in which a thread replies to a message
    /// This flag must be passed on both the SEND and RCV
    #[doc = "MACH_MSG_STRICT_REPLY"]
    pub const MSG_STRICT_REPLY: Self = Self(0x00000200);
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct Timeout(pub Natural);

impl Timeout {
    pub const NONE: Self = Self(0);
}

#[doc(alias = "mach_msg")]
#[inline]
pub fn msg(
    msg: &mut Header,
    option: MsgOpt,
    send_size: Size,
    rcv_size: Size,
    rcv_name: PortName,
    timeout: Timeout,
    notify: PortName,
) -> Return {
    unsafe { mach_msg(msg, option, send_size, rcv_size, rcv_name, timeout, notify) }
}

#[doc(alias = "mach_msg_overwrite")]
#[inline]
pub fn msg_overwrite(
    msg: *mut Header,
    option: MsgOpt,
    send_size: Size,
    rcv_size: Size,
    rcv_name: PortName,
    timeout: Timeout,
    notify: PortName,
    rcv_msg: *mut Header,
    rcv_limit: Size,
) -> Return {
    unsafe {
        mach_msg_overwrite(
            msg, option, send_size, rcv_size, rcv_name, timeout, notify, rcv_msg, rcv_limit,
        )
    }
}

unsafe extern "C-unwind" {
    fn mach_msg(
        msg: *mut Header,
        option: MsgOpt,
        send_size: Size,
        rcv_size: Size,
        rcv_name: PortName,
        timeout: Timeout,
        notify: PortName,
    ) -> Return;

    fn mach_msg_overwrite(
        msg: *mut Header,
        option: MsgOpt,
        send_size: Size,
        rcv_size: Size,
        rcv_name: PortName,
        timeout: Timeout,
        notify: PortName,
        rcv_msg: *mut Header,
        rcv_limit: Size,
    ) -> Return;
}
