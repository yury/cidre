use crate::{api, define_mtl, define_obj_type, define_opts, mtl, ns, objc};

/// The geometric primitive type for drawing commands.
#[doc(alias = "MTLPrimitiveType")]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(usize)]
pub enum Primitive {
    /// Rasterize a point at each vertex. The vertex shader must provide \[\[point_size\]\],
    /// or the point size is undefined.
    Point = 0,

    /// Rasterize a line between each separate pair of vertices,
    /// resulting in a series of unconnected lines. If there are
    /// an odd number of vertices, the last vertex is ignored
    Line = 1,

    /// Rasterize a line between each pair of adjacent vertices,
    /// resulting in a series of connected lines (also called a polyline).
    LineStrip = 2,

    /// For every separate set of three vertices, rasterize a triangle.
    /// If the number of vertices is not a multiple of three,
    /// either one or two vertices is ignored.
    Triangle = 3,

    /// For every three adjacent vertices, rasterize a triangle.
    TriangleStrip = 4,
}

#[doc(alias = "MTLVisibilityResultMode")]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(usize)]
pub enum VisibilityResultMode {
    #[doc(alias = "MTLVisibilityResultModeDisabled")]
    Disabled = 0,

    #[doc(alias = "MTLVisibilityResultModeBoolean")]
    Boolean = 1,

    #[doc(alias = "MTLVisibilityResultModeCounting")]
    Counting = 2,
}

#[doc(alias = "MTLScissorRect")]
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct ScissorRect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

/// A 3D rectangular region for the viewport clipping.
#[doc(alias = "MTLViewport")]
#[derive(Default, Debug, PartialEq, Copy, Clone)]
#[repr(C)]
pub struct Viewport {
    /// The x coordinate of the upper-left corner of the viewport.
    pub x: f64,
    /// The y coordinate of the upper-left corner of the viewport.
    pub y: f64,
    /// The width of the viewport, in pixels.
    pub w: f64,
    /// The height of the viewport, in pixels.
    pub h: f64,
    /// The z coordinate of the near clipping plane of the viewport.
    pub z_near: f64,
    /// The z coordinate of the far clipping plane of the viewport.
    pub z_far: f64,
}

impl Viewport {
    #[inline]
    pub const fn with_scissor_rect(rect: &ScissorRect) -> Self {
        Self {
            x: rect.x as _,
            y: rect.y as _,
            w: rect.w as _,
            h: rect.h as _,
            z_near: 0.0,
            z_far: 1.0,
        }
    }
}

/// The mode that determines whether to perform culling and which type of
/// primitive to cull.
#[doc(alias = "MTLCullMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum CullMode {
    /// Does not cull any primitives.
    None = 0,

    /// Culls front-facing primitives.
    Front = 1,

    /// Culls back-facing primitives.
    Back = 2,
}

/// The vertex winding rule that determines a front-facing primitive.
#[doc(alias = "MTLWinding")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(usize)]
pub enum Winding {
    /// Primitives whose vertices are specified in clockwise order are front-facing.
    #[doc(alias = "MTLWindingClockwise")]
    Cw = 0,

    /// Primitives whose vertices are specified in counter-clockwise order are front-facing.
    #[doc(alias = "MTLWindingCounterClockwise")]
    Ccw = 1,
}

#[doc(alias = "MTLDepthClipMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(usize)]
pub enum DepthClipMode {
    Clip = 0,
    Clamp = 1,
}

#[doc(alias = "MTLTriangleFillMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(usize)]
pub enum TriangleFillMode {
    #[doc(alias = "MTLTriangleFillModeFill")]
    Fill = 0,

    #[doc(alias = "MTLTriangleFillModeLines")]
    Lines = 1,
}

#[doc(alias = "MTLDrawPrimitivesIndirectArguments")]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[repr(C)]
pub struct DrawPrimitivesIndirectArgs {
    pub vertex_count: u32,
    pub inst_count: u32,
    pub vertex_start: u32,
    pub base_instance: u32,
}

#[doc(alias = "MTLDrawIndexedPrimitivesIndirectArguments")]
#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct DrawIndexedPrimitivesIndirectArgs {
    pub index_count: u32,
    pub inst_count: u32,
    pub index_start: u32,
    pub base_vertex: i32,
    pub base_instance: u32,
}

#[doc(alias = "MTLVertexAmplificationViewMapping")]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[repr(C)]
pub struct VertexAmplificationViewMapping {
    pub viewport_array_index_offset: u32,
    pub render_target_array_index_offset: u32,
}

