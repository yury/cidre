use crate::{arc, define_mtl, define_obj_type, ns, objc};

use super::CommandBuffer;

define_obj_type!(CommandQueue(ns::Id));

impl CommandQueue {
    define_mtl!(device, label, set_label);

    #[objc::msg_send(commandBuffer)]
    pub fn command_buffer_ar(&self) -> Option<arc::Rar<CommandBuffer>>;

    #[objc::rar_retain()]
    pub fn command_buffer(&self) -> Option<arc::R<CommandBuffer>>;

    #[objc::msg_send(commandBufferWithUnretainedReferences)]
    pub fn command_buffer_with_unretained_refs_ar(&self) -> Option<arc::Rar<CommandBuffer>>;

    #[objc::rar_retain()]
    pub fn command_buffer_with_unretained_refs(&self) -> Option<arc::R<CommandBuffer>>;
}
