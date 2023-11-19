use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum Usage {
    ///  Default usage
    None = 0,

    /// Enable refitting for this acceleration structure. Note that this may reduce
    /// acceleration structure quality.
    Refit = (1 << 0),

    /// Prefer building this acceleration structure quickly at the cost of reduced ray
    /// tracing performance.
    PreferFastBuild = (1 << 1),

    /// Enable extended limits for this acceleration structure, possibly at the cost of
    /// reduced ray tracing performance.
    ExtendedLimits = (1 << 2),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum InstanceOpts {
    /// No options
    None = 0,

    /// Disable triangle back or front face culling
    DisableTriangleCulling = (1 << 0),

    /// Disable triangle back or front face culling
    TriangleFrontFacingWindingCounterClockwise = (1 << 1),

    /// Geometry is opaque
    Opaque = (1 << 2),

    /// Geometry is non-opaque
    NonOpaque = (1 << 3),
}

define_obj_type!(pub Desc(ns::Id));

impl Desc {
    #[objc::msg_send(usage)]
    pub fn usage(&self) -> Usage;

    #[objc::msg_send(setUsage:)]
    pub fn set_usage(&mut self, val: Usage);
}

define_obj_type!(
    pub GeometryDesc(Desc),
    MTL_ACCELERATION_STRUCTURE_GEOMETRY_DESCRIPTOR
);

impl GeometryDesc {
    #[objc::msg_send(intersectionFunctionTableOffset)]
    pub fn intersection_fn_table_offset(&self) -> usize;

    #[objc::msg_send(setIntersectionFunctionTableOffset:)]
    pub fn set_intersection_fn_table_offset(&mut self, val: usize);

    #[objc::msg_send(opaque)]
    pub fn opaque(&self) -> bool;

    #[objc::msg_send(setOpaque:)]
    pub fn set_opaque(&mut self, val: bool);

    #[objc::msg_send(allowDuplicateIntersectionFunctionInvocation)]
    pub fn allow_duplicate_intersection_fn_invocation(&self) -> bool;

    #[objc::msg_send(setAllowDuplicateIntersectionFunctionInvocation:)]
    pub fn set_allow_duplicate_intersection_fn_invocation(&mut self, val: bool);

    define_mtl!(label, set_label);

    #[objc::msg_send(primitiveDataBuffer)]
    pub fn primitive_data_buf(&self) -> Option<&mtl::Buf>;

    #[objc::msg_send(setPrimitiveDataBuffer:)]
    pub fn set_primitive_data_buf(&mut self, val: Option<&mtl::Buf>);

    #[objc::msg_send(primitiveDataBufferOffset)]
    pub fn primitive_data_buf_offset(&self) -> usize;

    #[objc::msg_send(setPrimitiveDataBufferOffset:)]
    pub fn set_primitive_data_buf_offset(&mut self, val: usize);

    #[objc::msg_send(primitiveDataStride)]
    pub fn primitive_data_stride(&self) -> usize;

    #[objc::msg_send(setPrimitiveDataStride:)]
    pub fn set_primitive_data_stride(&mut self, val: usize);

    #[objc::msg_send(primitiveDataElementSize)]
    pub fn primitive_data_element_size(&self) -> usize;

    #[objc::msg_send(setPrimitiveDataElementSize:)]
    pub fn set_primitive_data_element_size(&self, val: usize);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum MotionBorderMode {
    /// Motion is stopped. (default)
    Clamp = 0,

    /// Object disappears
    Vanish = 1,
}

define_obj_type!(
    pub TriangleGeometryDesc(GeometryDesc),
    MTL_ACCELERATION_STRUCTURE_TRIANGLE_GEOMETRY_DESCRIPTOR
);

impl TriangleGeometryDesc {
    #[objc::msg_send(vertexBuffer)]
    pub fn vertex_buf(&self) -> Option<&mtl::Buf>;

    #[objc::msg_send(setVertexBuffer:)]
    pub fn set_vertex_buf(&mut self, val: Option<&mtl::Buf>);

    #[objc::msg_send(vertexBufferOffset)]
    pub fn vertex_buf_offset(&self) -> usize;

    #[objc::msg_send(setVertexBufferOffset:)]
    pub fn set_vertex_buf_offset(&self, val: usize);

    #[objc::msg_send(vertexFormat)]
    pub fn vertex_format(&self) -> mtl::AttrFormat;

    #[objc::msg_send(setVertexFormat:)]
    pub fn set_vertex_format(&mut self, val: mtl::AttrFormat);

    #[objc::msg_send(vertexStride)]
    pub fn vertex_stride(&self) -> usize;

    #[objc::msg_send(setVertexStride:)]
    pub fn set_vertex_stride(&mut self, val: usize);

    #[objc::msg_send(indexBuffer)]
    pub fn index_buf(&self) -> Option<&mtl::Buf>;

    #[objc::msg_send(setIndexBuffer:)]
    pub fn set_index_buf(&mut self, val: Option<&mtl::Buf>);

    #[objc::msg_send(indexBufferOffset)]
    pub fn index_buf_offset(&self) -> usize;

    #[objc::msg_send(setIndexBufferOffset:)]
    pub fn set_index_buf_offset(&mut self, val: usize);

    #[objc::msg_send(indexType)]
    pub fn index_type(&self) -> mtl::IndexType;

    #[objc::msg_send(setIndexType:)]
    pub fn set_index_type(&mut self, val: mtl::IndexType);

    #[objc::msg_send(triangleCount)]
    pub fn triangle_count(&self) -> usize;

    #[objc::msg_send(setTriangleCount:)]
    pub fn set_triangle_count(&mut self, val: usize);

    #[objc::msg_send(transformationMatrixBuffer)]
    pub fn tranformation_matrix_buf(&self) -> Option<&mtl::Buf>;

    #[objc::msg_send(setTransformationMatrixBuffer:)]
    pub fn set_tranformation_matrix_buf(&mut self, val: Option<&mtl::Buf>);

    #[objc::msg_send(transformationMatrixBufferOffset)]
    pub fn tranformation_matrix_buf_offset(&self) -> usize;

    #[objc::msg_send(setTransformationMatrixBufferOffset:)]
    pub fn set_tranformation_matrix_buf_offset(&mut self, val: usize);
}

define_obj_type!(
    pub MotionBoundingBoxGeometryDesc(GeometryDesc),
    MTL_ACCELERATION_STRUCTURE_MOTION_BOUNDING_BOX_GEOMETRY_DESCRIPTOR
);

impl MotionBoundingBoxGeometryDesc {
    //
}

define_obj_type!(pub AccelerationStructure(mtl::Resource));

impl AccelerationStructure {
    #[objc::msg_send(size)]
    pub fn size(&self) -> usize;

    define_mtl!(gpu_resource_id);
}

extern "C" {
    static MTL_ACCELERATION_STRUCTURE_GEOMETRY_DESCRIPTOR: &'static objc::Class<GeometryDesc>;
    static MTL_ACCELERATION_STRUCTURE_TRIANGLE_GEOMETRY_DESCRIPTOR:
        &'static objc::Class<TriangleGeometryDesc>;
    static MTL_ACCELERATION_STRUCTURE_MOTION_BOUNDING_BOX_GEOMETRY_DESCRIPTOR:
        &'static objc::Class<TriangleGeometryDesc>;

}
