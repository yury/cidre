use crate::{api, arc, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTL4CommandAllocatorDescriptor")]
    pub CmdAllocatorDesc(ns::Id),
    MTL4_COMMAND_ALLOCATOR_DESCRIPTOR,
    #[api::available(macos = 26.0, ios = 26.0)]
);

define_obj_type!(
    #[doc(alias = "MTL4CommandAllocator")]
    pub CmdAllocator(ns::Id)
);

impl CmdAllocator {
    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, val: Option<&ns::String>);

    /// Queries the size of the internal memory heaps of this command allocator that support encoding
    /// commands into command buffers.
    ///
    /// Returns a size in bytes.
    #[objc::msg_send(allocatedSize)]
    pub fn allocated_size(&self) -> u64;

    /// Marks the command allocator's heaps for reuse.
    #[objc::msg_send(reset)]
    pub fn reset(&mut self);
}

unsafe extern "C" {
    static MTL4_COMMAND_ALLOCATOR_DESCRIPTOR: &'static objc::Class<CmdAllocatorDesc>;
}
