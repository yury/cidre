use crate::{arc, define_obj_type, mtl, mtl4, ns, objc};

#[doc(alias = "MTL4LogicalToPhysicalColorAttachmentMappingState")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(isize)]
pub enum LogicalToPhysicalColorAttachMappingState {
    Identiy = 0,
    Inherited = 1,
}

define_obj_type!(
    #[doc(alias = "MTL4RenderPipelineColorAttachmentDescriptor")]
    pub RenderPipelineColorAttachDesc(ns::Id)
);

impl RenderPipelineColorAttachDesc {
    #[objc::msg_send(pixelFormat)]
    pub fn pixel_format(&self) -> mtl::PixelFormat;

    #[objc::msg_send(setPixelFormat:)]
    pub fn set_pixel_format(&mut self, val: mtl::PixelFormat);

    #[objc::msg_send(blendingState)]
    pub fn blending_state(&self) -> mtl4::BlendState;

    #[objc::msg_send(setBlendingState:)]
    pub fn set_blending_state(&mut self, val: mtl4::BlendState);

    #[objc::msg_send(sourceRGBBlendFactor)]
    pub fn src_rgb_blend_factor(&self) -> mtl::BlendFactor;

    /// Configures the source RGB blend factor.
    ///
    /// This property defaults to [`mtl::BlendFactor::One`].
    #[objc::msg_send(setSourceRGBBlendFactor:)]
    pub fn set_src_rgb_blend_factor(&mut self, val: mtl::BlendFactor);

    #[objc::msg_send(destinationRGBBlendFactor)]
    pub fn dst_rgb_blend_factor(&self) -> mtl::BlendFactor;

    #[objc::msg_send(setDestinationRGBBlendFactor:)]
    pub fn set_dst_rgb_blend_factor(&mut self, val: mtl::BlendFactor);

    #[objc::msg_send(rgbBlendOperation)]
    pub fn rgb_blend_op(&self) -> mtl::BlendOp;

    #[objc::msg_send(setRgbBlendOperation:)]
    pub fn set_rgb_blend_op(&mut self, val: mtl::BlendOp);

    #[objc::msg_send(sourceAlphaBlendFactor)]
    pub fn src_alpha_blend_factor(&self) -> mtl::BlendFactor;

    #[objc::msg_send(setSourceAlphaBlendFactor:)]
    pub fn set_src_alpha_blend_factor(&mut self, val: mtl::BlendFactor);

    #[objc::msg_send(destinationAlphaBlendFactor)]
    pub fn dst_alpha_blend_factor(&self) -> mtl::BlendFactor;

    #[objc::msg_send(setDestinationAlphaBlendFactor:)]
    pub fn set_dst_alpha_blend_factor(&mut self, val: mtl::BlendFactor);

    #[objc::msg_send(alphaBlendOperation)]
    pub fn alpha_blend_op(&self) -> mtl::BlendOp;

    #[objc::msg_send(setAlphaBlendOperation:)]
    pub fn set_alpha_blend_op(&mut self, val: mtl::BlendOp);

    #[objc::msg_send(writeMask)]
    pub fn write_mask(&self) -> mtl::ColorWriteMask;

    #[objc::msg_send(setWriteMask:)]
    pub fn set_write_mask(&mut self, val: mtl::ColorWriteMask);

    #[objc::msg_send(reset)]
    pub fn reset(&mut self);
}

define_obj_type!(
    #[doc(alias = "MTL4RenderPipelineColorAttachmentDescriptorArray")]
    pub RenderPipelineColorAttachDescArray(ns::Id)
);

impl RenderPipelineColorAttachDescArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get(&self, index: usize) -> arc::R<RenderPipelineColorAttachDesc>;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set(&mut self, val: Option<&RenderPipelineColorAttachDesc>, index: usize);

    #[objc::msg_send(reset)]
    pub fn reset(&mut self);
}

define_obj_type!(
    #[doc(alias = "MTLLogicalToPhysicalColorAttachmentMap")]
    pub LogicalToPhysicalColorAttachMap(ns::Id)
);

impl LogicalToPhysicalColorAttachMap {
    #[objc::msg_send(setPhysicalIndex:forLogicalIndex:)]
    pub fn set(&mut self, physical_index: usize, logical_index: usize);

