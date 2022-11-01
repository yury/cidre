use crate::{define_mtl, define_obj_type, msg_send, ns};

use super::CommandBuffer;

define_obj_type!(CommandQueue(ns::Id));

impl CommandQueue {
    define_mtl!(device, label, set_label);

    #[inline]
    pub fn command_buffer<'ar>(&self) -> Option<&'ar CommandBuffer> {
        msg_send!("mtl", self, sel_commandBuffer)
    }

    #[inline]
    pub fn command_buffer_with_unretained_refs<'ar>(&self) -> Option<&'ar CommandBuffer> {
        msg_send!("mtl", self, sel_commandBufferWithUnretainedReferences)
    }
}
