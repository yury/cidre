use crate::{define_mtl, define_obj_type, ns, objc};

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ResourceUsage(pub usize);

impl ResourceUsage {
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1);
    pub const SAMPLE: Self = Self(1 << 2);
}

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct BarrierScope(pub usize);

impl BarrierScope {
    pub const BUFFERS: Self = Self(1 << 0);
    pub const TEXTURES: Self = Self(1 << 1);
    pub const RENDER_TARGETS: Self = Self(1 << 2);
}

define_obj_type!(CmdEncoder(ns::Id));

impl CmdEncoder {
    define_mtl!(device, label, set_label, push_debug_group, pop_debug_group);

    #[objc::msg_send(endEncoding)]
    pub fn end_encoding(&mut self);

    #[objc::msg_send(insertDebugSignpost:)]
    pub fn insert_debug_signpost(&mut self, signpost: &ns::String);
}
