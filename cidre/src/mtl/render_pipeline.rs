use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

use super::{argument::Arg, Fn, PixelFormat};

#[doc(alias = "MTLBlendFactor")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SrcColor = 2,
    OneMinusSrcColor = 3,
    SrcAlpha = 4,
    OneMinusSrcAlpha = 5,
    DstColor = 6,
    OneMinusDstColor = 7,
    DstAlpha = 8,
    OneMinusDstAlpha = 9,
    SrcAlphaSaturated = 10,
    BlendColor = 11,
    OneMinusBlendColor = 12,
    BlendAlpha = 13,
    OneMinusBlendAlpha = 14,
    Src1Color = 15,
    OneMinusSrc1Color = 16,
    Src1Alpha = 17,
    OneMinusSrc1Alpha = 18,
}

#[doc(alias = "MTLBlendOperation")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum BlendOp {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
}

#[doc(alias = "MTLColorWriteMask")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum ColorWriteMask {
    None = 0,
    Red = 0x1 << 3,
    Green = 0x1 << 2,
    Blue = 0x1 << 1,
    Alpha = 0x1 << 0,
    All = 0xf,
}

#[doc(alias = "MTLPrimitiveTopologyClass")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum PrimitiveTopologyClass {
    Unspecified = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
}

#[doc(alias = "MTLTessellationPartitionMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationPartitionMode {
    Pow2 = 0,
    Integer = 1,
    FractionalOdd = 2,
    FractionalEven = 3,
}

