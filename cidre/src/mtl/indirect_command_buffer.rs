use crate::{define_mtl, define_obj_type, mtl, ns, objc};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct ExecutionRange {
    pub location: u32,
    pub length: u32,
}

define_obj_type!(Descriptor(ns::Id));
define_obj_type!(IndirectCmdBuf(mtl::Resource));

impl IndirectCmdBuf {
    define_mtl!(gpu_resouce_id);

    #[objc::msg_send(size)]
    pub fn size(&self) -> usize;

    #[objc::msg_send(resetWithRange:)]
    pub fn reset_with_range(&mut self, range: ns::Range);

    // - (id <MTLIndirectRenderCommand>)indirectRenderCommandAtIndex:(NSUInteger)commandIndex;
    // pub fn inderect_render_cmd_at_throws(&self) -> &mtl::InderectRen
    // - (id <MTLIndirectComputeCommand>)indirectComputeCommandAtIndex:(NSUInteger)commandIndex API_AVAILABLE(macos(11.0), macCatalyst(14.0), ios(13.0));
}
