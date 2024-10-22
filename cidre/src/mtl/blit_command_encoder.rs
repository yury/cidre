use crate::{define_mtl, define_obj_type, define_opts, mtl, ns, objc};

define_opts!(
    #[doc(alias = "MTLBlitOption")]
    pub BlitOpt(usize)
);

impl BlitOpt {
    pub const NONE: Self = Self(0);
    pub const DEPTH_FROM_DEPTH_STENCIL: Self = Self(1 << 0);
    pub const STENCIL_FROM_DEPTH_STENCIL: Self = Self(1 << 1);
    pub const ROW_LINEAR_PVRTC: Self = Self(1 << 2);
}

define_obj_type!(
    #[doc(alias = "MTLBlitCommandEncoder")]
    pub BlitCmdEncoder(mtl::CmdEncoder)
);

impl BlitCmdEncoder {
    define_mtl!(update_fence, wait_for_fence);

    #[objc::msg_send(fillBuffer:range:value:)]
    pub fn fill_buf(&self, buffer: &mut mtl::Buf, range: ns::Range, val: u8);

    #[objc::msg_send(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:)]
    pub fn copy_texture(
        &self,
        src_texture: &mtl::Texture,
        src_slice: usize,
        src_level: usize,
        src_origin: mtl::Origin,
        src_size: mtl::Size,
        dst_texture: &mut mtl::Texture,
        dst_slice: usize,
        dst_level: usize,
        dst_origin: mtl::Origin,
    );

    #[objc::msg_send(copyFromTexture:toTexture:)]
    pub fn copy_texture_to_texture(
        &self,
        src_texture: &mtl::Texture,
        dst_texture: &mut mtl::Texture,
    );

    #[objc::msg_send(generateMipmapsForTexture:)]
    pub fn generate_mipmaps(&self, texture: &mut mtl::Texture);

    /// Encodes a command that copies data from one buffer into another.
    ///
    /// You can pass the same buffer to the src_buf and dst_buf parameters
    /// if size is less than the distance between src_offset and dst_offset.
    ///
    /// # Important
    ///
    /// Copying data to overlapping regions within the same buffer may result in unexpected behavior.
    #[objc::msg_send(copyFromBuffer:sourceOffset:toBuffer:destinationOffset:)]
    pub fn copy_buf_to_buf(
        &self,
        src_buf: &mtl::Buf,
        src_offset: usize,
        dst_buf: &mut mtl::Buf,
        dst_offset: usize,
    );

    #[objc::msg_send(copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:)]
    pub fn copy_from_buf_to_texture(
        &self,
        src_buf: &mtl::Buf,
        src_offset: usize,
        src_bytes_per_row: usize,
        src_bytes_per_image: usize,
        src_size: mtl::Size,
        dst_texture: &mut mtl::Texture,
        dst_slice: usize,
        dst_level: usize,
        dst_origin: mtl::Origin,
    );

    #[objc::msg_send(copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:options:)]
    pub fn copy_from_buf_to_texture_opts(
        &self,
        src_buf: &mtl::Buf,
        src_offset: usize,
        src_bytes_per_row: usize,
        src_bytes_per_image: usize,
        src_size: mtl::Size,
        dst_texture: &mut mtl::Texture,
        dst_slice: usize,
        dst_level: usize,
        dst_origin: mtl::Origin,
        opts: mtl::BlitOpt,
    );

    /// Encodes a command that improves the performance of the GPU’s accesses to a texture.
    ///
    /// This command can reduce the time it takes the GPU to access a texture. Apps typically run the command for:
    /// - Textures the GPU accesses for an extended period of time
    /// - Textures with a storageMode property that’s mtl::StorageMode::Shared or mtl::StorageMode::Managed
    #[objc::msg_send(optimizeContentsForGPUAccess:)]
    pub fn optimize_contents_for_gpu_access(&self, texture: &mut mtl::Texture);

    #[objc::msg_send(optimizeContentsForGPUAccess:slice:level:)]
    pub fn optimize_contents_for_gpu_access_slice_level(
        &self,
        texture: &mut mtl::Texture,
        slice: usize,
        level: usize,
    );

    #[objc::msg_send(optimizeIndirectCommandBuffer:withRange:)]
    pub fn optimize_indirect_cmd_buf_with_range(
        &self,
        buf: &mut mtl::IndirectCmdBuf,
        range: ns::Range,
    );

    #[inline]
    pub fn optimize_indirect_cmd_buf(
        &self,
        buf: &mut mtl::IndirectCmdBuf,
        range: std::ops::Range<usize>,
    ) {
        self.optimize_indirect_cmd_buf_with_range(
            buf,
            ns::Range {
                loc: range.start,
                len: range.end - range.start,
            },
        )
    }

    #[objc::msg_send(resetCommandsInBuffer:withRange:)]
    pub fn reset_cmds_in_buf(&self, buf: &mut mtl::IndirectCmdBuf, range: ns::Range);
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let device = mtl::Device::sys_default().unwrap();

        let cmd_queue = device.new_cmd_queue().unwrap();
        let cmd_buf = cmd_queue.new_cmd_buf().unwrap();

        let fence = device.new_fence().unwrap();

        let blit_enc = cmd_buf.new_blit_cmd_enc().unwrap();

        blit_enc.update_fence(&fence);
        blit_enc.end();

        let compute_enc = cmd_buf.new_compute_cmd_enc().unwrap();
        compute_enc.wait_for_fence(&fence);
        compute_enc.end();

        cmd_buf.commit();
        cmd_buf.wait_until_completed();
    }
}
