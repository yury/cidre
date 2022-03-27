use crate::{define_mtl, define_obj_type, objc::Id};

define_obj_type!(Fence(Id));

impl Fence {
    define_mtl!(device, label, set_label);
}
