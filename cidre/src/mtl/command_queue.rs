use crate::{api, arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(
    /// A serial queue of command buffers to be executed by the device.
    #[doc(alias = "MTLCommandQueue")]
    pub CmdQueue(ns::Id)
);

impl CmdQueue {
    define_mtl!(set_label);

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(commandBuffer)]
    pub fn new_cmd_buf(&self) -> Option<arc::R<mtl::CmdBuf>>;

    #[objc::msg_send(commandBufferWithUnretainedReferences)]
    pub fn new_cmd_buf_unretained_refs(&self) -> Option<arc::R<mtl::CmdBuf>>;

    /// Marks the residency set as part of the command queue execution.
    ///
    /// This ensures that the residency set is resident during execution of all
    /// the command buffers within the queue.
    #[objc::msg_send(addResidencySet:)]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn add_residency_set(&self, set: &mtl::ResidencySet);

    #[objc::msg_send(addResidencySets:count:)]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub unsafe fn add_residency_sets_count(&self, sets: *const &mtl::ResidencySet, count: usize);

    /// Marks the residency sets as part of the command queue execution.
    ///
    /// This ensures that the residency sets are resident during execution of all
    /// the command buffers within the queue.
    #[inline]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn add_residency_sets(&self, sets: &[&mtl::ResidencySet]) {
        unsafe { self.add_residency_sets_count(sets.as_ptr(), sets.len()) }
    }

    /// Removes the residency set from the command queue execution.
    ///
    /// This ensures that only the remaining residency sets are resident during execution
    /// of all the command buffers within the queue.
    #[objc::msg_send(removeResidencySet:)]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn remove_residency_set(&self, set: &mtl::ResidencySet);

    #[objc::msg_send(removeResidencySets:count:)]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub unsafe fn remove_residency_sets_count(&self, sets: *const &mtl::ResidencySet, count: usize);

    /// Removes the residency sets from the command queue execution.
    ///
    /// This ensures that only the remaining residency sets are resident during execution
    /// of all the command buffers within the queue.
    #[inline]
    #[api::available(
        macos = 15.0,
        ios = 18.0,
        maccatalyst = 18.0,
        tvos = 18.0,
        visionos = 2.0
    )]
    pub fn remove_residency_sets(&self, sets: &[&mtl::ResidencySet]) {
        unsafe { self.remove_residency_sets_count(sets.as_ptr(), sets.len()) }
    }
}
