use crate::{define_obj_type, objc::Id, define_mtl};

define_obj_type!(Event(Id));

impl Event {
    define_mtl!(device, mut label);
}