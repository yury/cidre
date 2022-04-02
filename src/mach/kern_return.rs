// KERN_SUCCESS

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct KernReturn(pub i32);

impl KernReturn {
    pub const SUCCESS: Self = Self(0);
    /// Specified address is not currently valid.
    pub const INVALID_ADDRESS: Self = Self(1);
    /// Specified memory is valid, but does not permit the required forms of access.
    pub const PROTECTION_FAILURE: Self = Self(2);
    /// The address range specified is already in use, or no address range of the size specified could be found.
    pub const NO_SPACE: Self = Self(3);
    pub const INVALID_ARGUMENT: Self = Self(4);
    pub const FAILURE: Self = Self(5);
    pub const RESOURCE_SHORTAGE: Self = Self(6);
    pub const NOT_RECEIVER: Self = Self(7);
    pub const NO_ACCESS: Self = Self(8);
    pub const MEMORY_FAILURE: Self = Self(9);
    pub const MEMORY_ERROR: Self = Self(10);
    pub const ALREADY_IN_SET: Self = Self(11);
    pub const NOT_IN_SET: Self = Self(12);
    pub const NAME_EXISTS: Self = Self(13);
    pub const ABORTED: Self = Self(14);
    pub const INVALID_NAME: Self = Self(15);
    pub const INVALID_TASK: Self = Self(16);
    pub const INVALID_RIGHT: Self = Self(17);
    pub const INVALID_VALUE: Self = Self(18);
    pub const UREFS_OVERFLOW: Self = Self(19);
    pub const INVALID_CAPABILITY: Self = Self(20);
    pub const RIGHT_EXISTS: Self = Self(21);
    pub const INVALID_HOST: Self = Self(22);
    pub const MEMORY_PRESENT: Self = Self(23);
    pub const MEMORY_DATA_MOVED: Self = Self(24);
    pub const MEMORY_RESTART_COPY: Self = Self(25);
    pub const INVALID_PROCESSOR_SET: Self = Self(26);
    pub const POLICY_LIMIT: Self = Self(27);
    pub const INVALID_POLICY: Self = Self(28);
    pub const INVALID_OBJECT: Self = Self(29);
    pub const ALREADY_WAITING: Self = Self(30);
    pub const DEFAULT_SET: Self = Self(31);
    pub const EXCEPTION_PROTECTED: Self = Self(32);
    pub const INVALID_LEDGER: Self = Self(33);
    pub const INVALID_MEMORY_CONTROL: Self = Self(34);
    pub const INVALID_SECURITY: Self = Self(35);
    pub const NOT_DEPRESSED: Self = Self(36);
    pub const TERMINATED: Self = Self(37);
    pub const LOCK_SET_DESTROYED: Self = Self(38);
    pub const LOCK_UNSTABLE: Self = Self(39);
    pub const LOCK_OWNED: Self = Self(40);
    pub const LOCK_OWNED_SELF: Self = Self(41);
    pub const SEMAPHORE_DESTROYED: Self = Self(42);
    pub const RPC_SERVER_TERMINATED: Self = Self(43);
    pub const RPC_TERMINATE_ORPHAN: Self = Self(44);
    pub const RPC_CONTINUE_ORPHAN: Self = Self(45);
    pub const NOT_SUPPORTED: Self = Self(46);
    pub const NODE_DOWN: Self = Self(47);
    pub const NOT_WAITING: Self = Self(48);
    pub const OPERATION_TIMED_OUT: Self = Self(49);
    pub const CODESIGN_ERROR: Self = Self(50);
    pub const POLICY_STATIC: Self = Self(51);
    pub const INSUFFICIENT_BUFFER_SIZE: Self = Self(52);
    pub const DENIED: Self = Self(53);
    pub const MISSING_KC: Self = Self(54);
    pub const INVALID_KC: Self = Self(55);
    pub const NOT_FOUND: Self = Self(56);
    pub const RETURN_MAX: Self = Self(0x100);

    #[inline]
    pub fn is_ok(&self) -> bool {
        *self == Self::SUCCESS
    }
}
