use crate::ns;
use crate::{define_mtl, define_obj_type};
pub struct ResourceUsage(pub usize);

impl ResourceUsage {
    pub const READ: Self = Self(1 << 0);
    pub const WRITE: Self = Self(1 << 1);
    pub const SAMPLE: Self = Self(1 << 2);
}

#[repr(transparent)]
pub struct BarrierScope(pub usize);

impl BarrierScope {
    pub const BUFFERS: Self = Self(1 << 0);
    pub const TEXTURES: Self = Self(1 << 1);
    pub const RENDER_TARGETS: Self = Self(1 << 2);
}

define_obj_type!(CommandEncoder(ns::Id));

impl CommandEncoder {
    define_mtl!(device, mut label);

    pub fn end_encoding(&mut self) {
        unsafe { wsel_endEncoding(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn wsel_endEncoding(id: &mut ns::Id);
}
