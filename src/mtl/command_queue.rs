use crate::{define_mtl, define_obj_type, msg_send};

use crate::ns::Id;

use super::CommandBuffer;

define_obj_type!(CommandQueue(Id));

impl CommandQueue {
    define_mtl!(device, label, set_label);

    #[inline]
    pub fn command_buffer<'pool>(&self) -> Option<&'pool CommandBuffer> {
        msg_send!("mtl", self, sel_commandBuffer)
    }
}
