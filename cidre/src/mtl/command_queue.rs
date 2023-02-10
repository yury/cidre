use crate::{arc, define_mtl, define_obj_type, ns, objc};

use super::CmdBuf;

define_obj_type!(CmdQueue(ns::Id));

impl CmdQueue {
    define_mtl!(device, label, set_label);

    #[objc::msg_send(commandBuffer)]
    pub fn new_cmd_buf_ar(&self) -> Option<&'ar CmdBuf>;

    #[objc::rar_retain()]
    pub fn new_cmd_buf(&self) -> Option<arc::R<CmdBuf>>;

    #[objc::msg_send(commandBufferWithUnretainedReferences)]
    pub fn new_cmd_buf_unretained_refs_ar(&self) -> Option<&'ar CmdBuf>;

    #[objc::rar_retain()]
    pub fn new_cmd_buf_unretained_refs(&self) -> Option<arc::R<CmdBuf>>;
}
