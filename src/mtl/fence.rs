use crate::{define_obj_type, objc::Id, define_mtl};

define_obj_type!(Fence(Id));

impl Fence {
    define_mtl!(device, get label);
}