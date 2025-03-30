use crate::{define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "NSTextView")]
    pub TextView(ns::Text)
);
