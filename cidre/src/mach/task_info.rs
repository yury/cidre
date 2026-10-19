use crate::mach;

#[doc(alias = "mach_task_basic_info")]
#[derive(Debug)]
#[repr(C)]
pub struct TaskBasicInfo {
    /// Virtual memory size (bytes)
    pub virtual_size: mach::VmSize,

    /// Resident memory size (bytes)
    pub resident_size: mach::VmSize,

    /// Maximum resident memory size (bytes)
    pub resident_size_max: mach::VmSize,

    /// Total user run time for terminated threads
    pub user_time: mach::TimeValue,

    /// Total system run time for terminated threads
    pub system_time: mach::TimeValue,

    /// Default policy for new threads
    pub policy: mach::Policy,

    /// Suspend count for task
    pub suspend_count: mach::Integer,
}

impl TaskBasicInfo {
    pub const fn flavor() -> TaskFlavor {
        TaskFlavor(20)
    }

    pub const fn count() -> mach::message::Number {
        (std::mem::size_of::<Self>() / std::mem::size_of::<mach::Natural>()) as _
    }
}

#[repr(transparent)]
pub struct TaskFlavor(pub mach::Natural);

/// What `task_info` tells of a task's memory (`TASK_VM_INFO`, revision 7). `phys_footprint`
/// is the figure Activity Monitor's "Memory" and the jetsam limits go by.
#[doc(alias = "task_vm_info")]
#[doc(alias = "task_vm_info_data_t")]
#[derive(Debug, Default, Clone, Copy)]
#[repr(C, packed(4))]
pub struct TaskVmInfo {
    /// Virtual memory size (bytes)
    pub virtual_size: u64,
    /// Number of memory regions
    pub region_count: mach::Integer,
    pub page_size: mach::Integer,
    /// Resident memory size (bytes)
    pub resident_size: u64,
    /// Peak resident size (bytes)
    pub resident_size_peak: u64,

    pub device: u64,
    pub device_peak: u64,
    pub internal: u64,
    pub internal_peak: u64,
    pub external: u64,
    pub external_peak: u64,
    pub reusable: u64,
    pub reusable_peak: u64,
    pub purgeable_volatile_pmap: u64,
    pub purgeable_volatile_resident: u64,
    pub purgeable_volatile_virtual: u64,
    pub compressed: u64,
    pub compressed_peak: u64,
    pub compressed_lifetime: u64,

    // rev1
    pub phys_footprint: u64,

    // rev2
    pub min_address: u64,
    pub max_address: u64,

    // rev3
    pub ledger_phys_footprint_peak: i64,
    pub ledger_purgeable_nonvolatile: i64,
    pub ledger_purgeable_novolatile_compressed: i64,
    pub ledger_purgeable_volatile: i64,
    pub ledger_purgeable_volatile_compressed: i64,
    pub ledger_tag_network_nonvolatile: i64,
    pub ledger_tag_network_nonvolatile_compressed: i64,
    pub ledger_tag_network_volatile: i64,
    pub ledger_tag_network_volatile_compressed: i64,
    pub ledger_tag_media_footprint: i64,
    pub ledger_tag_media_footprint_compressed: i64,
    pub ledger_tag_media_nofootprint: i64,
    pub ledger_tag_media_nofootprint_compressed: i64,
    /// Graphics memory (IOSurfaces, Metal resources) in the footprint (bytes)
    pub ledger_tag_graphics_footprint: i64,
    pub ledger_tag_graphics_footprint_compressed: i64,
    pub ledger_tag_graphics_nofootprint: i64,
    pub ledger_tag_graphics_nofootprint_compressed: i64,
    pub ledger_tag_neural_footprint: i64,
    pub ledger_tag_neural_footprint_compressed: i64,
    pub ledger_tag_neural_nofootprint: i64,
    pub ledger_tag_neural_nofootprint_compressed: i64,

    // rev4
    pub limit_bytes_remaining: u64,

    // rev5
    pub decompressions: mach::Integer,

    // rev6
    pub ledger_swapins: i64,

    // rev7
    pub ledger_tag_neural_nofootprint_total: i64,
    pub ledger_tag_neural_nofootprint_peak: i64,
}

impl TaskVmInfo {
    pub const fn flavor() -> TaskFlavor {
        TaskFlavor(22)
    }

    pub const fn count() -> mach::message::Number {
        (std::mem::size_of::<Self>() / std::mem::size_of::<mach::Natural>()) as _
    }
}
