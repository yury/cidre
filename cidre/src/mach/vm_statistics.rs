use crate::define_options;

define_options!(pub VMAllocationFlags(i32));

impl VMAllocationFlags {
    /// Allocate new VM region at the specified virtual address, if possible.
    pub const FIXED: Self = Self(0x00000000);

    /// Allocate new VM region anywhere it would fit in the address space.
    pub const ANYWHERE: Self = Self(0x00000001);

    /// Create a purgable VM object for that new VM region.
    pub const PURGABLE: Self = Self(0x00000002);

    /// The new VM region will be chunked up into 4GB sized pieces.
    pub const _4GB_CHUNK: Self = Self(0x00000004);

    pub const RANDOM_ADDR: Self = Self(0x00000008);

    /// Pages brought in to this VM region are placed on the speculativeA
    /// queue instead of the active queue.  In other words, they are not
    /// cached so that they will be stolen first if memory runs low.
    pub const NO_CACHE: Self = Self(0x00000010);
    pub const RESILIENT_CODESIGN: Self = Self(0x00000020);
    pub const RESILIENT_MEDIA: Self = Self(0x00000040);
    pub const PERMANENT: Self = Self(0x00001000);

    /// The new VM region can replace existing VM regions if necessary
    /// (to be used in combination with VM_FLAGS_FIXED).
    pub const VM_FLAGS_OVERWRITE: Self = Self(0x00004000);

    pub const fn make_tag(tag: i32) -> Self {
        Self(tag << 24)
    }
}

pub const VM_MEMORY_MACH_MSG: i32 = 20;
pub const VM_MEMORY_SHARED_PMAP: i32 = 32;
