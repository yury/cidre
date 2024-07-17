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

    /// Commits any pending adds/removes.
    ///
    /// If the residency set is resident, this will try to make added resources and heaps
    /// resident instantly, and make removed resources and heaps non-resident.
    #[objc::msg_send(commit)]
    pub fn commit(&mut self);
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let mut desc = mtl::ResidencySetDesc::new().unwrap();
        assert_eq!(0, desc.initial_capacity());
        desc.set_initial_capacity(10);
        assert_eq!(10, desc.initial_capacity());
    }
}
