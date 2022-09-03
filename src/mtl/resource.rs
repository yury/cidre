use crate::{define_mtl, define_obj_type, objc::Id};

#[repr(transparent)]
pub struct Options(usize);

pub const CPU_CACHE_MODE_SHIFT: usize = 0;
pub const CPU_CACHE_MODE_MASK: usize = 0xfusize << CPU_CACHE_MODE_SHIFT;

pub const STORAGE_MODE_SHIFT: usize = 4;
pub const STORAGE_MODE_MASK: usize = 0xfusize << STORAGE_MODE_SHIFT;

pub const HAZARD_TRACKING_MODE_SHIFT: usize = 8;
pub const HAZARD_TRACKING_MODE_MASK: usize = 0x3usize << HAZARD_TRACKING_MODE_SHIFT;

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
    MemoryLess = 3,
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

impl Options {
    pub const CPU_CACHE_MODE_DEFAULT: Self =
        Self((CPUCacheMode::DefaultCache as usize) << CPU_CACHE_MODE_SHIFT);
}

impl Default for Options {
    fn default() -> Self {
        Options(0)
    }
}
// typedef NS_OPTIONS(NSUInteger, MTLResourceOptions)
// {
//     MTLResourceCPUCacheModeDefaultCache  = MTLCPUCacheModeDefaultCache  << MTLResourceCPUCacheModeShift,
//     MTLResourceCPUCacheModeWriteCombined = MTLCPUCacheModeWriteCombined << MTLResourceCPUCacheModeShift,

//     MTLResourceStorageModeShared API_AVAILABLE(macos(10.11), ios(9.0)) = MTLStorageModeShared << MTLResourceStorageModeShift,
//     MTLResourceStorageModeManaged API_AVAILABLE(macos(10.11), macCatalyst(13.0)) API_UNAVAILABLE(ios) = MTLStorageModeManaged << MTLResourceStorageModeShift,
//     MTLResourceStorageModePrivate API_AVAILABLE(macos(10.11), ios(9.0)) = MTLStorageModePrivate << MTLResourceStorageModeShift,
//     MTLResourceStorageModeMemoryless API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(10.0)) = MTLStorageModeMemoryless << MTLResourceStorageModeShift,

//     MTLResourceHazardTrackingModeDefault API_AVAILABLE(macos(10.13), ios(10.0)) = MTLHazardTrackingModeDefault << MTLResourceHazardTrackingModeShift,
//     MTLResourceHazardTrackingModeUntracked API_AVAILABLE(macos(10.13), ios(10.0)) = MTLHazardTrackingModeUntracked << MTLResourceHazardTrackingModeShift,
//     MTLResourceHazardTrackingModeTracked API_AVAILABLE(macos(10.15), ios(13.0)) = MTLHazardTrackingModeTracked << MTLResourceHazardTrackingModeShift,

//     // Deprecated spellings
//     MTLResourceOptionCPUCacheModeDefault       = MTLResourceCPUCacheModeDefaultCache,
//     MTLResourceOptionCPUCacheModeWriteCombined = MTLResourceCPUCacheModeWriteCombined,
// } API_AVAILABLE(macos(10.11), ios(8.0));

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
