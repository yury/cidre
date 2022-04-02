

// KERN_SUCCESS

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
#[repr(transparent)]
pub struct KernReturn(pub i32);

impl KernReturn {
    pub const SUCCESS:Self = Self(0); 
    pub const INVALID_ADDRESS: Self = Self(1);
    pub const PROTECTION_FAILURE: Self = Self(2);
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

    #[inline]
    pub fn is_ok(&self) -> bool {
      *self == Self::SUCCESS
    }
}