#[doc(alias = "MTLTessellationFactorStepFunction")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationFactorStepFn {
    Constant = 0,
    PerPatch = 1,
    PerInstance = 2,
    PerPatchAndPerInstance = 3,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationFactorFormat {
    MTLTessellationFactorFormatHalf = 0,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TessellationControlPointIndexType {
    None = 0,
    U16 = 1,
    U32 = 2,
}

define_obj_type!(
    #[doc(alias = "MTLRenderPipelineColorAttachmentDescriptor")]
    pub ColorAttachDesc(ns::Id)
);

impl ColorAttachDesc {
    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> PixelFormat;

    #[objc::msg_send(setPixelFormat:)]
    pub fn set_pixel_format(&mut self, val: PixelFormat);

    #[objc::msg_send(isBlendingEnabled)]
    pub fn blending_enabled(&self) -> bool;

    #[objc::msg_send(setBlendingEnabled:)]
    pub fn set_blending_enabled(&mut self, val: bool);

    #[objc::msg_send(sourceRGBBlendFactor)]
    pub fn src_rgb_blend_factor(&self) -> BlendFactor;

    #[objc::msg_send(setSourceRGBBlendFactor:)]
    pub fn set_src_rgb_blend_factor(&mut self, val: BlendFactor);

    #[objc::msg_send(destinationRGBBlendFactor)]
    pub fn dst_rgb_blend_factor(&self) -> BlendFactor;

    #[objc::msg_send(setDestinationRGBBlendFactor:)]
    pub fn set_dst_rgb_blend_factor(&mut self, val: BlendFactor);

    #[objc::msg_send(rgbBlendOperation)]
    pub fn rgb_blend_op(&self) -> BlendOp;

    #[objc::msg_send(setRgbBlendOperation:)]
    pub fn set_rgb_blend_op(&mut self, val: BlendOp);

    #[objc::msg_send(sourceAlphaBlendFactor)]
    pub fn src_alpha_blend_factor(&self) -> BlendFactor;

    #[objc::msg_send(setSourceAlphaBlendFactor:)]
    pub fn set_src_alpha_blend_factor(&mut self, val: BlendFactor);

    #[objc::msg_send(destinationAlphaBlendFactor)]
    pub fn dst_alpha_blend_factor(&self) -> BlendFactor;

    #[objc::msg_send(setDestinationAlphaBlendFactor:)]
    pub fn set_dst_alpha_blend_factor(&mut self, val: BlendFactor);

    #[objc::msg_send(alphaBlendOperation)]
    pub fn alpha_blend_op(&self) -> BlendOp;

    #[objc::msg_send(setAlphaBlendOperation:)]
    pub fn set_alpha_blend_op(&mut self, val: BlendOp);

    #[objc::msg_send(writeMask)]
    pub fn write_mask(&self) -> ColorWriteMask;

    #[objc::msg_send(setWriteMask:)]
    pub fn set_write_mask(&mut self, val: ColorWriteMask);
}

define_obj_type!(
    #[doc(alias = "MTLRenderPipelineReflection")]
    pub Reflection(ns::Id)
);

impl Reflection {
    #[objc::msg_send(vertexArguments)]
    pub fn vertex_args(&self) -> Option<arc::R<ns::Array<Arg>>>;

    #[objc::msg_send(fragmentArguments)]
    pub fn fragment_args(&self) -> Option<arc::R<ns::Array<Arg>>>;

    #[objc::msg_send(tileArguments)]
    pub fn tile_args(&self) -> Option<arc::R<ns::Array<Arg>>>;
}

define_obj_type!(
   #[doc(alias = "MTLRenderPipelineDescriptor")]
   pub Desc(ns::Id),
   MTL_RENDER_PIPELINE_DESCRIPTOR
);

impl arc::R<Desc> {
    #[inline]
    pub fn with_fns(mut self, vertex_fn: &Fn, fragment_fn: &Fn) -> Self {
        self.set_vertex_fn(Some(vertex_fn));
        self.set_fragment_fn(Some(fragment_fn));
        self
    }

    #[inline]
    pub fn with_fns_vertex_desc(
        mut self,
        vertex_fn: &Fn,
        fragment_fn: &Fn,
        vertex_desc: &mtl::VertexDesc,
    ) -> Self {
        self.set_vertex_fn(Some(vertex_fn));
        self.set_fragment_fn(Some(fragment_fn));
        self.set_vertex_desc(Some(vertex_desc));
        self
    }
}

impl Desc {
    define_mtl!(reset);

    #[objc::msg_send(vertexFunction)]
    pub fn vertex_fn(&self) -> Option<arc::R<Fn>>;

    #[objc::msg_send(setVertexFunction:)]
    pub fn set_vertex_fn(&mut self, val: Option<&Fn>);

    #[objc::msg_send(vertexDescriptor)]
    pub fn vertex_desc(&self) -> Option<arc::R<mtl::VertexDesc>>;

    #[objc::msg_send(setVertexDescriptor:)]
    pub fn set_vertex_desc(&mut self, val: Option<&mtl::VertexDesc>);

    #[objc::msg_send(fragmentFunction)]
    pub fn fragment_fn(&self) -> Option<arc::R<Fn>>;

    #[objc::msg_send(setFragmentFunction:)]
    pub fn set_fragment_fn(&mut self, val: Option<&Fn>);

    #[objc::msg_send(colorAttachments)]
    pub fn color_attaches(&self) -> &ColorAttachDescArray;

    #[objc::msg_send(colorAttachments)]
    pub fn color_attaches_mut(&mut self) -> &mut ColorAttachDescArray;

    #[objc::msg_send(rasterSampleCount)]
    pub fn raster_sample_count(&self) -> usize;

    #[objc::msg_send(setRasterSampleCount:)]
    pub fn set_raster_sample_count(&mut self, val: usize);

    #[objc::msg_send(depthAttachmentPixelFormat)]
    pub fn depth_attach_pixel_format(&self) -> PixelFormat;

    #[objc::msg_send(setDepthAttachmentPixelFormat:)]
    pub fn set_depth_attach_pixel_format(&mut self, val: PixelFormat);

    #[objc::msg_send(stencilAttachmentPixelFormat)]
    pub fn stencil_attach_pixel_format(&self) -> PixelFormat;

    #[objc::msg_send(setStencilAttachmentPixelFormat:)]
    pub fn set_stencil_attach_pixel_format(&mut self, val: PixelFormat);

    #[objc::msg_send(supportIndirectCommandBuffers)]
    pub fn support_icbs(&self) -> bool;

    #[objc::msg_send(setSupportIndirectCommandBuffers:)]
    pub fn set_support_icbs(&mut self, val: bool);
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_RENDER_PIPELINE_DESCRIPTOR: &'static objc::Class<Desc>;
    static MTL_TILE_RENDER_PIPELINE_DESCRIPTOR: &'static objc::Class<TileRenderPipelineDesc>;
}

define_obj_type!(
    #[doc(alias = "MTLRenderPipelineFunctionsDescriptor")]
    pub FnsDesc(ns::Id)
);

define_obj_type!(
    #[doc(alias = "MTLRenderPipelineState")]
    pub State(ns::Id)
);

impl State {
    define_mtl!(gpu_res_id);

    #[objc::msg_send(device)]
    pub fn device(&self) -> &mtl::Device;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<&ns::String>;

    #[objc::msg_send(maxTotalThreadsPerThreadgroup)]
    pub fn max_total_threads_per_threadgroup(&self) -> usize;

    #[objc::msg_send(threadgroupSizeMatchesTileSize)]
    pub fn threadgroup_size_matches_tile_size(&self) -> bool;

    #[objc::msg_send(imageblockSampleLength)]
    pub fn imageblock_sample_length(&self) -> usize;

    #[objc::msg_send(supportIndirectCommandBuffers)]
    pub fn support_indirect_cmd_bufs(&self) -> bool;
}

define_obj_type!(
    #[doc(alias = "MTLRenderPipelineColorAttachmentDescriptorArray")]
    pub ColorAttachDescArray(ns::Id)
);

impl ColorAttachDescArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get(&self, index: usize) -> &ColorAttachDesc;

    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get_mut(&mut self, index: usize) -> &mut ColorAttachDesc;

    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get_at(&self, index: usize) -> arc::R<ColorAttachDesc>;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set(&mut self, object: Option<&ColorAttachDesc>, index: usize);
}

