use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "NSText")]
    pub Text(ns::View)
);
