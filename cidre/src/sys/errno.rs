use crate::os::Error;

/// Operation not permitted
pub const EPERM: Error = Error::new_unchecked(1);

/// No such file or directory
pub const ENOENT: Error = Error::new_unchecked(2);

/// No such process
pub const ESRCH: Error = Error::new_unchecked(3);

///  Interrupted system call
pub const EINTR: Error = Error::new_unchecked(4);

/// Input/output error
pub const EIO: Error = Error::new_unchecked(5);

/// Device not configured
pub const ENXIO: Error = Error::new_unchecked(6);

/// Argument list too long
pub const E2BIG: Error = Error::new_unchecked(7);

/// Exec format error
pub const ENOEXEC: Error = Error::new_unchecked(8);

/// Bad file descriptor
pub const EBADF: Error = Error::new_unchecked(9);

/// No child processes
pub const ECHILD: Error = Error::new_unchecked(10);

/// Resource deadlock avoided
pub const EDEADLK: Error = Error::new_unchecked(11);

/// Cannot allocate memory
pub const ENOMEM: Error = Error::new_unchecked(12);

/// Permission denied
pub const EACCES: Error = Error::new_unchecked(13);

/// Bad address
pub const EFAULT: Error = Error::new_unchecked(14);

/// Block device required
pub const ENOTBLK: Error = Error::new_unchecked(15);

/// Device / Resource busy
pub const EBUSY: Error = Error::new_unchecked(16);

/// File exists
pub const EEXIST: Error = Error::new_unchecked(17);

/// Cross-device link
pub const EXDEV: Error = Error::new_unchecked(18);

/// Operation not supported by device
pub const ENODEV: Error = Error::new_unchecked(19);

/// Not a directory
pub const ENOTDIR: Error = Error::new_unchecked(20);

/// Is a directory
pub const EISDIR: Error = Error::new_unchecked(21);

/// Invalid argument
pub const EINVAL: Error = Error::new_unchecked(22);

/// Too many open files in system
pub const ENFILE: Error = Error::new_unchecked(23);

/// Too many open files
pub const EMFILE: Error = Error::new_unchecked(24);

/// Inappropriate ioctl for device
pub const ENOTTY: Error = Error::new_unchecked(25);

/// Text file busy
pub const ETXTBSY: Error = Error::new_unchecked(26);

/// File too large
pub const EFBIG: Error = Error::new_unchecked(27);

/// No space left on device
pub const ENOSPC: Error = Error::new_unchecked(28);

/// Illegal seek
pub const ESPIPE: Error = Error::new_unchecked(29);

/// Read-only file system
pub const EROFS: Error = Error::new_unchecked(30);

/// Too many links
pub const EMLINK: Error = Error::new_unchecked(31);

/// Broken pipe
pub const EPIPE: Error = Error::new_unchecked(32);

/// Numerical argument out of domain
pub const EDOM: Error = Error::new_unchecked(33);

/// Result too large
pub const ERANGE: Error = Error::new_unchecked(34);

/// Resource temporarily unavailable
pub const EAGAIN: Error = Error::new_unchecked(35);

/// Operation would block
pub const EWOULDBLOCK: Error = EAGAIN;

/// Operation now in progress
pub const EINPROGRESS: Error = Error::new_unchecked(36);

/// Operation already in progress
pub const EALREADY: Error = Error::new_unchecked(37);

/// Socket operation on non-socket
pub const ENOTSOCK: Error = Error::new_unchecked(38);

/// Destination address required
pub const EDESTADDRREQ: Error = Error::new_unchecked(39);

/// Message too long
pub const EMSGSIZE: Error = Error::new_unchecked(40);

/// Protocol wrong type for socket
pub const EPROTOTYPE: Error = Error::new_unchecked(41);

/// Protocol not available
pub const ENOPROTOOPT: Error = Error::new_unchecked(42);

/// Protocol not supported
pub const EPROTONOSUPPORT: Error = Error::new_unchecked(43);

/// Socket type not supported
pub const ESOCKTNOSUPPORT: Error = Error::new_unchecked(44);

/// Operation not supported
pub const ENOTSUP: Error = Error::new_unchecked(45);

/// Operation not supported on socket
pub const EOPNOTSUPP: Error = ENOTSUP;

/// Protocol family not supported
pub const EPFNOSUPPORT: Error = Error::new_unchecked(46);

/// Address family not supported by protocol family
pub const EAFNOSUPPORT: Error = Error::new_unchecked(47);

/// Address already in use
pub const EADDRINUSE: Error = Error::new_unchecked(48);

/// Can't assign requested address
pub const EADDRNOTAVAIL: Error = Error::new_unchecked(49);

/// Network is down
pub const ENETDOWN: Error = Error::new_unchecked(50);

/// Network is unreachable
pub const ENETUNREACH: Error = Error::new_unchecked(51);

/// Network dropped connection on reset
pub const ENETRESET: Error = Error::new_unchecked(52);

/// Software caused connection abort
pub const ECONNABORTED: Error = Error::new_unchecked(53);

/// Connection reset by peer
pub const ECONNRESET: Error = Error::new_unchecked(54);

