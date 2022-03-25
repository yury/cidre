use crate::{define_obj_type, objc::Id};

use super::{CommandEncoder, Fence};

define_obj_type!(ComputeCommandEncoder(CommandEncoder));

impl ComputeCommandEncoder {
    #[inline]
    pub fn update_fence(&self, fence: &Fence) {
        unsafe { wsel_updateFence(self, fence) }
    }

    #[inline]
    pub fn wait_for_fence(&self, fence: &Fence) {
        unsafe { wsel_waitForFence(self, fence) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn wsel_updateFence(id: &Id, fence: &Fence);
    fn wsel_waitForFence(id: &Id, fence: &Fence);
}
