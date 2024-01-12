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
