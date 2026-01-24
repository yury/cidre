use crate::{define_obj_type, ns, objc};

#[objc::protocol(UISpringLoadedInteractionContext)]
pub trait SpringLoadedInteractionContext: objc::Obj {}

define_obj_type!(
    pub AnySpringLoadedInteractionContext(ns::Id)
);

impl SpringLoadedInteractionContext for AnySpringLoadedInteractionContext {}
