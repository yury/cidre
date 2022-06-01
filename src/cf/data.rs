use crate::{cf, define_cf_type};

define_cf_type!(Data(cf::Type));
define_cf_type!(MutableData(Data));

impl Data {
  pub fn create<'a>(allocator: Option<&cf::Allocator>, bytes: *const u8, length: cf::Index) -> Option<cf::Retained<'a, cf::Data>> {
    unsafe {
      CFDataCreate(allocator, bytes, length)
    }
  }

  #[inline]
  pub fn length(&self) -> cf::Index {
    unsafe { CFDataGetLength(self) }
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.length() as _
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }
}

/// ```
/// use cidre::cf;
/// let data = cf::Retained::from(&[1u8][..]);
/// assert_eq!(data.len(), 1);
/// data.show();
/// ```
impl<'a> From<&[u8]> for cf::Retained<'a, Data> {
    fn from(bytes: &[u8]) -> Self {
        unsafe {Data::create(None, bytes.as_ptr(), bytes.len() as _).unwrap_unchecked() }
    }
}

extern "C" {
  fn CFDataGetTypeID() -> cf::TypeId;
  fn CFDataCreate<'a>(allocator: Option<&cf::Allocator>, bytes: *const u8, length: cf::Index) -> Option<cf::Retained<'a, cf::Data>>;
  fn CFDataGetLength(data: &Data) -> cf::Index;
}