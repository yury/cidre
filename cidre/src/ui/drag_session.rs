use crate::{define_obj_type, ns, objc};

#[objc::protocol(UIDragSession)]
pub trait DragSession: objc::Obj {}

define_obj_type!(
    pub AnyDragSession(ns::Id)
);

impl DragSession for AnyDragSession {}
