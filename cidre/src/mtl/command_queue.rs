use crate::{arc, define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(
    /// A serial queue of command buffers to be executed by the device.
    #[doc(alias = "MTLCommandQueue")]
    pub CmdQueue(ns::Id)
);

impl CmdQueue {
    define_mtl!(device, label, set_label);

    #[objc::msg_send(commandBuffer)]
    pub fn new_cmd_buf_ar(&self) -> Option<arc::Rar<mtl::CmdBuf>>;

    #[objc::rar_retain()]
    pub fn new_cmd_buf(&self) -> Option<arc::R<mtl::CmdBuf>>;

    #[objc::msg_send(commandBufferWithUnretainedReferences)]
    pub fn new_cmd_buf_unretained_refs_ar(&self) -> Option<arc::Rar<mtl::CmdBuf>>;

    #[objc::rar_retain()]
    pub fn new_cmd_buf_unretained_refs(&self) -> Option<arc::R<mtl::CmdBuf>>;
}
