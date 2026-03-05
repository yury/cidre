use crate::define_opts;

mod session;
pub use session::Session;

mod disk;
pub use disk::Disk;

define_opts!(
    #[doc(alias = "DADiskMountOptions")]
    pub DiskMountOpts(u32)
);

impl DiskMountOpts {
    pub const WHOLE: Self = Self(0x00000001);
}

define_opts!(
    #[doc(alias = "DADiskRenameOptions")]
    pub DiskRenameOpts(u32)
);

define_opts!(
    #[doc(alias = "DADiskUnmountOptions")]
    pub DiskUnmountOpts(u32)
);

impl DiskUnmountOpts {
    pub const FORCE: Self = Self(0x00080000);
    pub const WHOLE: Self = Self(0x00000001);
}

define_opts!(
    #[doc(alias = "DADiskEjectOptions")]
    pub DiskEjectOpts(u32)
);

define_opts!(
    #[doc(alias = "DADiskClaimOptions")]
    pub DiskClaimOpts(u32)
);

define_opts!(
    #[doc(alias = "DADiskOptions")]
    pub DiskOpts(u32)
);

#[link(name = "DiskArbitration", kind = "framework")]
unsafe extern "C" {}
