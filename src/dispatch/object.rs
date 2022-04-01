use crate::{objc::Id, define_obj_type};

define_obj_type!(Object(Id));

impl Object {
    pub fn active(&self) {
      unsafe {
        dispatch_activate(self)
      }
    }

    pub fn suspend(&self) {
      unsafe {
        dispatch_suspend(self)
      }
    }

    pub fn resume(&self) {
      unsafe {
        dispatch_resume(self)
      }
    }
}

extern "C" {
  fn dispatch_activate(object: &Object);
  fn dispatch_suspend(object: &Object);
  fn dispatch_resume(object: &Object);
}