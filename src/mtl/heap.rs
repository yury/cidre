use crate::{define_mtl, define_obj_type, msg_send};

use crate::ns::Id;

define_obj_type!(Heap(Id));

impl Heap {
    define_mtl!(device, label, set_label);
}

impl Heap {
    pub fn size(&self) -> usize {
        msg_send!("common", self, size)
    }
}
