use crate::{arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSStatusBar")]
    pub StatusBar(ns::Id),
    sym NSStatusBar
);

impl StatusBar {
    #[objc::msg_send(systemStatusBar)]
    pub fn system() -> arc::R<Self>;

    #[objc::msg_send(isVertical)]
    pub fn is_vertical(&self) -> bool;

    #[objc::msg_send(thickness)]
    pub fn thickness(&self) -> cg::Float;
}
