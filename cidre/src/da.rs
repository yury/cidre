use crate::{cf, define_opts};

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
extern "C" {
    static kDADiskDescriptionMatchMediaUnformatted: &'static cf::DictionaryOf<cf::String, cf::Type>;
    static kDADiskDescriptionMatchMediaWhole: &'static cf::DictionaryOf<cf::String, cf::Type>;
    static kDADiskDescriptionMatchVolumeMountable: &'static cf::DictionaryOf<cf::String, cf::Type>;
    static kDADiskDescriptionMatchVolumeUnrecognized:
        &'static cf::DictionaryOf<cf::String, cf::Type>;
    static kDADiskDescriptionWatchVolumeName: &'static cf::ArrayOf<cf::String>;
    static kDADiskDescriptionWatchVolumePath: &'static cf::ArrayOf<cf::String>;
}
