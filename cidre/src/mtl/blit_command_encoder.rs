use crate::{define_mtl, define_obj_type, define_options, mtl, ns, objc};

define_options!(BlitOption(usize));

impl BlitOption {
    pub const NONE: Self = Self(0);
    pub const DEPTH_FROM_DEPTH_STENCIL: Self = Self(1 << 0);
    pub const STENCIL_FROM_DEPTH_STENCIL: Self = Self(1 << 1);
    pub const ROW_LINEAR_PVRTC: Self = Self(1 << 2);
}

define_obj_type!(BlitCommandEncoder(mtl::CommandEncoder));

/// ```no_run
/// use cidre::{mtl};
///
/// let device = mtl::Device::default().unwrap();
///
/// let command_queue = device.command_queue().unwrap();
/// let command_buffer = command_queue.command_buffer().unwrap();
///
/// let fence = device.fence().unwrap();
///
/// let mut blit_encoder = command_buffer.blit_command_encoder().unwrap();
///
/// blit_encoder.update_fence(&fence);
/// blit_encoder.end_encoding();
///
/// let mut compute_encoder = command_buffer.compute_command_encoder().unwrap();
/// compute_encoder.wait_for_fence(&fence);
/// compute_encoder.end_encoding();
///
/// command_buffer.commit();
/// command_buffer.wait_until_completed();
///
/// ```
impl BlitCommandEncoder {
    define_mtl!(update_fence, wait_for_fence);

    #[objc::msg_send(fillBuffer:range:value:)]
    pub fn fill_buffer(&self, buffer: &mtl::Buffer, range: ns::Range, value: u8);

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
    pub fn reset_commands_in_buffer_with_range(
        &self,
        buffer: &mtl::IndirectCommandBuffer,
        range: ns::Range,
    );
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let device = mtl::Device::default().unwrap();

        let command_queue = device.command_queue().unwrap();
        let command_buffer = command_queue.command_buffer().unwrap();

        let fence = device.fence().unwrap();

        let mut blit_encoder = command_buffer.blit_command_encoder().unwrap();

        blit_encoder.update_fence(&fence);
        blit_encoder.end_encoding();

        let mut compute_encoder = command_buffer.compute_command_encoder().unwrap();
        compute_encoder.wait_for_fence(&fence);
        compute_encoder.end_encoding();

        command_buffer.commit();
        command_buffer.wait_until_completed();
    }
}
