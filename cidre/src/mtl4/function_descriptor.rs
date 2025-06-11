use crate::{define_obj_type, ns};

define_obj_type!(
    /// Base interface for describing a Metal 4 shader function.
    #[doc(alias = "MTL4FunctionDescriptor")]
    pub FnDesc(ns::Id)
);
