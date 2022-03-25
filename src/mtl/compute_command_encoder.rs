use crate::{define_obj_type, ns, objc::Id};

use super::{CommandEncoder, Fence};

define_obj_type!(ComputeCommandEncoder(CommandEncoder));

impl ComputeCommandEncoder {
  pub fn update_fence(&self, fence: &Fence) {
    unsafe { wsel_updateFence(self, fence) }
  }

  pub fn wait_for_fence(&self, fence: &Fence) {
      unsafe { wsel_waitForFence(self, fence) }
  }
}


#[link(name = "mtl", kind = "static")]
extern "C" {

    fn wsel_updateFence(id: &Id, fence: &Fence);
    fn wsel_waitForFence(id: &Id, fence: &Fence);

    // fn wsel_fillBuffer(id: &Id, buffer: &Buffer, range: ns::Range, value: u8);

}
