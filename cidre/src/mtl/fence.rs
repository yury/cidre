use crate::{define_mtl, define_obj_type, ns};

define_obj_type!(Fence(ns::Id));

impl Fence {
    define_mtl!(device, label, set_label);
}
