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
    define_mtl!(gpu_resource_id);

    #[objc::msg_send(size)]
    pub fn size(&self) -> usize;

    #[objc::msg_send(resetWithRange:)]
    pub fn reset_with_range(&mut self, range: ns::Range);

    #[objc::msg_send(indirectRenderCommandAtIndex:)]
    pub fn indirect_render_cmd_at_throws(&self, index: usize) -> &mtl::IndirectRenderCmd;

    #[inline]
    pub fn indirect_render_cmd_at<'ar>(
        &self,
        index: usize,
    ) -> Result<&mtl::IndirectRenderCmd, &'ar ns::Exception> {
        ns::try_catch(|| self.indirect_render_cmd_at_throws(index))
    }

    #[objc::msg_send(indirectComputeCommandAtIndex:)]
    pub fn indirect_compute_cmd_at_throws(&self, index: usize) -> &mtl::IndirectComputeCmd;

    #[inline]
    pub fn indirect_compute_cmd_at<'ar>(
        &self,
        index: usize,
    ) -> Result<&mtl::IndirectComputeCmd, &'ar ns::Exception> {
        ns::try_catch(|| self.indirect_compute_cmd_at_throws(index))
    }
}
