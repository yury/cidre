use crate::{define_obj_type, mtl, mtl4, ns, objc};

define_obj_type!(
    #[doc(alias = "MTL4ComputeCommandEncoder")]
    pub ComputeCmdEncoder(mtl4::CmdEncoder)
);

impl ComputeCmdEncoder {
    #[objc::msg_send(stages)]
    pub fn stages(&self) -> mtl::Stages;

    #[objc::msg_send(setComputePipelineState:)]
    pub fn set_compute_ps(&mut self, val: &mtl::ComputePipelineState);

    #[objc::msg_send(setThreadgroupMemoryLength:atIndex:)]
    pub fn set_threadgroup_mem_len(&mut self, val: usize, index: usize);

    #[objc::msg_send(setImageblockWidth:height:)]
    pub fn set_imageblock_size(&mut self, width: usize, height: usize);

    #[objc::msg_send(dispatchThreads:threadsPerThreadgroup:)]
    pub fn dispatch_threads(
        &mut self,
        threads_per_grid: mtl::Size,
        threads_per_threadgroup: mtl::Size,
    );

    #[objc::msg_send(dispatchThreadgroups:threadsPerThreadgroup:)]
    pub fn dispatch_threadgroups(
        &mut self,
        threadgroups_per_grid: mtl::Size,
        threads_per_threadgroup: mtl::Size,
    );

    #[objc::msg_send(dispatchThreadgroupsWithIndirectBuffer:threadsPerThreadgroup:)]
    pub fn dispatch_threadgroups_indirect_buf(
        &mut self,
        indirect_buf: u64,
        threads_per_threadgroup: mtl::Size,
    );

    #[objc::msg_send(dispatchThreadsWithIndirectBuffer:)]
    pub fn dispatch_threads_with_indirect_buf(&mut self, indirect_buf: u64);

    #[objc::msg_send(executeCommandsInBuffer:withRange:)]
    pub fn exec_cmds_in_buf(&mut self, icb: &mtl::IndirectCmdBuf, exec_range: ns::Range);

    #[objc::msg_send(executeCommandsInBuffer:indirectBuffer:)]
    pub fn exec_cmds_in_buf_indirect_range(
        &mut self,
        icb: &mtl::IndirectCmdBuf,
        indirect_range_buf: u64,
    );

    #[objc::msg_send(copyFromTexture:toTexture:)]
    pub fn copy_texture(&mut self, src: &mtl::Texture, dst: &mut mtl::Texture);

    // TODO: copy variants

    #[objc::msg_send(generateMipmapsForTexture:)]
    pub fn generate_mipmaps_for_texture(&mut self, texture: &mut mtl::Texture);

    #[objc::msg_send(fillBuffer:range:value:)]
    pub fn fill_buf(&mut self, buf: &mut mtl::Buf, range: ns::Range, value: u8);

    #[objc::msg_send(optimizeContentsForGPUAccess:)]
    pub fn optimize_contents_for_gpu_access(&mut self, texture: &mut mtl::Texture);

    #[objc::msg_send(optimizeContentsForGPUAccess:slice:level:)]
    pub fn optimize_contents_for_gpu_access_slice(
        &mut self,
        texture: &mut mtl::Texture,
        slice: usize,
        level: usize,
    );

    #[objc::msg_send(optimizeContentsForCPUAccess:)]
    pub fn optimize_contents_for_cpu_access(&mut self, texture: &mut mtl::Texture);

    #[objc::msg_send(optimizeContentsForCPUAccess:slice:level:)]
    pub fn optimize_contents_for_cpu_access_slice(
        &mut self,
        texture: &mut mtl::Texture,
        slice: usize,
        level: usize,
    );

    #[objc::msg_send(resetCommandsInBuffer:withRange:)]
    pub fn reset_cmds_in_buf(&mut self, buf: &mut mtl::IndirectCmdBuf, range: ns::Range);

    #[objc::msg_send(copyIndirectCommandBuffer:sourceRange:destination:destinationIndex:)]
    pub fn copy_icb(
        &mut self,
        src: &mtl::IndirectCmdBuf,
        src_range: ns::Range,
        dst: &mut mtl::IndirectCmdBuf,
        dst_index: usize,
    );

    #[objc::msg_send(optimizeIndirectCommandBuffer:withRange:)]
    pub fn optimize_icb(&mut self, buf: &mut mtl::IndirectCmdBuf, range: ns::Range);

    #[objc::msg_send(setArgumentTable:)]
    pub fn set_arg_table(&mut self, val: Option<&mtl4::ArgTable>);
}
