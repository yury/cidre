use crate::mach;

/// The system's page counts (`HOST_VM_INFO64`, revision 1): what `vm_stat` prints.
#[doc(alias = "vm_statistics64")]
#[doc(alias = "vm_statistics64_data_t")]
#[derive(Debug, Default, Clone, Copy)]
#[repr(C)]
pub struct VmStatistics64 {
    /// Pages free (speculative ones included)
    pub free_count: mach::Natural,
    pub active_count: mach::Natural,
    pub inactive_count: mach::Natural,
    /// Pages wired down
    pub wire_count: mach::Natural,
    pub zero_fill_count: u64,
    pub reactivations: u64,
    /// Pageins (lifetime)
    pub pageins: u64,
    pub pageouts: u64,
    pub faults: u64,
    pub cow_faults: u64,
    pub lookups: u64,
    pub hits: u64,
    /// Pages purged (lifetime): purgeable memory taken back while volatile
    pub purges: u64,
    /// Pages purgeable (volatile, can be taken back)
    pub purgeable_count: mach::Natural,
    /// Free pages holding speculatively read data
    pub speculative_count: mach::Natural,

    // rev1
    pub decompressions: u64,
    pub compressions: u64,
    pub swapins: u64,
    pub swapouts: u64,
    pub compressor_page_count: mach::Natural,
    pub throttled_count: mach::Natural,
    pub external_page_count: mach::Natural,
    pub internal_page_count: mach::Natural,
    pub total_uncompressed_pages_in_compressor: u64,
}

impl VmStatistics64 {
    pub const fn flavor() -> HostFlavor {
        HostFlavor(4)
    }

    pub const fn count() -> mach::message::Number {
        (std::mem::size_of::<Self>() / std::mem::size_of::<mach::Integer>()) as _
    }
}

#[doc(alias = "host_flavor_t")]
#[repr(transparent)]
pub struct HostFlavor(pub mach::Integer);

/// The system's page counts (`host_statistics64` with `HOST_VM_INFO64`).
pub fn vm_statistics64() -> Result<VmStatistics64, mach::KernReturn> {
    let mut info = VmStatistics64::default();
    let mut cnt = VmStatistics64::count();
    let res = unsafe {
        let host = mach_host_self();
        let res = host_statistics64(
            host,
            VmStatistics64::flavor(),
            &mut info as *mut VmStatistics64 as _,
            &mut cnt as *mut mach::message::Number,
        );
        // `mach_host_self` hands out a send right each call.
        host.task_self_deallocate();
        res
    };
    if res.is_ok() { Ok(info) } else { Err(res) }
}

/// The host's page size (`vm_page_size`).
pub fn page_size() -> usize {
    unsafe { vm_page_size }
}

unsafe extern "C-unwind" {
    fn mach_host_self() -> mach::Port;
    fn host_statistics64(
        host_priv: mach::Port,
        flavor: HostFlavor,
        host_info64_out: *mut mach::Integer,
        host_info64_out_cnt: *mut mach::message::Number,
    ) -> mach::KernReturn;

    static vm_page_size: usize;
}

#[cfg(test)]
mod tests {
    use crate::mach;

    #[test]
    fn vm_statistics64() {
        // HOST_VM_INFO64_REV1_COUNT in the SDK.
        assert_eq!(mach::host::VmStatistics64::count(), 38);
        let stats = mach::host::vm_statistics64().expect("Failed to get vm statistics");
        assert!(stats.free_count > 0);
        assert!(stats.wire_count > 0);
        assert!(mach::host::page_size() >= 4096);
    }
}
