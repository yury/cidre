use crate::{define_obj_type, ns, objc};

#[doc(alias = "CLBodyIdentifiable")]
#[objc::protocol(CLBodyIdentifiable)]
pub trait BodyIdentifiable: objc::Obj {}

define_obj_type!(pub AnyBody(ns::Id));
impl BodyIdentifiable for AnyBody {}
