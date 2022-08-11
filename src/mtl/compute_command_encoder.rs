use crate::{define_mtl, define_obj_type, msg_send, mtl, ns};

use super::CommandEncoder;

define_obj_type!(ComputeCommandEncoder(CommandEncoder));

impl ComputeCommandEncoder {
    define_mtl!(
        update_fence,
        wait_for_fence,
        use_resource,
        use_resources,
        use_heap
    );

    #[inline]
    pub fn set_compute_pipeline_state(&mut self, state: &mtl::ComputePipelineState) {
        msg_send!("mtl", self, sel_setComputePipelineState, state)
    }

    #[inline]
    pub fn set_texture_at_index(&mut self, texture: Option<&mtl::Texture>, index: usize) {
        msg_send!("mtl", self, sel_setTexture_atIndex, texture, index)
    }

    #[inline]
    pub fn set_textures_with_range(&mut self, textures: *const &mtl::Texture, range: ns::Range) {
        msg_send!("mtl", self, sel_setTextures_withRange, textures, range)
    }

    #[inline]
    pub fn dispatch_threads(
        &mut self,
        threads_per_grid: mtl::Size,
        threads_per_threadgroup: mtl::Size,
    ) {
        msg_send!(
            "mtl",
            self,
            sel_dispatchThreads_threadsPerThreadgroup,
            threads_per_grid,
            threads_per_threadgroup
        )
    }

    #[inline]
    pub fn dispatch_threadgroups(
        &mut self,
        threadgroups_per_grid: mtl::Size,
        threads_per_threadgroup: mtl::Size,
    ) {
        msg_send!(
            "mtl",
            self,
            sel_dispatchThreadgroups_threadsPerThreadgroup,
            threadgroups_per_grid,
            threads_per_threadgroup
        )
    }

    #[inline]
    pub fn set_image_block_size(&mut self, width: usize, height: usize) {
        msg_send!("mtl", self, sel_setImageblockWidth_height, width, height)
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {}
