use crate::{arc, define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "WKProcessPool")]
    pub ProcessPool(ns::Id),
    sym WKProcessPool
);