impl std::ops::Index<usize> for ColorAttachDescArray {
    type Output = ColorAttachDesc;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl std::ops::IndexMut<usize> for ColorAttachDescArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

define_obj_type!(
    #[doc(alias = "MTLTileRenderPipelineColorAttachmentDescriptor")]
    pub TileRenderPipelineColorAttachDesc(ns::Id)
);

impl TileRenderPipelineColorAttachDesc {
    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setPixelFormat:)]
    pub fn set_pixel_format(&mut self, val: mtl::PixelFormat);
}

define_obj_type!(
    #[doc(alias = "MTLTileRenderPipelineColorAttachmentDescriptorArray")]
    pub TileRenderPipelineColorAttachDescArray(ns::Id)
);

impl TileRenderPipelineColorAttachDescArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get(&self, index: usize) -> &TileRenderPipelineColorAttachDesc;

    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get_mut(&mut self, index: usize) -> &mut TileRenderPipelineColorAttachDesc;

    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get_at(&self, index: usize) -> arc::R<TileRenderPipelineColorAttachDesc>;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set(&mut self, val: Option<&TileRenderPipelineColorAttachDesc>, index: usize);
}

impl std::ops::Index<usize> for TileRenderPipelineColorAttachDescArray {
    type Output = TileRenderPipelineColorAttachDesc;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl std::ops::IndexMut<usize> for TileRenderPipelineColorAttachDescArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

impl arc::R<TileRenderPipelineDesc> {
    #[inline]
    pub fn with_fn(mut self, tile_fn: &Fn) -> Self {
        self.set_tile_fn(tile_fn);
        self
    }
}

define_obj_type!(
    #[doc(alias = "MTLTileRenderPipelineDescriptor")]
    pub TileRenderPipelineDesc(ns::Id),
    MTL_TILE_RENDER_PIPELINE_DESCRIPTOR
);

impl TileRenderPipelineDesc {
    define_mtl!(set_label);

    #[objc::msg_send(device)]
    pub fn device(&self) -> &mtl::Device;

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<&ns::String>;

    #[objc::msg_send(tileFunction)]
    pub fn tile_fn(&self) -> &mtl::Fn;

    #[objc::msg_send(setTileFunction:)]
    pub fn set_tile_fn(&mut self, val: &mtl::Fn);

    #[objc::msg_send(rasterSampleCount)]
    pub fn raster_sample_count(&self) -> usize;

    #[objc::msg_send(setRasterSampleCount:)]
    pub fn set_raster_sample_count(&mut self, val: usize);

    #[objc::msg_send(colorAttachments)]
    pub fn color_attaches(&self) -> &TileRenderPipelineColorAttachDescArray;

    #[objc::msg_send(colorAttachments)]
    pub fn color_attaches_mut(&mut self) -> &mut TileRenderPipelineColorAttachDescArray;

    #[objc::msg_send(threadgroupSizeMatchesTileSize)]
    pub fn threadgroup_size_matches_tile_size(&self) -> bool;

    #[objc::msg_send(setThreadgroupSizeMatchesTileSize:)]
    pub fn set_threadgroup_size_matches_tile_size(&mut self, val: bool);

