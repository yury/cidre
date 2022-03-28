use crate::{define_obj_type, objc::Id};

use super::Resource;

#[repr(C)]
pub struct ExecutionRange {
    pub location: i32,
    pub length: i32,
}

define_obj_type!(Descriptor(Id));
define_obj_type!(IndirectCommandBuffer(Resource));
