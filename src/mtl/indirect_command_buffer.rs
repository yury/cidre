use crate::{define_mtl, define_obj_type, msg_send, objc::Id};

use super::Resource;

#[repr(C)]
pub struct ExecutionRange {
    pub location: i32,
    pub length: i32,
}

define_obj_type!(Descriptor(Id));
define_obj_type!(IndirectCommandBuffer(Resource));

impl IndirectCommandBuffer {
    define_mtl!(gpu_resouce_id);
}
