use crate::{define_mtl, define_obj_type, objc::Id};

use super::{CommandEncoder, Fence};

define_obj_type!(ComputeCommandEncoder(CommandEncoder));

impl ComputeCommandEncoder {
    define_mtl!(update_fence);
    define_mtl!(wait_for_fence);
    define_mtl!(use_resource);
    define_mtl!(use_resources);
    define_mtl!(use_heap);
}

#[link(name = "mtl", kind = "static")]
extern "C" {}
