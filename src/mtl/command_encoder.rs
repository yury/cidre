use crate::{cf, define_mtl, define_obj_type, ext_msg_send, ns};

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

define_obj_type!(CommandEncoder(ns::Id));

impl CommandEncoder {
    define_mtl!(device, label, set_label, push_debug_group, pop_debug_group);

    #[inline]
    pub fn end_encoding(&mut self) {
        ext_msg_send!("mtl", self, sel_endEncoding)
    }

    pub fn insert_debug_signpost(&mut self, signpost: &cf::String) {
        unsafe { wsel_insertDebugSignpost(self, signpost) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn wsel_insertDebugSignpost(id: &mut ns::Id, signpost: &cf::String);

}
