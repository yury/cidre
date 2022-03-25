use std::ffi::c_void;

use crate::mtl::RenderPipelineState;
use crate::{define_obj_type, objc::Id};

use super::{Buffer, CommandEncoder, Heap, Resource, ResourceUsage};

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
    pub fn set_render_pipeline_state(&mut self, state: &RenderPipelineState) {
        unsafe { wsel_setRenderPipelineState(self, state) }
    }

    pub fn set_vertex_bytes(&mut self, bytes: &[u8], at_index: usize) {
        unsafe { wsel_setVertexBytes(self, bytes.as_ptr() as _, bytes.len(), at_index) }
    }

    pub fn set_vertex_buffer(&mut self, buffer: Option<&Buffer>, offset: usize, at_index: usize) {
        unsafe { wsel_setVertexBuffer(self, buffer, offset, at_index) }
    }

    pub fn set_fragment_buffer(&mut self, buffer: Option<&Buffer>, offset: usize, at_index: usize) {
        unsafe { wsel_setFragmentBuffer(self, buffer, offset, at_index) }
    }

    pub fn use_resource(&mut self, resource: &Resource, usage: ResourceUsage) {
        unsafe { wsel_useResource(self, resource, usage) }
    }

    pub fn use_resources(&mut self, resources: &[Resource], usage: ResourceUsage) {
        unsafe { wsel_useResources(self, resources.as_ptr(), resources.len(), usage) }
    }

    pub fn use_heap(&mut self, heap: &Heap) {
        unsafe { wsel_useHeap(self, heap) }
    }

    pub fn draw(
        &mut self,
        primitive_type: PrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
        instance_count: usize,
    ) {
        unsafe {
            ic_wsel_drawPrimitives(
                self,
                primitive_type,
                vertex_start,
                vertex_count,
                instance_count,
            )
        }
    }

    pub fn draw_primitives(
        &mut self,
        primitive_type: PrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
    ) {
        unsafe { wsel_drawPrimitives(self, primitive_type, vertex_start, vertex_count) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn wsel_setRenderPipelineState(id: &mut Id, state: &RenderPipelineState);
    fn wsel_setVertexBytes(id: &mut Id, bytes: *const c_void, length: usize, at_index: usize);
    fn wsel_setVertexBuffer(id: &mut Id, buffer: Option<&Buffer>, offset: usize, at_index: usize);
    fn wsel_setFragmentBuffer(id: &mut Id, buffer: Option<&Buffer>, offset: usize, at_index: usize);

    fn wsel_useResource(id: &mut Id, resource: &Resource, usage: ResourceUsage);
    fn wsel_useResources(
        id: &mut Id,
        resources: *const Resource,
        count: usize,
        usage: ResourceUsage,
    );
    fn wsel_useHeap(id: &mut Id, heap: &Heap);

    fn wsel_drawPrimitives(
        id: &mut Id,
        primitive_type: PrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
    );
    fn ic_wsel_drawPrimitives(
        id: &mut Id,
        primitive_type: PrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
        instance_count: usize,
    );
}
