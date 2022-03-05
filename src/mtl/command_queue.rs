use crate::{define_mtl_device_and_label, define_obj_type};

use crate::ns::Id;

define_obj_type!(CommandQueue(Id));

impl CommandQueue {
    define_mtl_device_and_label!();
}
