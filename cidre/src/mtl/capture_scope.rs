use crate::{arc, define_obj_type, mtl, ns, objc};

define_obj_type!(
    pub CaptureScope(ns::Id)
);

impl CaptureScope {
    #[objc::msg_send(beginScope)]
    pub fn begin(&mut self);

    #[objc::msg_send(endScope)]
    pub fn end(&mut self);

    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(device)]
    pub fn device(&self) -> arc::R<mtl::Device>;

    #[objc::msg_send(commandQueue)]
    pub fn cmd_queue(&self) -> Option<arc::R<mtl::CmdQueue>>;
}
