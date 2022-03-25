use crate::cf::Retained;
use crate::{define_mtl, define_obj_type};

use crate::ns::Id;

use super::CommandBuffer;

define_obj_type!(CommandQueue(Id));

impl CommandQueue {
    define_mtl!(device, mut label);

    pub fn command_buffer<'new>(&self) -> Option<Retained<'new, CommandBuffer>> {
        unsafe {
            rsel_commandBuffer(self)
        }
    }
}

extern "C" {
    fn rsel_commandBuffer<'new>(id: &Id) -> Option<Retained<'new, CommandBuffer>>;
}
