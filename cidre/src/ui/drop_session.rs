use crate::{define_obj_type, ns, objc};

#[objc::protocol(UIDropSession)]
pub trait DropSession: objc::Obj {}

define_obj_type!(
    pub AnyDropSession(ns::Id)
);

impl DropSession for AnyDropSession {}
