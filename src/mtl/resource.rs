use std::mem::transmute;

use crate::{define_mtl, define_obj_type, define_options, objc::Id};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum PurgableState {
    /// The purgeabelity state is not changed.
    KeepCurrent = 1,
    /// The contents of this resource may not be discarded.
    NoneVolatile = 2,
    /// The contents of this resource may be discarded.
    Volatile = 3,
    /// The contents of this are discarded.
    Empty = 4,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum CPUCacheMode {
    DefaultCache = 0,
    WriteCombined = 1,
}

/// Describes location and CPU mapping of MTLTexture.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum StorageMode {
    /// In this mode, CPU and device will nominally both use the same underlying memory
    /// when accessing the contents of the texture resource.
    /// However, coherency is only guaranteed at command buffer boundaries to minimize
    /// the required flushing of CPU and GPU caches.
    /// This is the default storage mode for iOS Textures.
    Shared = 0,
    /// This mode relaxes the coherency requirements and requires that the developer make explicit requests to maintain
    /// coherency between a CPU and GPU version of the texture resource.  In order for CPU to access up to date GPU results,
    /// first, a blit synchronizations must be completed (see synchronize methods of mtl::BlitCommandEncoder).
    /// Blit overhead is only incurred if GPU has modified the resource.
    /// This is the default storage mode for OS X Textures.
    Managed = 1,
    /// This mode allows the texture resource data to be kept entirely to GPU (or driver) private memory
    /// that will never be accessed by the CPU directly, so no
    /// conherency of any kind must be maintained.
    Private = 2,
    /// his mode allows creation of resources that do not have a GPU or CPU memory backing, but do have on-chip storage for TBDR
    /// devices. The contents of the on-chip storage is undefined and does not persist, but its configuration is controlled by the
    /// mtl::Texture descriptor. Textures created with MTLStorageModeMemoryless dont have an IOAccelResource at any point in their
    /// lifetime. The only way to populate such resource is to perform rendering operations on it. Blit operations are disallowed.
    Memoryless = 3,
}

/// Describes how hazard tracking is performed.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum HazardTrackingMode {
    /// The default hazard tracking mode for the context. Refer to the usage of the field for semantics.
    Default = 0,
    /// Do not perform hazard tracking.
    Untracked = 1,
    /// Do perform hazard tracking.
    Tracked = 2,
}

impl Default for HazardTrackingMode {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}

define_options!(Options(usize));

pub const CPU_CACHE_MODE_SHIFT: usize = 0;
pub const CPU_CACHE_MODE_MASK: usize = 0xfusize << CPU_CACHE_MODE_SHIFT;

pub const STORAGE_MODE_SHIFT: usize = 4;
pub const STORAGE_MODE_MASK: usize = 0xfusize << STORAGE_MODE_SHIFT;

pub const HAZARD_TRACKING_MODE_SHIFT: usize = 8;
pub const HAZARD_TRACKING_MODE_MASK: usize = 0x3usize << HAZARD_TRACKING_MODE_SHIFT;

/// A set of optional arguments to influence the creation of a mtl::Resource (mtl::Texture or mtl::Buffer)
impl Options {
    pub const CPU_CACHE_MODE_DEFAULT: Self =
        Self((CPUCacheMode::DefaultCache as usize) << CPU_CACHE_MODE_SHIFT);

    pub const CPU_CACHE_MODE_WRITE_COMBINED: Self =
        Self((CPUCacheMode::WriteCombined as usize) << CPU_CACHE_MODE_SHIFT);

    pub const STORAGE_MODE_SHARED: Self =
        Self((StorageMode::Shared as usize) << STORAGE_MODE_SHIFT);

    pub const STORAGE_MODE_MANAGED: Self =
        Self((StorageMode::Managed as usize) << STORAGE_MODE_SHIFT);

    pub const STORAGE_MODE_PRIVATE: Self =
        Self((StorageMode::Private as usize) << STORAGE_MODE_SHIFT);

    pub const STORAGE_MODE_MEMORYLESS: Self =
        Self((StorageMode::Memoryless as usize) << STORAGE_MODE_SHIFT);

    pub const HAZARD_TRACKING_MODE_DEFAULT: Self =
        Self((HazardTrackingMode::Default as usize) << HAZARD_TRACKING_MODE_SHIFT);

    pub const HAZARD_TRACKING_MODE_UNTRACKED: Self =
        Self((HazardTrackingMode::Untracked as usize) << HAZARD_TRACKING_MODE_SHIFT);

    pub const HAZARD_TRACKING_MODE_TRACKED: Self =
        Self((HazardTrackingMode::Tracked as usize) << HAZARD_TRACKING_MODE_SHIFT);

    #[inline]
    pub fn hazard_tracking_mode(&self) -> HazardTrackingMode {
        unsafe { transmute((self.0 & HAZARD_TRACKING_MODE_MASK) >> HAZARD_TRACKING_MODE_SHIFT) }
    }

    #[inline]
    pub fn storage_mode(&self) -> StorageMode {
        unsafe { transmute((self.0 & STORAGE_MODE_MASK) >> STORAGE_MODE_SHIFT) }
    }

    #[inline]
    pub fn cpu_cache_mode(&self) -> CPUCacheMode {
        unsafe { transmute(self.0 & CPU_CACHE_MODE_MASK) }
    }
}

define_obj_type!(Resource(Id));

impl Resource {
    define_mtl!(
        device,
        label,
        set_label,
        cpu_cache_mode,
        storage_mode,
        hazard_tracking_mode,
        resource_options
    );
}

#[cfg(test)]
mod tests {
    use crate::mtl::{CPUCacheMode, StorageMode};

    use super::{HazardTrackingMode, Options};

    #[test]
    fn options_default() {
        let opts = Options::default();
        assert_eq!(opts.cpu_cache_mode(), CPUCacheMode::DefaultCache);
        assert_eq!(opts.storage_mode(), StorageMode::Shared);
        assert_eq!(opts.hazard_tracking_mode(), HazardTrackingMode::Default);
    }

    #[test]
    fn options_non_default() {
        let opts = Options::STORAGE_MODE_MEMORYLESS
            | Options::CPU_CACHE_MODE_WRITE_COMBINED
            | Options::HAZARD_TRACKING_MODE_UNTRACKED;

        assert_eq!(opts.cpu_cache_mode(), CPUCacheMode::WriteCombined);
        assert_eq!(opts.storage_mode(), StorageMode::Memoryless);
        assert_eq!(opts.hazard_tracking_mode(), HazardTrackingMode::Untracked);
    }
}
