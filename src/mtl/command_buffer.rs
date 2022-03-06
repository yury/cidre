use crate::{define_mtl, define_obj_type};

use crate::ns::Id;

#[repr(usize)]
pub enum Status {
  NotEnqueued = 0,
  Enqueued = 1,
  Committed = 2,
  Scheduled = 3,
  Completed = 4,
  Error = 5,
}

define_obj_type!(CommandBuffer(Id));

impl CommandBuffer {
    define_mtl!(device, mut label);

    pub fn enqueue(&self) {
      unsafe {
        wsel_enqueue(self)
      }
    }

    pub fn commit(&self) {
      unsafe {
        wsel_commit(self)
      }
    }

    pub fn wait_untint_scheduled(&self) {
      unsafe {
        wsel_waitUntilScheduled(self)
      }
    }

    pub fn wait_until_completed(&self) {
      unsafe {
        wsel_waitUntilCompleted(self)
      }
    }
}

extern "C" {
  fn wsel_enqueue(id: &Id);
  fn wsel_commit(id: &Id);
  fn wsel_waitUntilScheduled(id: &Id);
  fn wsel_waitUntilCompleted(id: &Id);

}