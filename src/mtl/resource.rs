use crate::{define_mtl, define_obj_type, objc::Id};

#[repr(transparent)]
pub struct Options(usize);

pub const CPU_CACHE_MODE_SHIFT: usize = 0;
pub const CPU_CACHE_MODE_MASK: usize = 0xfusize << CPU_CACHE_MODE_SHIFT;

pub const STORAGE_MODE_SHIFT: usize = 4;
pub const STORAGE_MODE_MASK: usize = 0xfusize << STORAGE_MODE_SHIFT;

pub const HAZARD_TRACKING_MODE_SHIFT: usize = 8;
pub const HAZARD_TRACKING_MODE_MASK: usize = 0x3usize << HAZARD_TRACKING_MODE_SHIFT;

#[repr(usize)]
pub enum CPUCacheMode {
    DefaultCache = 0,
    WriteCombined = 1,
}

#[repr(usize)]
pub enum StorageMode {
    Shared = 0,
    Managed = 1,
    Private = 2,
    MemoryLess = 3,
}

#[repr(usize)]
pub enum HazardTrackingMode {
    Default = 0,
    Untracked = 1,
    Tracked = 2,
}

impl Options {
    pub const CPU_CACHE_MODE_DEFAULT: Self =
        Self((CPUCacheMode::DefaultCache as usize) << CPU_CACHE_MODE_SHIFT);
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
    define_mtl!(device, mut label);

    #[inline]
    pub fn resource_options(&self) -> Options {
        unsafe { rsel_resourceOptions(self) }
    }

    #[inline]
    pub fn cpu_cache_mode(&self) -> CPUCacheMode {
        unsafe { rsel_cpuCacheMode(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_resourceOptions(id: &Id) -> Options;
    fn rsel_cpuCacheMode(id: &Id) -> CPUCacheMode;
}
