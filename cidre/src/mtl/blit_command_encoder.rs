use crate::{define_mtl, define_obj_type, define_options, mtl, ns, objc};

define_options!(BlitOption(usize));

impl BlitOption {
    pub const NONE: Self = Self(0);
    pub const DEPTH_FROM_DEPTH_STENCIL: Self = Self(1 << 0);
    pub const STENCIL_FROM_DEPTH_STENCIL: Self = Self(1 << 1);
    pub const ROW_LINEAR_PVRTC: Self = Self(1 << 2);
}

define_obj_type!(BlitCmdEncoder(mtl::CmdEncoder));

impl BlitCmdEncoder {
    define_mtl!(update_fence, wait_for_fence);

    #[objc::msg_send(fillBuffer:range:value:)]
    pub fn fill_buf(&self, buffer: &mtl::Buf, range: ns::Range, value: u8);

    #[objc::msg_send(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:)]
    pub fn copy_texture(
        &self,
        src_texture: &mtl::Texture,
        src_slice: usize,
        src_level: usize,
        src_origin: mtl::Origin,
        src_size: mtl::Size,
        dst_texture: &mtl::Texture,
        dst_slice: usize,
        dst_level: usize,
        dst_origin: mtl::Origin,
    );

    #[objc::msg_send(copyFromTexture:toTexture:)]
    pub fn copy_texture_to_texture(&self, src_texture: &mtl::Texture, dest_texture: &mtl::Texture);

    #[objc::msg_send(optimizeContentsForGPUAccess:)]
    pub fn optimize_contents_for_gpu_access(&self, texture: &mtl::Texture);

    #[objc::msg_send(resetCommandsInBuffer:withRange:)]
    pub fn reset_cmds_in_buf(&self, buf: &mtl::IndirectCmdBuf, range: ns::Range);
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let device = mtl::Device::default().unwrap();

        let cmd_queue = device.new_cmd_queue().unwrap();
        let cmd_buf = cmd_queue.new_cmd_buf().unwrap();

        let fence = device.new_fence().unwrap();

        let mut blit_enc = cmd_buf.new_blit_cmd_enc().unwrap();

        blit_enc.update_fence(&fence);
        blit_enc.end_encoding();

        let mut compute_enc = cmd_buf.new_compute_cmd_enc().unwrap();
        compute_enc.wait_for_fence(&fence);
        compute_enc.end_encoding();

        cmd_buf.commit();
        cmd_buf.wait_until_completed();
    }
}