    #[objc::msg_send(tileBuffers)]
    pub fn tile_bufs(&self) -> &mtl::PipelineBufDescArray;

    #[objc::msg_send(tileBuffers)]
    pub fn tile_bufs_mut(&mut self) -> &mut mtl::PipelineBufDescArray;
}

define_obj_type!(
    #[doc(alias = "MTLMeshRenderPipelineDescriptor")]
    pub MeshRenderPipelineDesc(ns::Id)
);

impl MeshRenderPipelineDesc {
    define_mtl!(reset, set_label);

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<&ns::String>;

    /// Optional shader function responsible for determining how many threadgroups of the mesh shader to run,
    /// can optionally provide payload data for the mesh stage.
    /// If this is None, no payload data is available to the mesh function, and the draw command determines
    /// how many threadgroups of the mesh stage to run.
    /// The default value is None.
    #[objc::msg_send(objectFunction)]
    pub fn object_fn(&self) -> Option<&Fn>;

    #[objc::msg_send(setObjectFunction:)]
    pub fn set_object_fn(&mut self, val: Option<&Fn>);

    /// Shader function responsible for exporting a chunk of geometry per threadgroup for the rasterizer.
    /// The default value is None.
    #[objc::msg_send(meshFunction)]
    pub fn mesh_fn(&self) -> Option<&Fn>;

    #[objc::msg_send(setMeshFunction:)]
    pub fn set_mesh_fn(&mut self, val: Option<&Fn>);

    /// Like a classical render pipeline, this fragments covered by the rasterized geometry are shaded
    /// with this function.
    /// The default value is None. To create a pipeline, you must either set fragment_fn to Some,
    /// or set_rasterization_enabled to false.
    #[objc::msg_send(fragmentFunction)]
    pub fn fragment_fn(&self) -> Option<&Fn>;

    #[objc::msg_send(setFragmentFunction:)]
    pub fn set_fragment_fn(&mut self, val: Option<&Fn>);

    #[objc::msg_send(maxTotalThreadsPerObjectThreadgroup)]
    pub fn max_total_threads_per_obj_threadgroup(&self) -> usize;

    #[objc::msg_send(setMaxTotalThreadsPerObjectThreadgroup:)]
    pub fn set_max_total_threads_per_obj_threadgroup(&mut self, val: usize);

    #[objc::msg_send(maxTotalThreadsPerMeshThreadgroup)]
    pub fn max_total_threads_per_mesh_threadgroup(&self) -> usize;

    #[objc::msg_send(setMaxTotalThreadsPerMeshThreadgroup:)]
    pub fn set_max_total_threads_per_mesh_threadgroup(&mut self, val: usize);

    #[objc::msg_send(objectThreadgroupSizeIsMultipleOfThreadExecutionWidth)]
    pub fn obj_threadgroup_size_is_multiple_of_thread_execution_width(&self) -> bool;

    #[objc::msg_send(setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth:)]
    pub fn set_obj_threadgroup_size_is_multiple_of_thread_execution_width(&mut self, val: bool);

    #[objc::msg_send(meshThreadgroupSizeIsMultipleOfThreadExecutionWidth)]
    pub fn mesh_threadgroup_size_is_multiple_of_thread_execution_width(&self) -> bool;

    #[objc::msg_send(setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth:)]
    pub fn set_mesh_threadgroup_size_is_multiple_of_thread_execution_width(&mut self, val: bool);

    #[objc::msg_send(payloadMemoryLength)]
    pub fn payload_mem_len(&self) -> usize;

    #[objc::msg_send(setPayloadMemoryLength:)]
    pub fn set_payload_mem_len(&mut self, val: usize);

    #[objc::msg_send(maxTotalThreadgroupsPerMeshGrid)]
    pub fn max_total_threadgroups_per_mesh_grid(&self) -> usize;

    #[objc::msg_send(setMaxTotalThreadgroupsPerMeshGrid:)]
    pub fn set_max_total_threadgroups_per_mesh_grid(&mut self, val: usize);

    /// Provide mutability information on the buffers used by obj_fn.
    /// Specifying these values is optional; it may be used to optimize the shader code.
    #[objc::msg_send(objectBuffers)]
    pub fn obj_bufs(&self) -> &mtl::PipelineBufDescArray;

