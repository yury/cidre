use crate::{arc, define_obj_type, mtl, mtl4, ns, objc};

define_obj_type!(
    #[doc(alias = "MTL4CommandBufferOptions")]
    pub CmdBufOpts(ns::Id),
    MTL4_COMMAND_BUFFER_OPTIONS
);

impl CmdBufOpts {
    // log state
}

define_obj_type!(
    #[doc(alias = "MTL4CommandBuffer")]
    pub CmdBuf(ns::Id)
);

impl CmdBuf {
    /// Returns the GPU device that this command buffer belongs to.
    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(beginCommandBufferWithAllocator:)]
    pub fn begin_cmd_buf(&mut self, allocator: &mut mtl4::CmdAllocator);

    #[objc::msg_send(beginCommandBufferWithAllocator:options:)]
    pub fn begin_cmd_buf_with_opts(
        &mut self,
        allocator: &mut mtl4::CmdAllocator,
        options: &CmdBufOpts,
    );

    #[objc::msg_send(endCommandBuffer)]
    pub fn end_cmd_buf(&mut self);

    #[objc::msg_send(renderCommandEncoderWithDescriptor:)]
    pub fn render_cmd_encoder(
        &self,
        desc: &mtl4::RenderPassDesc,
    ) -> Option<arc::R<mtl4::RenderCmdEncoder>>;

    #[objc::msg_send(renderCommandEncoderWithDescriptor:options:)]
    pub fn render_cmd_encoder_with_opts(
        &self,
        desc: &mtl4::RenderPassDesc,
        options: mtl4::RenderEncoderOpts,
    ) -> Option<arc::R<mtl4::RenderCmdEncoder>>;

    #[objc::msg_send(computeCommandEncoder)]
    pub fn compute_cmd_encdoer(&self) -> Option<arc::R<mtl4::ComputeCmdEncoder>>;

    #[objc::msg_send(machineLearningCommandEncoder)]
    pub fn ml_cmd_encoder(&self) -> Option<arc::R<mtl4::MlCmdEncoder>>;

    #[objc::msg_send(useResidencySet:)]
    pub fn use_residency_set(&mut self, val: &mtl::ResidencySet);

    #[objc::msg_send(useResidencySets:count:)]
    pub unsafe fn use_residency_sets_count(
        &mut self,
        residency_sets: *const &mtl::ResidencySet,
        count: usize,
    );

    #[inline]
    pub fn use_residency_sets(&mut self, sets: &[&mtl::ResidencySet]) {
        unsafe {
            self.use_residency_sets_count(sets.as_ptr(), sets.len());
        }
    }

    #[objc::msg_send(pushDebugGroup:)]
    pub fn push_debug_group(&mut self, val: &ns::String);

    #[objc::msg_send(popDebugGroup)]
    pub fn pop_debug_group(&mut self);

    #[objc::msg_send(writeTimestampIntoHeap:atIndex:)]
    pub fn write_timestamp_into_heap(&self, heap: &mut mtl4::CounterHeap, index: usize);

    #[objc::msg_send(resolveCounterHeap:withRange:intoBuffer:atOffset:waitFence:updateFence:)]
    pub fn resolve_counter_heap(
        &self,
        heap: &mtl4::CounterHeap,
        range: ns::Range,
        into_buffer: &mut mtl::Buf,
        aligned_offset: usize,
        wait_fence: Option<&mtl::Fence>,
        update_fence: Option<&mtl::Fence>,
    );
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL4_COMMAND_BUFFER_OPTIONS: &'static objc::Class<CmdBufOpts>;
}
