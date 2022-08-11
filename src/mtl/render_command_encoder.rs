use std::ffi::c_void;

use crate::mtl::RenderPipelineState;
use crate::{define_mtl, msg_send};
use crate::{define_obj_type, objc::Id};

use super::{Buffer, CommandEncoder, Texture};

#[repr(usize)]
pub enum PrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
}

#[repr(usize)]
pub enum VisibilityResultMode {
    Disabled = 0,
    Boolean = 1,
    Counting = 2,
}

#[repr(C)]
pub struct ScissorRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

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

#[repr(usize)]
pub enum CullMode {
    None = 0,
    Front = 1,
    Back = 2,
}

#[repr(usize)]
pub enum Winding {
    Clockwise = 0,
    CounterClockwise = 1,
}

#[repr(usize)]
pub enum DepthClipMode {
    Clip = 0,
    Clamp = 1,
}

#[repr(usize)]
pub enum TriangleFillMode {
    Fill = 0,
    Lines = 1,
}

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

#[repr(C)]
pub struct VertexAmplificationViewMapping {
    pub viewport_array_index_offset: u32,
    pub render_target_array_index_offset: u32,
}

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

#[repr(transparent)]
pub struct RenderStages(usize);

impl RenderStages {
    pub const VERTEX: Self = Self(1usize << 0);
    pub const FRAGMENT: Self = Self(1usize << 1);
    pub const TILE: Self = Self(1usize << 2);
}

define_obj_type!(RenderCommandEncoder(CommandEncoder));

impl RenderCommandEncoder {
    define_mtl!(use_resource, use_resources, use_heap);

    #[inline]
    pub fn set_render_pipeline_state(&mut self, state: &RenderPipelineState) {
        msg_send!("mtl", self, sel_setRenderPipelineState, state)
    }

    #[inline]
    pub fn set_vertex_bytes(&mut self, bytes: &[u8], at_index: usize) {
        unsafe { wsel_setVertexBytes(self, bytes.as_ptr() as _, bytes.len(), at_index) }
    }

    #[inline]
    pub fn set_vertex_buffer(&mut self, buffer: Option<&Buffer>, offset: usize, at_index: usize) {
        msg_send!("mtl", self, sel_setVertexBuffer_offset_atIndex, buffer, offset, at_index)
    }

    #[inline]
    pub fn set_fragment_buffer(&mut self, buffer: Option<&Buffer>, offset: usize, at_index: usize) {
        msg_send!("mtl", self, sel_setFragmentBuffer_offset_atIndex, buffer, offset, at_index)
    }

    #[inline]
    pub fn set_fragment_texture_at(&mut self, texture: Option<&Texture>, at_index: usize) {
        msg_send!("mtl", self, sel_setFragmentTexture_atIndex, texture, at_index)
    }

    #[inline]
    pub fn draw_primitives_instance_count(
        &mut self,
        primitive_type: PrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
        instance_count: usize,
    ) {
        msg_send!(
            "mtl",
            self,
            sel_drawPrimitives_vertexStart_vertexCount_instanceCount,
            primitive_type,
            vertex_start,
            vertex_count,
            instance_count
        )
    }

    #[inline]
    pub fn draw_primitives(
        &mut self,
        primitive_type: PrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
    ) {
        msg_send!(
            "mtl",
            self,
            sel_drawPrimitives_vertexStart_vertexCount,
            primitive_type,
            vertex_start,
            vertex_count
        )
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn wsel_setVertexBytes(id: &mut Id, bytes: *const c_void, length: usize, at_index: usize);
}
