//! A task's virtual memory in 64-bit addresses and sizes (`<mach/mach_vm.h>`).

use crate::{mach, os};

/// An address in a task's address space.
#[doc(alias = "mach_vm_address_t")]
pub type Addr = u64;

/// A size in a task's address space.
#[doc(alias = "mach_vm_size_t")]
pub type Size = u64;

/// Copies `data.len()` bytes at `address` in `target_task` into `data`; how many it copied.
///
/// An address that is not mapped, or not readable, fails (`KERN_INVALID_ADDRESS`,
/// `KERN_PROTECTION_FAILURE`) rather than faulting: a way to probe memory, the current task's
/// ([`mach::Port::current_task`]) included.
#[doc(alias = "mach_vm_read_overwrite")]
pub fn read_overwrite(
    target_task: mach::VmMapRead,
    address: Addr,
    data: &mut [u8],
) -> os::Result<usize> {
    let mut copied: Size = 0;
    unsafe {
        mach_vm_read_overwrite(
            target_task,
            address,
            data.len() as Size,
            data.as_mut_ptr() as Addr,
            &mut copied,
        )
    }
    .result()?;
    Ok(copied as usize)
}

unsafe extern "C-unwind" {
    /// Copies `size` bytes at `address` in `target_task` to `data` in the current task.
    pub fn mach_vm_read_overwrite(
        target_task: mach::VmMapRead,
        address: Addr,
        size: Size,
        data: Addr,
        outsize: *mut Size,
    ) -> mach::KernReturn;
}

#[cfg(test)]
mod tests {
    use crate::mach;

    #[test]
    fn reads_and_reports_unmapped() {
        let value = 0x1234_5678u32;
        let mut out = [0u8; 4];
        let copied = mach::mach_vm::read_overwrite(
            mach::Port::current_task(),
            &value as *const u32 as mach::MachVmAddr,
            &mut out,
        )
        .unwrap();
        assert_eq!(copied, 4);
        assert_eq!(u32::from_ne_bytes(out), value);

        // The zero page is never mapped.
        let err = mach::mach_vm::read_overwrite(mach::Port::current_task(), 8, &mut out);
        assert!(err.is_err());
    }
}
