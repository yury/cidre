use super::{KernReturn, VMAddress, VMAllocationFlags, VMSize};

extern "C" {
    pub fn vm_allocate(
        target_task: super::VMMap,
        address: *mut VMAddress,
        size: VMSize,
        flags: VMAllocationFlags,
    ) -> KernReturn;

    pub fn vm_deallocate(target_task: super::VMMap, address: VMAddress, size: VMSize)
        -> KernReturn;
}