    #[objc::msg_send(getPhysicalIndexForLogicalIndex:)]
    pub fn get(&self, logical_index: usize);

    #[objc::msg_send(reset)]
    pub fn reset(&mut self);
}

define_obj_type!(
    #[doc(alias = "MTL4RenderPipelineDescriptor")]
    pub RenderPipelineDesc(mtl4::PipelineDesc)
);

impl RenderPipelineDesc {
    #[objc::msg_send(vertexFunctionDescriptor)]
    pub fn vertex_fn_desc(&self) -> Option<arc::R<mtl4::FnDesc>>;

    /// Assigns the shader function that this pipeline executes for each vertex.
    #[objc::msg_send(setVertexFunctionDescriptor:)]
    pub fn set_vertex_fn_desc(&mut self, val: Option<&mtl4::FnDesc>);

    #[objc::msg_send(fragmentFunctionDescriptor)]
    pub fn fragment_fn_desc(&self) -> Option<arc::R<mtl4::FnDesc>>;

    /// Assigns the shader function that this pipeline executes for each fragment.
    #[objc::msg_send(setFragmentFunctionDescriptor:)]
    pub fn set_fragment_fn_desc(&mut self, val: Option<&mtl4::FnDesc>);

    #[objc::msg_send(vertexDescriptor)]
    pub fn vertex_desc(&self) -> Option<arc::R<mtl::VertexDesc>>;

    #[objc::msg_send(setVertexDescriptor:)]
    pub fn set_vertex_desc(&mut self, val: Option<&mtl::VertexDesc>);

    #[objc::msg_send(rasterSampleCount)]
    pub fn raster_sample_count(&self) -> usize;

    #[objc::msg_send(setRasterSampleCount:)]
    pub fn set_raster_sample_count(&mut self, val: usize);

    #[objc::msg_send(alphaToCoverageState)]
    pub fn alpha_to_coverage_state(&self) -> mtl4::AlphaToCoverageState;

    #[objc::msg_send(setAlphaToCoverageState:)]
    pub fn set_alpha_to_coverage_state(&mut self, val: mtl4::AlphaToCoverageState);

    #[objc::msg_send(alphaToOneState)]
    pub fn alpha_to_one_state(&self) -> mtl4::AlphaToOneState;

    #[objc::msg_send(setAlphaToOneState:)]
    pub fn set_alpha_to_one_state(&mut self, val: mtl4::AlphaToOneState);

    #[objc::msg_send(isRasterizationEnabled)]
    pub fn is_rasterization_enabled(&self) -> bool;

    #[objc::msg_send(setRasterizationEnabled:)]
    pub fn set_rasterization_enabled(&mut self, val: bool);

    #[objc::msg_send(maxVertexAmplificationCount)]
    pub fn max_vertex_amplification_count(&self) -> usize;

    #[objc::msg_send(setMaxVertexAmplificationCount:)]
    pub fn set_max_vertex_amplification_count(&mut self, val: usize);

    #[objc::msg_send(colorAttachments)]
    pub fn color_attaches(&self) -> arc::R<RenderPipelineColorAttachDescArray>;

    #[objc::msg_send(inputPrimitiveTopology)]
    pub fn input_primitive_topology(&self) -> mtl::PrimitiveTopologyClass;

    #[objc::msg_send(setInputPrimitiveTopology:)]
    pub fn set_input_primitive_topology(&mut self, val: mtl::PrimitiveTopologyClass);

    #[objc::msg_send(colorAttachmentMappingState)]
    pub fn color_attach_mapping_state(&self) -> mtl::LogicalToPhysicalColorAttachMappingState;

    #[objc::msg_send(setColorAttachmentMappingState:)]
    pub fn set_color_attach_mapping_state(
        &mut self,
        val: mtl::LogicalToPhysicalColorAttachMappingState,
    );

    #[objc::msg_send(supportIndirectCommandBuffers)]
    pub fn support_icbs(&self) -> mtl4::IndirectCmdBufSupportState;

    #[objc::msg_send(setSupportIndirectCommandBuffers:)]
    pub fn set_support_icbs(&mut self, val: mtl4::IndirectCmdBufSupportState);

    #[objc::msg_send(reset)]
    pub fn reset(&mut self);
}
