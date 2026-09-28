use crate::{arc, define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "NSTextView")]
    pub TextView(ns::Text),
    sym NSTextView
);
