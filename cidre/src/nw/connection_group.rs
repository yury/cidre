use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_connection_group")]
    #[doc(alias = "nw_connection_group_t")]
    pub ConnectionGroup(ns::Id)
);
