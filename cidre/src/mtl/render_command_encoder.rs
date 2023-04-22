use crate::{define_mtl, define_obj_type, define_options, mtl, ns, objc};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(usize)]
pub enum PrimitiveType {
    /// Rasterize a point at each vertex. The vertex shader must provide [[point_size]],
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(usize)]
pub enum VisibilityResultMode {
    Disabled = 0,
    Boolean = 1,
    Counting = 2,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct ScissorRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct ViewPort {
    // double originX, originY, width, height, znear, zfar;
    pub origin_x: f64,
    pub origin_y: f64,
    pub width: f64,
    pub height: f64,
    pub z_near: f64,
    pub z_far: f64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum CullMode {
    None = 0,
    Front = 1,
    Back = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum Winding {
    Clockwise = 0,
    CounterClockwise = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum DepthClipMode {
    Clip = 0,
    Clamp = 1,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum TriangleFillMode {
    Fill = 0,
    Lines = 1,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DrawPrimitivesIndirectArguments {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub vertex_start: u32,
    pub base_instance: u32,
}

#[repr(C)]
pub struct DrawIndexedPrimitivesIndirectArguments {
    pub index_count: u32,
    pub instance_count: u32,
    pub index_start: u32,
    pub base_vertex: i32,
    pub base_instance: u32,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct VertexAmplificationViewMapping {
    pub viewport_array_index_offset: u32,
    pub render_target_array_index_offset: u32,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct DrawPatchIndirectArguments {
    pub patch_count: u32,
    pub instance_count: u32,
    pub patch_start: u32,
    pub base_instance: u32,
}

#[repr(C)]
pub struct QuadTessellationFactorsHalf {
    /* NOTE: edgeTessellationFactor and insideTessellationFactor are interpreted as half (16-bit floats) */
    pub edge_tessellation_factor: [u16; 4],
    pub inside_tessellation_factor: [u16; 2],
}

#[repr(C)]
pub struct TriangleTessellationFactorsHalf {
    /* NOTE: edgeTessellationFactor and insideTessellationFactor are interpreted as half (16-bit floats) */
    pub edge_tessellation_factor: [u16; 3],
    pub inside_tessellation_factor: u16,
}

define_options!(RenderStages(usize));

impl RenderStages {
    pub const VERTEX: Self = Self(1usize << 0);
    pub const FRAGMENT: Self = Self(1usize << 1);
    pub const TILE: Self = Self(1usize << 2);
    pub const OBJECT: Self = Self(1usize << 3);
    pub const MESH: Self = Self(1usize << 4);
}

define_obj_type!(RenderCmdEncoder(mtl::CmdEncoder));

impl RenderCmdEncoder {
    define_mtl!(use_resource, use_resources, use_heap);

    #[objc::msg_send(setRenderPipelineState:)]
    pub fn set_render_ps(&mut self, value: &mtl::RenderPipelineState);

    #[objc::msg_send(setVertexBytes:length:atIndex:)]
    pub fn set_vertex_bytes_at(
        &mut self,
        bytes: *const u8,
        length: ns::UInteger,
        at_index: ns::UInteger,
    );

    #[inline]
    pub fn set_vertex_slice_at(&mut self, slice: &[u8], at_index: usize) {
        self.set_vertex_bytes_at(slice.as_ptr(), slice.len(), at_index)
    }

    #[objc::msg_send(setVertexBuffer:offset:atIndex:)]
    pub fn set_vertex_buf_at(&mut self, buf: Option<&mtl::Buf>, offset: usize, at_index: usize);

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

    /// Set a global texture for all fragment shaders at the given bind point index.
    #[objc::msg_send(setFragmentTexture:atIndex:)]
    pub fn set_fragment_texture_at(&mut self, texture: Option<&mtl::Texture>, at_index: usize);

    #[objc::msg_send(drawPrimitives:vertexStart:vertexCount:instanceCount:)]
    pub fn draw_primitives_instance_count(
        &mut self,
        primitive_type: PrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
        instance_count: usize,
    );

    #[objc::msg_send(drawPrimitives:vertexStart:vertexCount:)]
    pub fn draw_primitives(
        &mut self,
        primitive_type: PrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
    );
}