#[doc(alias = "MTLDrawPatchIndirectArguments")]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[repr(C)]
pub struct DrawPatchIndirectArgs {
    pub patch_count: u32,
    pub instance_count: u32,
    pub patch_start: u32,
    pub base_instance: u32,
}

#[doc(alias = "MTLQuadTessellationFactorsHalf")]
#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct QuadTessellationFactorsHalf {
    /// NOTE: edge_tessellation_factor and inside_tessellation_factor are interpreted as half (16-bit floats)
    pub edge_tessellation_factor: [u16; 4],
    pub inside_tessellation_factor: [u16; 2],
}

#[doc(alias = "MTLTriangleTessellationFactorsHalf")]
#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct TriangleTessellationFactorsHalf {
    // NOTE: edge_tessellation_factor and inside_tessellation_factorare interpreted as half (16-bit floats) */
    pub edge_tessellation_factor: [u16; 3],
    pub inside_tessellation_factor: u16,
}

define_opts!(
    #[doc(alias = "MTLRenderStages")]
    pub RenderStages(usize)
);

impl RenderStages {
    /// The vertex rendering stage.
    ///
    /// All vertex work prior to rasterization has completed.
    #[doc(alias = "MTLRenderStageVertex")]
    pub const VERTEX: Self = Self(1usize << 0);

    /// The fragment rendering stage.
    ///
    /// All rendering work has completed.
    #[doc(alias = "MTLRenderStageFragment")]
    pub const FRAGMENT: Self = Self(1usize << 1);

    /// The tile rendering stage.
    #[doc(alias = "MTLRenderStageTile")]
    pub const TILE: Self = Self(1usize << 2);

    /// The object rendering stage.
    #[doc(alias = "MTLRenderStageObject")]
    pub const OBJECT: Self = Self(1usize << 3);

    /// The mesh rendering stage.
    #[doc(alias = "MTLRenderStageMesh")]
    pub const MESH: Self = Self(1usize << 4);
}

define_obj_type!(
    /// An interface that encodes a render pass into a command buffer,
    /// including all its draw calls and configuration.
    #[doc(alias = "MTLRenderCommandEncoder")]
    pub RenderCmdEncoder(mtl::CmdEncoder)
);

impl RenderCmdEncoder {
    define_mtl!(use_heap);

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

    /// Specifies an array of viewports, which are used to transform vertices from normalized device
    /// coordinates to window coordinates based on `[[ viewport_array_index ]]` value specified in
    /// the vertex shader.
    #[inline]
    pub fn set_vps(&mut self, val: &[mtl::Viewport]) {
        self.set_vps_count(val.as_ptr(), val.len());
    }

    /// An mtl::ScissorRect instance that represents a rectangle that needs to lie completely within the current render attachment.
    ///
    /// Specifies a rectangle for a fragment scissor test.  All fragments outside of this rectangle are discarded.
    #[objc::msg_send(setScissorRect:)]
    pub fn set_scissor_rect(&mut self, val: mtl::ScissorRect);

    #[objc::msg_send(setScissorRects:count:)]
    pub fn set_scissor_rects_count(&mut self, val: *const mtl::ScissorRect, count: usize);

    /// Specifies an array of rectangles for a fragment scissor test. The specific rectangle used is based
    /// on the `[[ viewport_array_index ]]` value output by the vertex shader. Fragments that lie outside
    /// the scissor rectangle are discarded.
    #[inline]
    pub fn set_scissor_rects(&mut self, val: &[mtl::ScissorRect]) {
        self.set_scissor_rects_count(val.as_ptr(), val.len());
    }

    #[objc::msg_send(setTriangleFillMode:)]
    pub fn set_triangle_fill_mode(&mut self, val: mtl::TriangleFillMode);

    #[objc::msg_send(setFrontFacingWinding:)]
    pub fn set_ffw(&mut self, val: mtl::Winding);

    #[objc::msg_send(setCullMode:)]
    pub fn set_cull_mode(&mut self, val: mtl::CullMode);

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

    #[objc::msg_send(setVertexBytes:length:atIndex:)]
    pub fn copy_bytes_to_vertex_at(
        &mut self,
        bytes: *const u8,
        length: ns::UInteger,
        binding_index: ns::UInteger,
    );

    #[inline]
    pub fn copy_slice_to_vertex_at<T>(&mut self, slice: &[T], index: usize) {
        self.copy_bytes_to_vertex_at(slice.as_ptr().cast(), std::mem::size_of_val(slice), index)
    }

