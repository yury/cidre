use crate::{define_mtl, define_obj_type};

use super::CommandEncoder;

define_obj_type!(ComputeCommandEncoder(CommandEncoder));

impl ComputeCommandEncoder {
    define_mtl!(
        update_fence,
        wait_for_fence,
        use_resource,
        use_resources,
        use_heap
    );
}

#[link(name = "mtl", kind = "static")]
extern "C" {}
