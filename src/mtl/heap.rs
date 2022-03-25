use crate::{define_mtl, define_obj_type};

use crate::ns::Id;

define_obj_type!(Heap(Id));

impl Heap {
    define_mtl!(device, mut label);
}
