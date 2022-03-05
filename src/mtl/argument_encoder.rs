use crate::ns::Id;
use crate::{define_mtl_device_and_label, define_obj_type};

define_obj_type!(ArgumentEncoder(Id));

impl ArgumentEncoder {
    define_mtl_device_and_label!();
}