    #[inline]
    pub fn copy_to_vertex_at<T>(&mut self, val: &T, index: usize) {
        self.copy_bytes_to_vertex_at(val as *const T as _, std::mem::size_of::<T>(), index)
    }

    #[objc::msg_send(setVertexBuffer:offset:atIndex:)]
    pub fn set_vertex_buf_at(&mut self, buf: Option<&mtl::Buf>, offset: usize, index: usize);

    #[objc::msg_send(useResource:usage:stages:)]
    pub fn use_resource(
        &mut self,
        resources: &mtl::Res,
        usage: mtl::ResUsage,
        stages: mtl::RenderStages,
    );

    #[objc::msg_send(useResources:count:usage:stages:)]
    pub unsafe fn use_resources_count(
        &mut self,
        resources: *const &mtl::Res,
        count: usize,
        usage: mtl::ResUsage,
        stages: mtl::RenderStages,
    );

    /// Declare that an array of resources may be accessed through an argument buffer by the render pass
    ///
    /// For hazard tracked resources, this method protects against data hazards.
    /// This method must be called before encoding any draw commands which may access the resources
    /// through an argument buffer. However, this method may cause color attachments to become decompressed.
    /// Therefore, this method should be called until as late as possible within a render command encoder.
    /// Declaring a minimal usage (i.e. read-only) may prevent color attachments from becoming decompressed on
    /// some devices.
    ///
    /// Note that calling use_resources() does not retain the resources. It is the responsiblity of the user
    /// to retain the resources until the command buffer has been executed.
    #[doc(alias = "useResources:count:usage:stages:")]
    #[inline]
    pub fn use_resources(
        &mut self,
        resources: &[&mtl::Res],
        usage: crate::mtl::ResUsage,
        stages: mtl::RenderStages,
    ) {
        unsafe { self.use_resources_count(resources.as_ptr(), resources.len(), usage, stages) };
    }

    /// Set the offset within the current global buffer for all vertex shaders at the given bind point index.
    ///
    /// Call this method to change the offset you specified when binding a single buffer with
    /// the set_vertex_buf_at method or multiple buffers with the setVertexBuffers:offsets:withRange:
    /// method. You can also use this method to specify a data offset after binding data directly
    /// to the vertex shader with the setVertexBytes:length:atIndex: method. Don’t rebind a buffer
    /// or block of data if you’re only updating its offset.
    ///
    /// For buffers in the device address space, align the offset to the data type consumed by
    /// the vertex shader (which is always less than or equal to 16 bytes).
    ///
    /// For buffers in the constant address space, align the offset to 256 bytes in macOS.
    /// In iOS, align the offset to the maximum of either the data type consumed by the vertex
    /// shader, or 4 bytes. A 16-byte alignment is safe in iOS if you don’t need to consider
    /// the data type.
    ///
    /// # Arguments:
    /// * `offset` - Where the data begins, in bytes, from the start of the buffer.
    /// * `index` - The index in the buffer argument table.
    #[objc::msg_send(setVertexBufferOffset:atIndex:)]
    pub fn set_vertex_buf_offset_at(&mut self, offset: usize, index: usize);

    #[objc::msg_send(setVertexBuffers:offsets:withRange:)]
    pub fn set_vertex_bufs_offsets_with_range(
        &mut self,
        bufs: *const &mtl::Buf,
        offsets: *const usize,
        range: ns::Range,
    );

    pub fn set_vertex_bufs<const N: usize>(&mut self, bufs: &[&mtl::Buf; N]) {
        self.set_vertex_bufs_offsets_with_range(
            bufs.as_ptr(),
            [0; N].as_ptr(),
            ns::Range::new(0, N),
        )
    }

    /// Set a global sampler for all vertex shaders at the given bind point index.
    #[objc::msg_send(setVertexSamplerState:atIndex:)]
    pub fn set_vertex_sampler_state_at(
        &mut self,
        sampler: Option<&mtl::SamplerState>,
        at_index: usize,
    );

    /// Set a global buffer for all fragment shaders at the given bind point index.
    #[objc::msg_send(setFragmentBuffer:offset:atIndex:)]
    pub fn set_fragment_buf_at(&mut self, buf: Option<&mtl::Buf>, offset: usize, at_index: usize);

    #[objc::msg_send(setFragmentBytes:length:atIndex:)]
    pub fn copy_bytes_to_fragment_at(&mut self, bytes: *const u8, len: usize, at_index: usize);

