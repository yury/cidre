use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum IntersectionFnSignature {
    ///
    /// No signature
    ///
    None = 0,

    ///
    /// The intersection functions are entitled to read the built-in instance_id as described in
    /// the Metal Shading Language Guide.
    ///
    Instancing = (1 << 0),

    ///
    /// The triangle intersection functions are entitled to to read the built-in barycentric_coord
    /// and front_facing as described in the Metal Shading Language Guide.
    ///
    TriangleData = (1 << 1),

    ///
    /// The intersection functions are entitled to query world_space_origin and
    /// world_space_direction as described in the Metal Shading Language Guide.
    ///
    WorldSpaceData = (1 << 2),

    ///
    /// The intersection functions may be called from intersectors using the
    /// instance_motion intersection tag as described in the Metal Shading Language Guide.
    ///
    InstanceMotion = (1 << 3),

    ///
    /// The intersection functions are entitled to query time, motion_start_time,
    /// motion_end_time and key_frame_count as described in the Metal Shading Language Guide.
    ///
    PrimitiveMotion = (1 << 4),

    ///
    /// The intersection functions may be called from intersectors using the
    /// extended_limits intersection tag as described in the Metal Shading Language Guide.
    ///
    ExtendedLimits = (1 << 5),
}

define_obj_type!(pub Desc(ns::Id), MTL_INTERSECTION_FUNCTION_TABLE_DESCRIPTOR);

impl Desc {
    /// The number of functions in the table.
    #[objc::msg_send(functionCount)]
    pub fn fn_count(&self) -> usize;

    #[objc::msg_send(setFunctionCount:)]
    pub fn set_fn_count(&mut self, val: usize);
}

define_obj_type!(pub IntersectionFnTable(mtl::Res));

impl IntersectionFnTable {
    define_mtl!(gpu_res_id);

    #[objc::msg_send(setBuffer:atIndex:)]
    pub fn set_buf_at(&mut self, buf: Option<&mtl::Buf>, index: usize);

    #[objc::msg_send(setBuffers:offsets:withRange:)]
    pub fn set_bufs(
        &mut self,
        buffers: *const Option<&mtl::Buf>,
        offsets: *const usize,
        range: ns::Range,
    );

    #[objc::msg_send(setFunction:atIndex:)]
    pub fn set_fn_at<H: mtl::FnHandle>(&mut self, function: Option<&H>, index: usize);

    #[objc::msg_send(setFunctions:withRange:)]
    pub fn set_fns_with_range<H: mtl::FnHandle>(
        &mut self,
        functions: *const Option<&H>,
        range: ns::Range,
    );

    #[objc::msg_send(setOpaqueTriangleIntersectionFunctionWithSignature:atIndex:)]
    pub fn set_opaque_triangle_intersection_fn_with_signature(
        &mut self,
        signature: mtl::IntersectionFnSignature,
        index: usize,
    );

    #[objc::msg_send(setOpaqueTriangleIntersectionFunctionWithSignature:withRange:)]
    pub fn set_opaque_triangle_intersection_fn_with_signature_with_range(
        &mut self,
        signature: mtl::IntersectionFnSignature,
        range: ns::Range,
    );

    #[objc::msg_send(setVisibleFunctionTable:atBufferIndex:)]
    pub fn set_visible_fn_table(
        &mut self,
        function_table: Option<&mtl::VisibleFnTable>,
        buffer_index: usize,
    );

    #[objc::msg_send(setVisibleFunctionTables:withBufferRange:)]
    pub fn set_visible_fn_tables(
        &mut self,
        function_tables: *const Option<&mtl::VisibleFnTable>,
        buffer_range: ns::Range,
    );
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL_INTERSECTION_FUNCTION_TABLE_DESCRIPTOR: &'static objc::Class<Desc>;
}
