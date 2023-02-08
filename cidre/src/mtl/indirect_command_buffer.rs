use crate::{define_mtl, define_obj_type, mtl, ns};

#[derive(Debug)]
#[repr(C)]
pub struct ExecutionRange {
    pub location: i32,
    pub length: i32,
}

define_obj_type!(Descriptor(ns::Id));
define_obj_type!(IndirectCmdBuf(mtl::Resource));

impl IndirectCmdBuf {
    define_mtl!(gpu_resouce_id);
}
