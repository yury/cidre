use crate::{define_mtl, define_obj_type, msg_send};

use crate::ns::Id;

#[derive(Debug, Eq, PartialEq)]
#[repr(isize)]
pub enum Type {
    Automatic = 0,
    Placement = 1,
    Sparce = 2,
}

define_obj_type!(Descriptor(Id));

impl Descriptor {
    define_mtl!(
        storage_mode,
        set_storage_mode,
        cpu_cache_mode,
        set_cpu_cache_mode,
        hazard_tracking_mode,
        set_hazard_tracking_mode,
        resource_options,
        set_resource_options
    );
}

define_obj_type!(Heap(Id));

impl Heap {
    define_mtl!(
        device,
        label,
        set_label,
        hazard_tracking_mode,
        resource_options
    );
}

impl Heap {
    pub fn size(&self) -> usize {
        msg_send!("common", self, size)
    }
}
