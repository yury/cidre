use crate::{define_mtl, define_obj_type};

use crate::ns::Id;

define_obj_type!(CommandQueue(Id));

impl CommandQueue {
    define_mtl!(device, mut label);
}
