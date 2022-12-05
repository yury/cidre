use crate::{define_mtl, define_obj_type, msg_send, mtl, ns};

#[derive(Debug)]
#[repr(C)]
pub struct ExecutionRange {
    pub location: i32,
    pub length: i32,
}

define_obj_type!(Descriptor(ns::Id));
define_obj_type!(IndirectCommandBuffer(mtl::Resource));

impl IndirectCommandBuffer {
    define_mtl!(gpu_resouce_id);
}
