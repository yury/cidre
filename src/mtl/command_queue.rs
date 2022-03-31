use crate::cf::runtime::Autoreleased;
use crate::{define_mtl, define_obj_type, msg_send};

use crate::ns::Id;

use super::CommandBuffer;

define_obj_type!(CommandQueue(Id));

impl CommandQueue {
    define_mtl!(device, label, set_label);

    #[inline]
    pub fn command_buffer<'pool>(&self) -> Option<Autoreleased<'pool, CommandBuffer>> {
        msg_send!(self, sel_commandBuffer)
    }
}