    #[inline]
    pub fn copy_slice_to_fragment_at<T>(&mut self, slice: &[T], at_index: usize) {
        self.copy_bytes_to_fragment_at(
            slice.as_ptr().cast(),
            std::mem::size_of_val(slice),
            at_index,
        )
    }

    #[inline]
    pub fn copy_to_fragment_at<T>(&mut self, val: &T, index: usize) {
        self.copy_bytes_to_fragment_at(val as *const T as _, std::mem::size_of::<T>(), index)
    }

    /// Set a global texture for all fragment shaders at the given bind point index.
    #[objc::msg_send(setFragmentTexture:atIndex:)]
    pub fn set_fragment_texture_at(&mut self, val: Option<&mtl::Texture>, index: usize);

    #[objc::msg_send(setFragmentTextures:withRange:)]
    pub fn set_fragment_textures_with_range(
        &mut self,
        ptr: *const Option<&mtl::Texture>,
        range: ns::Range,
    );

    #[inline]
    pub fn set_fragment_textures_with_array<const N: usize>(
        &mut self,
        arr: &[Option<&mtl::Texture>; N],
    ) {
        self.set_fragment_textures_with_range(arr.as_ptr(), ns::Range::new(0, N));
    }

    #[inline]
    pub fn set_fragment_textures_with_slice(&mut self, slice: &[Option<&mtl::Texture>]) {
        self.set_fragment_textures_with_range(slice.as_ptr(), ns::Range::new(0, slice.len()));
    }

    #[objc::msg_send(drawPrimitives:vertexStart:vertexCount:)]
    pub fn draw_primitives(
        &mut self,
        primitive_type: Primitive,
        vertex_start: usize,
        vertex_count: usize,
    );

    #[objc::msg_send(drawPrimitives:vertexStart:vertexCount:instanceCount:)]
    pub fn draw_primitives_instance_count(
        &mut self,
        primitive_type: Primitive,
        vertex_start: usize,
        vertex_count: usize,
        instance_count: usize,
    );

    #[objc::msg_send(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:)]
    pub fn draw_indexed_primitives_instance_count(
        &mut self,
        primitive_type: mtl::Primitive,
        index_count: usize,
        index_type: mtl::IndexType,
        index_buffer: &mtl::Buf,
        index_buffer_offset: usize,
        instance_count: usize,
    );

    #[objc::msg_send(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:)]
    pub fn draw_indexed_primitives_index_type_index_count_instance_count(
        &mut self,
        primitive_type: mtl::Primitive,
        index_count: usize,
        index_type: mtl::IndexType,
        index_buf: &mtl::Buf,
        index_buf_offset: usize,
        inst_count: usize,
        base_vertex: isize,
        base_instance: usize,
    );

    #[inline]
    pub fn draw_indexed_triangles_u16(
        &mut self,
        index_buf: &mtl::Buf,
        index_range: &std::ops::Range<usize>,
        inst_range: &std::ops::Range<usize>,
    ) {
        self.draw_indexed_primitives_index_type_index_count_instance_count(
            mtl::Primitive::Triangle,
            index_range.len(),
            mtl::IndexType::U16,
            index_buf,
            index_range.start * std::mem::size_of::<u16>(),
            inst_range.len(),
            0, // base vertex,
            inst_range.start,
        );
    }

    #[inline]
    pub fn draw_indexed_triangles_u32(
        &mut self,
        index_buf: &mtl::Buf,
        index_range: &std::ops::Range<usize>,
        instance_range: &std::ops::Range<usize>,
    ) {
        self.draw_indexed_primitives_index_type_index_count_instance_count(
            mtl::Primitive::Triangle,
            index_range.len(),
            mtl::IndexType::U32,
            index_buf,
            index_range.start * std::mem::size_of::<u32>(),
            instance_range.len(),
            0, // base vertex,
            instance_range.start,
        );
    }

    #[objc::msg_send(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:)]
    pub fn draw_indexed_primitives(
        &mut self,
        primitive_type: mtl::Primitive,
        index_count: usize,
        index_type: mtl::IndexType,
        index_buf: &mtl::Buf,
        index_buf_offset: usize,
    );

    #[objc::msg_send(updateFence:afterStages:)]
    pub fn update_fence_after_stages(&self, fence: &mtl::Fence, stages: mtl::RenderStages);

    #[objc::msg_send(waitForFence:beforeStages:)]
    pub fn wait_fence_before_stages(&self, fence: &mtl::Fence, stages: mtl::RenderStages);

