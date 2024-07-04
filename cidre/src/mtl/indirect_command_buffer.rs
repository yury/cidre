use crate::{arc, define_mtl, define_obj_type, define_opts, mtl, ns, objc};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct ExecutionRange {
    pub loc: u32,
    pub len: u32,
}

define_opts!(
    #[doc(alias = "MTLIndirectCommandType")]
    pub IndirectCmdType(usize)
);

impl IndirectCmdType {
    ///A draw call command.
    pub const DRAW: Self = Self(1 << 0);

    /// An indexed draw call command.
    pub const DRAW_INDEXED: Self = Self(1 << 1);

    /// A draw call command for tessellated patches.
    pub const DRAW_PATCHES: Self = Self(1 << 2);

    /// An indexed draw call command for tessellated patches.
    pub const DRAW_INDEXED_PATCHES: Self = Self(1 << 3);

    /// A compute command using a grid aligned to threadgroup boundaries.
    pub const CONCURRENT_DISPATCH: Self = Self(1 << 5); /* Dispatch threadgroups with concurrent execution */

    /// A compute command using an arbitrarily sized grid.
    pub const CONCURRENT_DISPATCH_THREADS: Self = Self(1 << 6); /* Dispatch threads with concurrent execution */

    pub const DRAW_MESH_THREADGROUPS: Self = Self(1 << 7);
    pub const DRAW_MESH_THREADS: Self = Self(1 << 8);
}

define_obj_type!(
    /// Describes the limits and features that can be used in an indirect command
    #[doc(alias = "MTLIndirectCommandBufferDescriptor")]
    pub Desc(ns::Id),
    MTL_INDIRECT_COMMAND_BUFFER_DESCRIPTOR
);

impl Desc {
    #[objc::msg_send(commandTypes)]
    pub fn cmd_types(&self) -> IndirectCmdType;

    #[objc::msg_send(setCommandTypes:)]
    pub fn set_cmd_types(&mut self, val: IndirectCmdType);

    #[objc::msg_send(inheritPipelineState)]
    pub fn inherit_ps(&self) -> bool;

    #[objc::msg_send(setInheritPipelineState:)]
    pub fn set_inherit_ps(&self, val: bool) -> bool;

    #[objc::msg_send(inheritBuffers)]
    pub fn inherit_bufs(&self) -> bool;

    #[objc::msg_send(setInheritBuffers:)]
    pub fn set_inherit_bufs(&self, val: bool);

    #[objc::msg_send(maxVertexBufferBindCount)]
    pub fn max_vertex_buf_bind_count(&self) -> usize;

    #[objc::msg_send(maxVertexBufferBindCount:)]
    pub fn set_max_vertex_buf_bind_count(&mut self, val: usize);

    #[objc::msg_send(maxFragmentBufferBindCount)]
    pub fn max_fragment_buf_bind_count(&self) -> usize;

    #[objc::msg_send(maxFragmentBufferBindCount:)]
    pub fn set_max_fragment_buf_bind_count(&mut self, val: usize);

    #[objc::msg_send(maxKernelBufferBindCount)]
    pub fn max_kernel_buf_bind_count(&self) -> usize;

    #[objc::msg_send(maxKernelBufferBindCount:)]
    pub fn set_max_kernel_buf_bind_count(&mut self, val: usize);

    #[objc::msg_send(supportRayTracing)]
    pub fn support_ray_tracing(&self) -> bool;

    #[objc::msg_send(setSupportRayTracing:)]
    pub fn set_support_ray_tracing(&mut self, val: bool);
}

define_obj_type!(
    /// A command buffer containing reusable commands, encoded either on the CPU or GPU.
    #[doc(alias = "MTLIndirectCommandBuffer")]
    pub IndirectCmdBuf(mtl::Res)
);

impl IndirectCmdBuf {
    define_mtl!(gpu_resource_id);

    #[objc::msg_send(size)]
    pub fn size(&self) -> usize;

    #[objc::msg_send(resetWithRange:)]
    pub fn reset_with_range(&mut self, range: ns::Range);

    #[objc::msg_send(indirectRenderCommandAtIndex:)]
    pub unsafe fn indirect_render_cmd_at_throws(
        &self,
        index: usize,
    ) -> arc::R<mtl::IndirectRenderCmd>;

    #[inline]
    pub fn indirect_render_cmd_at<'ar>(
        &self,
        index: usize,
    ) -> Result<arc::R<mtl::IndirectRenderCmd>, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.indirect_render_cmd_at_throws(index) })
    }

    #[objc::msg_send(indirectComputeCommandAtIndex:)]
    pub unsafe fn indirect_compute_cmd_at_throws(
        &self,
        index: usize,
    ) -> arc::R<mtl::IndirectComputeCmd>;

    #[inline]
    pub fn indirect_compute_cmd_at<'ar>(
        &self,
        index: usize,
    ) -> Result<arc::R<mtl::IndirectComputeCmd>, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.indirect_compute_cmd_at_throws(index) })
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_INDIRECT_COMMAND_BUFFER_DESCRIPTOR: &'static objc::Class<Desc>;
}
