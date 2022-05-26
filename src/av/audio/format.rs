use crate::{define_obj_type, ns};

#[repr(usize)]
pub enum CommonFormat {
    Other = 0,
    PCMFloat32 = 1,
    PCMFloat64 = 2,
    PCMInt16 = 3,
    PCMInt32 = 4,
}

define_obj_type!(Format(ns::Id));
