use crate::{define_obj_type, define_opts, mtl, mtl4, ns, objc};

define_opts!(
    #[doc(alias = "MTL4RenderEncoderOptions")]
    pub RenderEncoderOpts(usize)
);

impl RenderEncoderOpts {
    /// Declares that this render pass doesn't suspend nor resume.
    pub const NONE: Self = Self(0);

    /// Configures the render pass as *suspending*.
    pub const SUSPENDING: Self = Self(1 << 0);

    /// Configures the render pass to as *resuming*.
    pub const RESUMING: Self = Self(1 << 1);
}

define_obj_type!(
    #[doc(alias = "MTL4RenderCommandEncoder")]
    pub RenderCmdEncoder(mtl4::CmdEncoder)
);

impl RenderCmdEncoder {
    #[objc::msg_send(tileWidth)]
    pub fn tile_width(&self) -> usize;

    #[objc::msg_send(tileHeight)]
    pub fn tile_height(&self) -> usize;

    #[objc::msg_send(setColorAttachmentMap:)]
    pub fn set_color_attach_map(&mut self, val: &mtl::LogicalToPhysicalColorAttachMap);

    #[objc::msg_send(setRenderPipelineState:)]
    pub fn set_render_ps(&mut self, val: &mtl::RenderPipelineState);

    #[objc::msg_send(setViewport:)]
    pub fn set_vp(&mut self, val: mtl::Viewport);

    #[inline]
    pub fn set_vp_rect<F: Into<f64>>(&mut self, x: F, y: F, width: F, height: F) {
        self.set_vp(mtl::Viewport {
            x: x.into(),
            y: y.into(),
            w: width.into(),
            h: height.into(),
            z_near: 0.into(),
            z_far: 1.into(),
        })
    }

    #[objc::msg_send(setViewports:count:)]
    pub fn set_vps_count(&mut self, val: *const mtl::Viewport, count: usize);

    #[inline]
    pub fn set_vps(&mut self, val: &[mtl::Viewport]) {
        self.set_vps_count(val.as_ptr(), val.len());
    }

    #[objc::msg_send(setScissorRect:)]
    pub fn set_scissor_rect(&mut self, val: mtl::ScissorRect);

    #[objc::msg_send(setScissorRects:count:)]
    pub fn set_scissor_rects_count(&mut self, val: *const mtl::ScissorRect, count: usize);

    #[inline]
    pub fn set_scissor_rects(&mut self, val: &[mtl::ScissorRect]) {
        self.set_scissor_rects_count(val.as_ptr(), val.len());
    }

    #[objc::msg_send(setVertexAmplificationCount:viewMappings:)]
    pub unsafe fn set_vertex_amplification_count(
        &mut self,
        count: usize,
        view_mapping: *const mtl::VertexAmplificationViewMapping,
    );

    pub fn set_vertex_amplifications(&mut self, values: &[mtl::VertexAmplificationViewMapping]) {
        unsafe {
            self.set_vertex_amplification_count(values.len(), values.as_ptr());
        }
    }

    #[objc::msg_send(setCullMode:)]
    pub fn set_cull_mode(&mut self, val: mtl::CullMode);

    #[objc::msg_send(setDepthClipMode:)]
    pub fn set_depth_clip_mode(&mut self, val: mtl::DepthClipMode);

    #[objc::msg_send(setDepthBias:slopeScale:clamp:)]
    pub fn set_depth_bias(&mut self, depth_bias: f32, slope_scale: f32, clamp: f32);

    #[objc::msg_send(setTriangleFillMode:)]
    pub fn set_triangle_fill_mode(&mut self, val: mtl::TriangleFillMode);

    #[objc::msg_send(setBlendColorRed:green:blue:alpha:)]
    pub fn set_blend_color(&mut self, r: f32, g: f32, b: f32, a: f32);

    #[objc::msg_send(setDepthStencilState:)]
    pub fn set_depth_stencil_state(&mut self, val: Option<&mtl::DepthStencilState>);

    #[objc::msg_send(setStencilReferenceValue:)]
    pub fn set_stencil_reference_value(&mut self, val: u32);

    #[objc::msg_send(setStencilFrontReferenceValue:backReferenceValue:)]
    pub fn set_stencil_front_back_reference_values(
        &mut self,
        front_reference_val: u32,
        back_reference_val: u32,
    );

    #[objc::msg_send(setVisibilityResultMode:offset:)]
    pub fn set_visibility_result_mode(&mut self, val: mtl::VisibilityResultMode, offset: usize);

    #[objc::msg_send(setColorStoreAction:atIndex:)]
    pub fn set_color_store_action_at(&mut self, val: mtl::StoreAction, index: usize);

    #[objc::msg_send(setDepthStoreAction:)]
    pub fn set_depth_store_action(&mut self, val: mtl::StoreAction);

    #[objc::msg_send(setStencilStoreAction:)]
    pub fn set_stencil_store_action(&mut self, val: mtl::StoreAction);

    #[objc::msg_send(drawPrimitives:vertexStart:vertexCount:)]
    pub fn draw_primitives(
        &mut self,
        primitive_type: mtl::Primitive,
        vertex_start: usize,
        vertex_count: usize,
    );

    #[objc::msg_send(drawPrimitives:vertexStart:vertexCount:instanceCount:)]
    pub fn draw_primitives_instance_count(
        &mut self,
        primitive_type: mtl::Primitive,
        vertex_start: usize,
        vertex_count: usize,
        instance_count: usize,
    );

    #[objc::msg_send(drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:)]
    pub fn draw_primitives_instance_count_base_instance(
        &mut self,
        primitive_type: mtl::Primitive,
        vertex_start: usize,
        vertex_count: usize,
        instance_count: usize,
        base_instance: usize,
    );

    #[objc::msg_send(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferLength:)]
    pub fn draw_indexed_primitives(
        &mut self,
        primitive_type: mtl::Primitive,
        index_count: usize,
        index_type: mtl::IndexType,
        index_buf: usize,
        index_buf_len: usize,
    );

    #[objc::msg_send(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferLength:instanceCount:)]
    pub fn draw_indexed_primitives_instance_count(
        &mut self,
        primitive_type: mtl::Primitive,
        index_count: usize,
        index_type: mtl::IndexType,
        index_buf: usize,
        index_buf_len: usize,
        instance_count: usize,
    );

    #[objc::msg_send(executeCommandsInBuffer:withRange:)]
    pub fn exec_cmds_in_buf(&mut self, icb: &mtl::IndirectCmdBuf, range: ns::Range);

    #[objc::msg_send(executeCommandsInBuffer:indirectBuffer:)]
    pub fn exec_cmds_in_buf_indirect_range_buf(
        &mut self,
        icb: &mtl::IndirectCmdBuf,
        indirect_range_buf: u64,
    );

    #[objc::msg_send(dispatchThreadsPerTile:)]
    pub fn dispatch_threads_per_tile(&mut self, threads_per_tile: mtl::Size);

    #[objc::msg_send(setThreadgroupMemoryLength:offset:atIndex:)]
    pub fn set_threadgroup_mem_len(&mut self, length: usize, offset: usize, index: usize);

    #[objc::msg_send(setArgumentTable:atStages:)]
    pub fn set_arg_table(&mut self, table: &mtl4::ArgTable, stages: mtl::Stages);

    #[objc::msg_send(setFrontFacingWinding:)]
    pub fn set_ffw(&mut self, val: mtl::Winding);
}
