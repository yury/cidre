use crate::{
    define_mtl, define_obj_type,
    ns::{self, Range},
    objc::Id,
};

use super::{Buffer, CommandEncoder, IndirectCommandBuffer, Origin, Size, Texture};

#[repr(usize)]
pub enum BlitOption {
    None = 0,
    DepthFromDepthStencil = 1 << 0,
    StencilFromDepthStencil = 1 << 1,
    RowLinearPVRTC = 1 << 2,
}

define_obj_type!(BlitCommandEncoder(CommandEncoder));

/// ```
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

    #[inline]
    pub fn fill_buffer(&self, buffer: &Buffer, range: ns::Range, value: u8) {
        unsafe { wsel_fillBuffer(self, buffer, range, value) }
    }

    pub fn copy_texture(
        &self,
        src_texture: &Texture,
        src_slice: usize,
        src_level: usize,
        src_origin: Origin,
        src_size: Size,
        dest_texture: &Texture,
        dest_slice: usize,
        dest_level: usize,
        dest_origin: Origin,
    ) {
        unsafe {
            wsel_copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(self, src_texture, src_slice, src_level, src_origin, src_size, dest_texture, dest_slice, dest_level, dest_origin)
        }
    }

    #[inline]
    pub fn copy_texture_to_texture(&self, src_texture: &Texture, dest_texture: &Texture) {
        unsafe { wsel_copyFromTexture_toTexture(self, src_texture, dest_texture) }
    }

    #[inline]
    pub fn optimize_contents_for_gpu_access(&self, texture: &Texture) {
        unsafe { wsel_optimizeContentsForGPUAccess(self, texture) }
    }

    #[inline]
    pub fn reset_commands_in_buffer_with_range(
        &self,
        buffer: &IndirectCommandBuffer,
        range: Range,
    ) {
        unsafe { wsel_resetCommandsInBuffer_withRange(self, buffer, range) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn wsel_fillBuffer(id: &Id, buffer: &Buffer, range: ns::Range, value: u8);

    fn wsel_copyFromTexture_sourceSlice_sourceLevel_sourceOrigin_sourceSize_toTexture_destinationSlice_destinationLevel_destinationOrigin(
        id: &Id,
        src_texture: &Texture,
        src_slice: usize,
        src_level: usize,
        src_origin: Origin,
        src_size: Size,
        dest_texture: &Texture,
        dest_slice: usize,
        dest_level: usize,
        dest_origin: Origin,
    );

    fn wsel_copyFromTexture_toTexture(id: &Id, src_texture: &Texture, dest_texture: &Texture);

    fn wsel_optimizeContentsForGPUAccess(id: &Id, texture: &Texture);

    fn wsel_resetCommandsInBuffer_withRange(
        id: &Id,
        buffer: &IndirectCommandBuffer,
        with_range: Range,
    );
}
