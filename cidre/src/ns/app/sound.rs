use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "NSSound")]
    pub Sound(ns::Id)
);
