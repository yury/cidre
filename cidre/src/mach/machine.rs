/// mask for architecture bits
pub const CPU_ARCH_MASK: u32 = 0xff000000;

/// 64 bit ABI
pub const CPU_ARCH_ABI64: u32 = 0x01000000;

/// ABI for 64-bit hardware with 32-bit types; LP32
pub const CPU_ARCH_ABI64_32: u32 = 0x02000000;

#[doc(alias = "cpu_type_t")]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct CpuType(pub i32);

impl CpuType {
    #[doc(alias = "CPU_TYPE_ANY")]
    pub const ANY: Self = Self(-1);

    #[doc(alias = "CPU_TYPE_VAX")]
    pub const VAX: Self = Self(1);

    #[doc(alias = "CPU_TYPE_MC680x0")]
    pub const MC680X0: Self = Self(6);

    #[doc(alias = "CPU_TYPE_X86")]
    pub const X86: Self = Self(7);

    #[doc(alias = "CPU_TYPE_I386")]
    pub const I386: Self = Self::X86;

    #[doc(alias = "CPU_TYPE_X86_64")]
    pub const X86_64: Self = Self(Self::X86.0 | CPU_ARCH_ABI64 as i32);

    #[doc(alias = "CPU_TYPE_MC98000")]
    pub const MC98000: Self = Self(10);

    #[doc(alias = "CPU_TYPE_HPPA")]
    pub const HPPA: Self = Self(11);

    #[doc(alias = "CPU_TYPE_ARM")]
    pub const ARM: Self = Self(12);

    #[doc(alias = "CPU_TYPE_ARM64")]
    pub const ARM64: Self = Self(Self::ARM.0 | CPU_ARCH_ABI64 as i32);

    #[doc(alias = "CPU_TYPE_ARM64_32")]
    pub const ARM64_32: Self = Self(Self::ARM.0 | CPU_ARCH_ABI64_32 as i32);

    #[doc(alias = "CPU_TYPE_MC88000")]
    pub const MC88000: Self = Self(13);

    #[doc(alias = "CPU_TYPE_SPARC")]
    pub const SPARC: Self = Self(14);

    #[doc(alias = "CPU_TYPE_I860")]
    pub const I860: Self = Self(15);

    #[doc(alias = "CPU_TYPE_POWERPC")]
    pub const POWERPC: Self = Self(18);

    #[doc(alias = "CPU_TYPE_POWERPC64")]
    pub const POWERPC64: Self = Self(Self::POWERPC.0 | CPU_ARCH_ABI64 as i32);
}
