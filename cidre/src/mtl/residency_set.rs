use crate::{api, arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTLResidencySetDescriptor")]
    pub ResidencySetDesc(ns::Id),
    MTL_RESIDENCY_SET_DESCRIPTOR,
    #[api::available(macos = 15.0, ios = 18.0, maccatalyst = 18.0, tvos = 18.0, visionos = 2.0)]
);

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_RESIDENCY_SET_DESCRIPTOR: &'static objc::Class<ResidencySetDesc>;
}

impl ResidencySetDesc {
    define_mtl!(label, set_label);

    /// If non-zero, defines the number of allocations for which to initialize the internal arrays. Defaults to zero.
    #[objc::msg_send(initialCapacity)]
    pub fn initial_capacity(&self) -> usize;

    #[objc::msg_send(setInitialCapacity:)]
    pub fn set_initial_capacity(&mut self, val: usize);
}

define_obj_type!(
    #[doc(alias = "MTLResidencySet")]
    pub ResidencySet(ns::Id)
);

impl ResidencySet {
    define_mtl!(device, label);

    /// The memory footprint of the set in bytes at the last commit operation.
    /// This may include internal allocations as well.
    #[objc::msg_send(allocatedSize)]
    pub fn allocated_size(&self) -> usize;

    /// Requests that the set and all the committed resources and heaps are made resident.
    #[objc::msg_send(requestResidency)]
    pub fn request_residency(&self);

    /// Requests that the set and all the committed resources and heaps are made non-resident.
    #[objc::msg_send(endResidency)]
    pub fn end_residency(&self);

    /// Adds one allocation to the set, leaving it uncommitted until commit is called.
    #[objc::msg_send(addAllocation:)]
    pub fn add_allocation(&mut self, val: &mtl::Allocation);

    #[objc::msg_send(addAllocations:count:)]
    pub unsafe fn add_allocations_count(&mut self, ptr: *const &mtl::Allocation, count: usize);

    #[inline]
    pub fn add_allocations(&mut self, allocations: &[&mtl::Allocation]) {
        unsafe { self.add_allocations_count(allocations.as_ptr(), allocations.len()) };
    }

    /// Marks an allocation to be removed from the set on the next commit call.
    #[objc::msg_send(removeAllocation:)]
    pub fn remove_allocation(&mut self, val: &mtl::Allocation);

    #[objc::msg_send(removeAllocations:count:)]
    pub unsafe fn remove_allocations_count(&mut self, ptr: *const &mtl::Allocation, count: usize);

    /// Marks allocations to be removed from the set on the next commit call.
    #[inline]
    pub fn remove_allocations(&mut self, allocations: &[&mtl::Allocation]) {
        unsafe { self.remove_allocations_count(allocations.as_ptr(), allocations.len()) };
    }

    /// Marks all allocations to be removed from the set on the next commit call.
    #[objc::msg_send(removeAllAllocations)]
    pub fn remove_all_allocations(&mut self);

    /// Returns a boolean indicating whether the allocation is present in the set or not.
    ///
    /// This check includes non-committed allocations in the set.
    #[objc::msg_send(containsAllocation:)]
    pub fn contains_allocation(&self, val: &mtl::Allocation) -> bool;

    /// Array of all allocations associated with the set.
    ///
    /// This property includes non-committed allocations in the set.
    #[objc::msg_send(allAllocations)]
    pub fn all_allocations(&self) -> arc::R<ns::Array<mtl::Allocation>>;

    /// Returns the current number of unique allocations present in the set.
    ///
    /// This property includes non-committed allocations in the set.
    #[objc::msg_send(allocationCount)]
    pub fn allocation_count(&self) -> usize;

    /// Commits any pending adds/removes.
    ///
    /// If the residency set is resident, this will try to make added resources and heaps
    /// resident instantly, and make removed resources and heaps non-resident.
    #[objc::msg_send(commit)]
    pub fn commit(&mut self);
}

#[cfg(test)]
mod tests {
    use crate::{api, mtl};

    #[test]
    fn basics() {
        if api::version!(macos = 15.0) {
            let mut desc = mtl::ResidencySetDesc::new().unwrap();
            assert_eq!(0, desc.initial_capacity());
            desc.set_initial_capacity(10);
            assert_eq!(10, desc.initial_capacity());
            let device = mtl::Device::sys_default().unwrap();
            let set = unsafe { device.new_residency_set(&desc).unwrap() };
            assert_eq!(0, set.allocation_count());
        } else {
            assert!(mtl::ResidencySetDesc::new().is_none());
        }
    }
}
