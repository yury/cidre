use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_path")]
    #[doc(alias = "nw_path_t")]
    pub Path(ns::Id)
);
