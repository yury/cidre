use super::{KernReturn, VMAddr, VMAllocationFlags, VMSize};

extern "C" {
    pub fn vm_allocate(
        target_task: super::VMMap,
        address: *mut VMAddr,
        size: VMSize,
        flags: VMAllocationFlags,
    ) -> KernReturn;

    pub fn vm_deallocate(target_task: super::VMMap, address: VMAddr, size: VMSize) -> KernReturn;
}
