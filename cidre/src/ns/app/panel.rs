use crate::{arc, define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "NSPanel")]
    pub Panel(ns::Window),
    sym NSPanel
);