    #[objc::msg_send(objectBuffers)]
    pub fn obj_bufs_mut(&mut self) -> &mut mtl::PipelineBufDescArray;

    /// Specifying these values is optional; it may be used to optimize the shader code.
    #[objc::msg_send(meshBuffers)]
    pub fn mesh_bufs(&self) -> &mtl::PipelineBufDescArray;

    #[objc::msg_send(meshBuffers)]
    pub fn mesh_bufs_mut(&mut self) -> &mut mtl::PipelineBufDescArray;

    /// Specifying these values is optional; it may be used to optimize the shader code.
    #[objc::msg_send(fragmentBuffers)]
    pub fn fragment_bufs(&self) -> &mtl::PipelineBufDescArray;

    #[objc::msg_send(fragmentBuffers)]
    pub fn fragment_bufs_mut(&mut self) -> &mut mtl::PipelineBufDescArray;

    /// The number of samples per fragment of the render pass in which this pipeline will be used.
    #[objc::msg_send(rasterSampleCount)]
    pub fn raster_sample_count(&self) -> usize;

    #[objc::msg_send(setRasterSampleCount:)]
    pub fn set_raster_sample_count(&mut self, val: usize);

    /// Whether the alpha value exported by the fragment shader for the first color attachment
    /// is converted to a sample mask, which is subsequently AND-ed with the fragments' sample mask
    /// The default value is false.
    #[objc::msg_send(isAlphaToCoverageEnabled)]
    pub fn is_alpha_to_coverage_enabled(&self) -> bool;

    #[objc::msg_send(setAlphaToCoverageEnabled:)]
    pub fn set_alpha_to_coverage_enabled(&self, val: bool);

    #[objc::msg_send(isAlphaToOneEnabled)]
    pub fn is_alpha_to_one_enabled(&self) -> bool;

    #[objc::msg_send(setAlphaToOneEnabled:)]
    pub fn set_alpha_to_one_enabled(&self, val: bool);

    /// Whether rasterization is disabled, all primitives are dropped prior to rasterization.
    /// Default is true
    #[objc::msg_send(isRasterizationEnabled)]
    pub fn is_rasterization_enabled(&self) -> bool;

    #[objc::msg_send(setRasterizationEnabled:)]
    pub fn set_rasterization_enabled(&mut self, val: bool);

    /// The maximum value that can be passed to setVertexAmplificationCount when using this pipeline.
    /// The default value is 1. The value must be supported by the device, which can be checked with
    /// supports_vertex_amplification_count.
    #[objc::msg_send(maxVertexAmplificationCount)]
    pub fn max_vertex_amplification_count(&self) -> usize;

    #[objc::msg_send(setMaxVertexAmplificationCount:)]
    pub fn set_max_vertex_amplification_count(&mut self, val: usize);

    #[objc::msg_send(colorAttachments)]
    pub fn color_attaches(&self) -> &mtl::RenderPipelineColorAttachDescArray;

    #[objc::msg_send(colorAttachments)]
    pub fn color_attaches_mut(&mut self) -> &mut mtl::RenderPipelineColorAttachDescArray;

    /// The pixel format of the depth attachment of the render pass in which this pipeline will be used.
    /// The default value is mtl::PixelFormat::Invalid; indicating no depth attachment will be used.
    #[objc::msg_send(depthAttachmentPixelFormat)]
    pub fn depth_attach_pixel_format(&self) -> PixelFormat;

    #[objc::msg_send(setDepthAttachmentPixelFormat:)]
    pub fn set_depth_attach_pixel_format(&mut self, val: PixelFormat);

    /// The pixel format of the stencil attachment of the render pass in which this pipeline will be used.
    /// The default value is mtl::PixelFormat::Invalid; indicating no stencil attachment will be used.
    #[objc::msg_send(stencilAttachmentPixelFormat)]
    pub fn stencil_attach_pixel_format(&self) -> PixelFormat;

    #[objc::msg_send(setStencilAttachmentPixelFormat:)]
    pub fn set_stencil_attach_pixel_format(&mut self, val: PixelFormat);
}

#[cfg(test)]
mod tests {
    use crate::mtl;
    #[test]
    fn basics() {
        let mut desc = mtl::RenderPipelineDesc::new();

        assert!(desc.vertex_fn().is_none());
        assert!(desc.fragment_fn().is_none());

        desc.reset();
    }
}
