use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_object")]
    pub Obj(ns::Id)
);
