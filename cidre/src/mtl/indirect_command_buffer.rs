use crate::{define_mtl, define_obj_type, define_options, mtl, ns, objc};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct ExecutionRange {
    pub location: u32,
    pub length: u32,
}

define_options!(IndirectCmdType(usize));

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
}

define_obj_type!(Desc(ns::Id));

impl Desc {
    #[objc::msg_send(commandTypes)]
    pub fn cmd_types(&self) -> IndirectCmdType;

    #[objc::msg_send(setCommandTypes:)]
    pub fn set_cmd_types(&mut self, value: IndirectCmdType);

    #[objc::msg_send(inheritPipelineState)]
    pub fn inherit_ps(&self) -> bool;

    #[objc::msg_send(setInheritPipelineState:)]
    pub fn set_inherit_ps(&self, value: bool) -> bool;

    #[objc::msg_send(inheritBuffers)]
    pub fn inherit_bufs(&self) -> bool;

    #[objc::msg_send(setInheritBuffers:)]
    pub fn set_inherit_bufs(&self, value: bool);

    #[objc::msg_send(maxVertexBufferBindCount)]
    pub fn max_vertex_buf_bind_count(&self) -> usize;

    #[objc::msg_send(maxVertexBufferBindCount:)]
    pub fn set_max_vertex_buf_bind_count(&mut self, value: usize);

    #[objc::msg_send(maxFragmentBufferBindCount)]
    pub fn max_fragment_buf_bind_count(&self) -> usize;

    #[objc::msg_send(maxFragmentBufferBindCount:)]
    pub fn set_max_fragment_buf_bind_count(&mut self, value: usize);

    #[objc::msg_send(maxKernelBufferBindCount)]
    pub fn max_kernel_buf_bind_count(&self) -> usize;

    #[objc::msg_send(maxKernelBufferBindCount:)]
    pub fn set_max_kernel_buf_bind_count(&mut self, value: usize);

    #[objc::msg_send(supportRayTracing)]
    pub fn support_ray_tracing(&self) -> bool;

    #[objc::msg_send(setSupportRayTracing:)]
    pub fn set_support_ray_tracing(&mut self, value: bool);
}

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
