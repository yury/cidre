use crate::{api, arc, define_obj_type, mtl, ns, objc};

define_obj_type!(
    #[doc(alias = "MTL4RenderPassDescriptor")]
    pub RenderPassDesc(ns::Id),
    MTL4_RENDER_PASS_DESCRIPTOR,
    #[api::available(macos = 26.0, ios = 26.0, maccatalyst = 26.0, tvos = 26.0, visionos = 26.0)]
);

impl RenderPassDesc {
    /// Accesses state information for a render attachment that stores stencil data.
    #[objc::msg_send(colorAttachments)]
    pub fn color_attaches(&self) -> arc::R<mtl::RenderPassColorAttachDescArray>;

    #[objc::msg_send(defaultRasterSampleCount)]
    pub fn default_raster_sample_count(&self) -> usize;

    /// Accesses state information for a render attachment that stores stencil data.
    #[objc::msg_send(depthAttachment)]
    pub fn depth_attach(&self) -> arc::R<mtl::RenderPassDepthAttachDesc>;

    #[objc::msg_send(imageblockSampleLength)]
    pub fn imageblock_sample_len(&self) -> usize;

    #[objc::msg_send(renderTargetArrayLength)]
    pub fn render_target_array_len(&self) -> usize;

    #[objc::msg_send(renderTargeHeight)]
    pub fn render_target_height(&self) -> usize;

    #[objc::msg_send(renderTargetWidth)]
    pub fn render_target_width(&self) -> usize;

    /// Sets the default raster sample count for the render pass when it references no attachments.
    #[objc::msg_send(setDefaultRasterSampleCount:)]
    pub fn set_default_raster_sample_count(&mut self, val: usize);

    #[objc::msg_send(setDepthAttachment:)]
    pub fn set_depth_attach(&mut self, val: Option<&mtl::RenderPassDepthAttachDesc>);

    /// Assigns the per-sample size, in bytes, of the largest explicit imageblock layout in the render pass.
    #[objc::msg_send(setImageblockSampleLength:)]
    pub fn set_imageblock_sample_len(&mut self, val: usize);

    /// Assigns the number of layers that all attachments this descriptor references have.
    #[objc::msg_send(setRenderTargetArrayLength:)]
    pub fn set_render_target_array_len(&mut self, val: usize);

    #[objc::msg_send(setRenderTargetHeight:)]
    pub fn set_render_target_height(&mut self, val: usize);

    #[objc::msg_send(setRenderTargetWidth:)]
    pub fn set_render_target_width(&mut self, val: usize);

    #[objc::msg_send(setStencilAttachment:)]
    pub fn set_stencil_attach(&mut self, val: Option<&mtl::RenderPassStencilAttachDesc>);

    /// Assigns the per-tile size, in bytes, of the persistent threadgroup memory allocation of this render pass.
    #[objc::msg_send(setThreadgroupMemoryLength:)]
    pub fn set_threadgroup_memory_len(&mut self, val: usize);

    /// Sets the height, in pixels, of the tiles in which Metal divides the render attachments for tile-based rendering.
    #[objc::msg_send(setTileHeight:)]
    pub fn set_tile_height(&mut self, val: usize);

    /// Sets the width, in pixels, of the tiles into which Metal divides the render attachments for tile-based rendering.
    #[objc::msg_send(setTileWidth:)]
    pub fn set_tile_width(&mut self, val: usize);

    #[objc::msg_send(setVisibilityResultBuffer:)]
    pub fn set_visibility_result_buf(&mut self, val: Option<&mtl::Buf>);

    #[objc::msg_send(setVisibilityResultType:)]
    pub fn set_visibility_result_type(&mut self, val: mtl::VisibilityResultMode);

    /// Accesses state information for a render attachment that stores stencil data.
    #[objc::msg_send(stencilAttachment)]
    pub fn stencil_attach(&self) -> arc::R<mtl::RenderPassStencilAttachDesc>;

    #[objc::msg_send(threadgroupMemoryLength)]
    pub fn threadgroup_mem_len(&self) -> usize;

    #[objc::msg_send(tileHeight)]
    pub fn tile_height(&self) -> usize;

    #[objc::msg_send(tileWidth)]
    pub fn tile_width(&self) -> usize;

    #[objc::msg_send(visibilityResultBuffer)]
    pub fn visibility_result_buf(&self) -> Option<arc::R<mtl::Buf>>;

    #[objc::msg_send(visibilityResultType)]
    pub fn visibility_result_type(&self) -> mtl::VisibilityResultMode;

    #[objc::msg_send(setSamplePositions:count:)]
    pub unsafe fn set_sample_positions_count(
        &mut self,
        posistions: *const mtl::SamplePos,
        count: usize,
    );

    #[inline]
    pub fn set_sample_positions(&mut self, posistions: &[mtl::SamplePos]) {
        unsafe { self.set_sample_positions_count(posistions.as_ptr(), posistions.len()) };
    }

    #[objc::msg_send(getSamplePositions:count:)]
    pub unsafe fn get_sample_positions_count(
        &self,
        positions: *mut mtl::SamplePos,
        count: usize,
    ) -> usize;

    pub unsafe fn get_sample_positions(&self, positions: &mut [mtl::SamplePos]) -> usize {
        unsafe { self.get_sample_positions_count(positions.as_mut_ptr(), positions.len()) }
    }

    #[objc::msg_send(supportColorAttachmentMapping)]
    pub fn support_color_attach_mapping(&self) -> bool;

    /// Controls if the render pass supports color attachment mapping.
    #[objc::msg_send(setSupportColorAttachmentMapping:)]
    pub fn set_support_color_attach_mapping(&mut self, val: bool);
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL4_RENDER_PASS_DESCRIPTOR: &'static objc::Class<RenderPassDesc>;
}
