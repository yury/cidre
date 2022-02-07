use crate::{cf, os};

pub type Session = cf::Type;

impl Session {

  #[inline]
  pub fn set_property(&mut self, key: &cf::String, value: Option<&cf::Type>) -> os::Status {
    unsafe {
      VTSessionSetProperty(self, key, value)
    }
  }

  #[inline]
  pub fn copy_property<'a>(&self, key: &cf::String, allocator: Option<&cf::Allocator>) -> Option<cf::Retained<'a, cf::Type>> {
    unsafe {
      VTSessionCopyProperty(self, key, allocator)
    }
  }
}

extern "C" {
  fn VTSessionSetProperty(session: &mut Session, property_key: &cf::String, property_value: Option<&cf::Type>) -> os::Status;
  fn VTSessionCopyProperty<'a>(session: &Session, property_key: &cf::String, allocator: Option<&cf::Allocator>) -> Option<cf::Retained<'a, cf::Type>>;
}