/// No buffer space available
pub const ENOBUFS: Error = Error::new_unchecked(55);

/// Socket is already connected
pub const EISCONN: Error = Error::new_unchecked(56);

/// Socket is not connected
pub const ENOTCONN: Error = Error::new_unchecked(57);

/// Can't send after socket shutdown
pub const ESHUTDOWN: Error = Error::new_unchecked(58);

/// Too many references: can't splice
pub const ETOOMANYREFS: Error = Error::new_unchecked(59);

/// Operation timed out
pub const ETIMEDOUT: Error = Error::new_unchecked(60);

/// Connection refused
pub const ECONNREFUSED: Error = Error::new_unchecked(61);

/// Too many levels of symbolic links
pub const ELOOP: Error = Error::new_unchecked(62);

/// File name too long
pub const ENAMETOOLONG: Error = Error::new_unchecked(63);

/// Host is down
pub const EHOSTDOWN: Error = Error::new_unchecked(64);

/// No route to host
pub const EHOSTUNREACH: Error = Error::new_unchecked(65);

/// Directory not empty
pub const ENOTEMPTY: Error = Error::new_unchecked(66);

///  Too many processes
pub const EPROCLIM: Error = Error::new_unchecked(67);

/// Too many users
pub const EUSERS: Error = Error::new_unchecked(68);

/// Disc quota exceeded
pub const EDQUOT: Error = Error::new_unchecked(69);

/// Stale NFS file handle
pub const ESTALE: Error = Error::new_unchecked(70);

/// Too many levels of remote in path
pub const EREMOTE: Error = Error::new_unchecked(71);

/// RPC struct is bad
pub const EBADRPC: Error = Error::new_unchecked(72);

/// RPC version wrong
pub const ERPCMISMATCH: Error = Error::new_unchecked(73);

/// RPC prog. not avail
pub const EPROGUNAVAIL: Error = Error::new_unchecked(74);

/// Program version wrong
pub const EPROGMISMATCH: Error = Error::new_unchecked(75);

/// Bad procedure for program
pub const EPROCUNAVAIL: Error = Error::new_unchecked(76);

/// No locks available
pub const ENOLCK: Error = Error::new_unchecked(77);

/// Function not implemented
pub const ENOSYS: Error = Error::new_unchecked(78);

/// Inappropriate file type or format
pub const EFTYPE: Error = Error::new_unchecked(79);

/// Authentication error
pub const EAUTH: Error = Error::new_unchecked(80);

/// Need authenticator
pub const ENEEDAUTH: Error = Error::new_unchecked(81);

/// Device power is off
pub const EPWROFF: Error = Error::new_unchecked(82);

/// Device error, e.g. paper out
pub const EDEVERR: Error = Error::new_unchecked(83);

/// Value too large to be stored in data type
pub const EOVERFLOW: Error = Error::new_unchecked(84);

/// Bad executable
pub const EBADEXEC: Error = Error::new_unchecked(85);

/// Bad CPU type in executable
pub const EBADARCH: Error = Error::new_unchecked(86);

/// Shared library version mismatch
pub const ESHLIBVERS: Error = Error::new_unchecked(87);

/// Malformed Macho file
pub const EBADMACHO: Error = Error::new_unchecked(88);

/// Operation canceled
pub const ECANCELED: Error = Error::new_unchecked(89);

/// Identifier removed
pub const EIDRM: Error = Error::new_unchecked(90);

/// No message of desired type
pub const ENOMSG: Error = Error::new_unchecked(91);

/// Illegal byte sequence
pub const EILSEQ: Error = Error::new_unchecked(92);

/// Attribute not found
pub const ENOATTR: Error = Error::new_unchecked(93);

/// Bad message
pub const EBADMSG: Error = Error::new_unchecked(94);

/// Reserved
pub const EMULTIHOP: Error = Error::new_unchecked(95);

/// No message available on STREAM
pub const ENODATA: Error = Error::new_unchecked(96);

/// Reserved
pub const ENOLINK: Error = Error::new_unchecked(97);

/// No STREAM resources
pub const ENOSR: Error = Error::new_unchecked(98);

/// Not a STREAM
pub const ENOSTR: Error = Error::new_unchecked(99);

/// Protocol error
pub const EPROTO: Error = Error::new_unchecked(100);

/// STREAM ioctl timeout
pub const ETIME: Error = Error::new_unchecked(101);

// pub const EOPNOTSUPP: Error = Error::new_unchecked(102); /* Operation not supported on socket */
/// No such policy registered
pub const ENOPOLICY: Error = Error::new_unchecked(103);

/// State not recoverable
pub const ENOTRECOVERABLE: Error = Error::new_unchecked(104);

/// Previous owner died
pub const EOWNERDEAD: Error = Error::new_unchecked(105);

/// Interface output queue is full
pub const EQFULL: Error = Error::new_unchecked(106);

/// Capabilities insufficient
pub const ENOTCAPABLE: Error = Error::new_unchecked(107);

/// Must be equal largest errno
pub const ELAST: Error = Error::new_unchecked(107);
