use crate::{arc, define_obj_type, mtl, mtl4, objc};

define_obj_type!(
    #[doc(alias = "MTL4TileRenderPipelineDescriptor")]
    pub TileRenderPipelineDesc(mtl4::PipelineDesc)
);

impl TileRenderPipelineDesc {
    #[objc::msg_send(tileFunctionDescriptor)]
    pub fn tile_fn_desc(&self) -> Option<arc::R<mtl4::FnDesc>>;

    #[objc::msg_send(setTileFunctionDescriptor:)]
    pub fn set_tile_fn_desc(&mut self, val: Option<&mtl4::FnDesc>);

    #[objc::msg_send(rasterSampleCount)]
    pub fn raster_sample_count(&self) -> usize;

    #[objc::msg_send(setRasterSampleCount:)]
    pub fn set_raster_sample_count(&mut self, val: usize);

    #[objc::msg_send(colorAttachments)]
    pub fn color_attaches(&self) -> arc::R<mtl::TileRenderPipelineColorAttachDescArray>;

    #[objc::msg_send(threadgroupSizeMatchesTileSize)]
    pub fn threadgroup_size_matches_tile_size(&self) -> bool;

    #[objc::msg_send(setThreadgroupSizeMatchesTileSize:)]
    pub fn set_threadgroup_size_matches_tile_size(&mut self, val: bool);

    #[objc::msg_send(maxTotalThreadsPerThreadgroup)]
    pub fn max_total_threads_per_threadgroup(&self) -> usize;

    #[objc::msg_send(setMaxTotalThreadsPerThreadgroup:)]
    pub fn set_max_total_threads_per_threadgroup(&mut self, val: usize);

    #[objc::msg_send(requiredThreadsPerThreadgroup)]
    pub fn required_threads_per_threadgroup(&self) -> mtl::Size;

    #[objc::msg_send(setRequiredThreadsPerThreadgroup:)]
    pub fn set_required_threads_per_threadgroup(&self, val: mtl::Size);

    #[objc::msg_send(supportBinaryLinking)]
    pub fn support_binary_linking(&self) -> bool;

    #[objc::msg_send(setSupportBinaryLinking:)]
    pub fn set_support_binary_linking(&mut self, val: bool);

    #[objc::msg_send(reset)]
    pub fn reset(&mut self);
}
