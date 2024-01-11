use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_txt_record")]
    #[doc(alias = "nw_txt_record_t")]
    pub TxtRecord(ns::Id)
);