    #[objc::msg_send(dispatchThreadsPerTile:)]
    #[api::available(macos = 11.0, maccatalyst = 14.0, ios = 11.0, tvos = 14.5)]
    pub fn dispatch_threads_per_tile(&self, val: mtl::Size);

    #[objc::msg_send(tileWidth)]
    #[api::available(macos = 11.0, maccatalyst = 14.0, ios = 11.0, tvos = 14.5)]
    pub fn tile_width(&self) -> usize;

    #[objc::msg_send(tileHeight)]
    #[api::available(macos = 11.0, maccatalyst = 14.0, ios = 11.0, tvos = 14.5)]
    pub fn tile_height(&self) -> usize;

    /// Set the data (by copy) for a given tile buffer binding point.  This will remove any existing mtl::Buffer from the binding point.
    ///
    /// The method is equivalent to creating an mtl::Buffer instance that contains the same data as bytes and calling
    /// the `set_tile_buf_at` method. However, this method avoids the overhead of creating a buffer to store your data;
    /// instead, Metal manages the data.
    ///
    /// Important:
    ///
    /// Only call this method for single-use data that’s smaller than 4 KB.
    #[objc::msg_send(setTileBytes:length:atIndex:)]
    #[api::available(macos = 11.0, maccatalyst = 14.0, ios = 11.0, tvos = 14.5)]
    pub fn copy_bytes_to_tile_at(
        &mut self,
        bytes: *const std::ffi::c_void,
        len: ns::UInteger,
        at_index: ns::UInteger,
    );

    #[inline]
    #[api::available(macos = 11.0, maccatalyst = 14.0, ios = 11.0, tvos = 14.5)]
    pub fn copy_slice_to_tile_at<T>(&mut self, slice: &[T], index: usize) {
        self.copy_bytes_to_tile_at(slice.as_ptr().cast(), std::mem::size_of_val(slice), index)
    }

    #[inline]
    #[api::available(macos = 11.0, maccatalyst = 14.0, ios = 11.0, tvos = 14.5)]
    pub fn copy_to_tile_at<T>(&mut self, val: &T, index: usize) {
        self.copy_bytes_to_tile_at(val as *const T as _, std::mem::size_of::<T>(), index)
    }

    #[objc::msg_send(setTileBuffer:offset:atIndex:)]
    #[api::available(macos = 11.0, maccatalyst = 14.0, ios = 11.0, tvos = 14.5)]
    pub fn set_tile_buf_at(&mut self, buf: Option<&mtl::Buf>, offset: usize, index: usize);

    #[objc::msg_send(setTileTexture:atIndex:)]
    #[api::available(macos = 11.0, maccatalyst = 14.0, ios = 11.0, tvos = 14.5)]
    pub fn set_tile_texture_at(&mut self, texture: Option<&mtl::Texture>, index: usize);

    #[objc::msg_send(setTileTextures:withRange:)]
    #[api::available(macos = 11.0, maccatalyst = 14.0, ios = 11.0, tvos = 14.5)]
    pub fn set_tile_textures_with_range(
        &mut self,
        ptr: *const Option<&mtl::Texture>,
        range: ns::Range,
    );

    #[inline]
    #[api::available(macos = 11.0, maccatalyst = 14.0, ios = 11.0, tvos = 14.5)]
    pub fn set_tile_textures_with_slice(&mut self, textures: &[Option<&mtl::Texture>]) {
        self.set_tile_textures_with_range(textures.as_ptr(), ns::Range::new(0, textures.len()));
    }

    #[inline]
    #[api::available(macos = 11.0, maccatalyst = 14.0, ios = 11.0, tvos = 14.5)]
    pub fn set_tile_textures_with_array<const N: usize>(
        &mut self,
        textures: &[Option<&mtl::Texture>; N],
    ) {
        self.set_tile_textures_with_range(textures.as_ptr(), ns::Range::new(0, N));
    }

    #[objc::msg_send(executeCommandsInBuffer:withRange:)]
    #[api::available(macos = 10.14, ios = 12.0)]
    pub fn execute_cmds_in_buf_with_range(&mut self, icb: &mtl::IndirectCmdBuf, range: ns::Range);

    #[inline]
    #[api::available(macos = 10.14, ios = 12.0)]
    pub fn execute_cmds_in_buf(
        &mut self,
        icb: &mtl::IndirectCmdBuf,
        range: std::ops::Range<usize>,
    ) {
        self.execute_cmds_in_buf_with_range(icb, ns::Range::new(range.start, range.len()))
    }
}
