use crate::{define_obj_type, objc::Id, msg_send};

use super::{Resource, ResourceID};

#[repr(C)]
pub struct ExecutionRange {
    pub location: i32,
    pub length: i32,
}

define_obj_type!(Descriptor(Id));
define_obj_type!(IndirectCommandBuffer(Resource));

impl IndirectCommandBuffer {
    #[inline]
    pub fn gpu_resouce_id(&self) -> ResourceID {
        msg_send!("mtl", self, sel_gpuResourceID) 
    }
}
