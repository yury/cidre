use crate::cf::Retained;
use crate::define_obj_type;

use crate::dispatch::Object;

define_obj_type!(Group(Object));

impl Group {
  pub fn new<'a>() -> Retained<'a, Self> {
    unsafe { dispatch_group_create() }
  }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
  fn dispatch_group_create<'a>() -> Retained<'a, Group>;
}