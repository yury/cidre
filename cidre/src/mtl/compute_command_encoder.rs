use crate::{define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(ComputeCmdEncoder(mtl::CmdEncoder));

impl ComputeCmdEncoder {
    define_mtl!(
        update_fence,
        wait_for_fence,
        use_resource,
        use_resources,
        use_heap
    );

    #[objc::msg_send(setComputePipelineState:)]
    pub fn set_compute_ps(&mut self, state: &mtl::ComputePipelineState);

    #[objc::msg_send(setTexture:atIndex:)]
    pub fn set_texture_at(&mut self, texture: Option<&mtl::Texture>, index: usize);

    #[objc::msg_send(setTextures:withRange:)]
    pub fn set_textures_with_range(&mut self, textures: *const &mtl::Texture, range: ns::Range);

    #[inline]
    pub fn set_textures(&mut self, textures: &[&mtl::Texture]) {
        self.set_textures_with_range(textures.as_ptr(), ns::Range::new(0, textures.len()))
    }

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

    #[objc::msg_send(setImageblockWidth:height:)]
    pub fn set_image_block_size(&mut self, width: usize, height: usize);
}
