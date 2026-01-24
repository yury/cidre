use crate::{define_obj_type, ns, objc};

#[objc::protocol(UIDragAnimating)]
pub trait DragAnimating: objc::Obj {}

define_obj_type!(
    pub AnyDragAnimating(ns::Id)
);

impl DragAnimating for AnyDragAnimating {}
