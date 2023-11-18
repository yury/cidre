use crate::{define_mtl, define_obj_type, define_options, ns, objc};

define_options!(
    /// Describes how a resource will be used by a shader through an argument buffer
    #[doc(alias = "MTLResourceUsage")]
    ResourceUsage(usize)
);

impl ResourceUsage {
    /// An option that enables reading from the resource.
    pub const READ: Self = Self(1 << 0);

    /// An option that enables writing to the resource.
    pub const WRITE: Self = Self(1 << 1);

    /// An option that enables sampling from the resource.
    #[deprecated(since = "0.1.0", note = "use mtl::ResourceUsage::READ instead")]
    pub const SAMPLE: Self = Self(1 << 2);
}

define_options!(
    /// Describes the types of resources that the a barrier operates on
    #[doc(alias = "MTLBarrierScope")]
    BarrierScope(usize)
);

impl BarrierScope {
    /// The barrier affects any buffer objects.
    pub const BUFFERS: Self = Self(1 << 0);

    /// The barrier affects textures.
    pub const TEXTURES: Self = Self(1 << 1);

    /// The barrier affects any render targets.
    pub const RENDER_TARGETS: Self = Self(1 << 2);
}

define_obj_type!(
    #[doc(alias = "MTLCommandEncoder")]
    CmdEncoder(ns::Id)
);

impl CmdEncoder {
    define_mtl!(device, label, set_label, push_debug_group, pop_debug_group);

    #[objc::msg_send(endEncoding)]
    pub fn end_encoding(&mut self);

    #[objc::msg_send(insertDebugSignpost:)]
    pub fn insert_debug_signpost(&mut self, signpost: &ns::String);
}
