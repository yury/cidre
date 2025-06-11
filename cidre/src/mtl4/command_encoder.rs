use crate::{arc, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "MTL4VisibilityOptions")]
    pub VisibilityOpts(isize)
);

define_obj_type!(
    #[doc(alias = "MTL4CommandEncoder")]
    pub CmdEncoder(ns::Id)
);

impl CmdEncoder {
    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, val: Option<&ns::String>);
}
