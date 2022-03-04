use crate::{cf, define_cf_type};

use super::Retained;


#[repr(isize)]
pub enum PathStyle {
  Posix = 0,
  // HFS = 1,
  Windows = 2
}

define_cf_type!(URL(cf::Type));

impl URL {

  #[inline]
  pub fn create_with_bytes<'a>(allocator: Option<&cf::Allocator>, url_bytes: *const u8, length: cf::Index, encoding: cf::StringEncoding, base_url: Option<&URL>) -> Option<Retained<'a, URL>> {
    unsafe {
      CFURLCreateWithBytes(allocator, url_bytes, length, encoding, base_url)
    }
  }

  #[inline]
  pub fn create_with_string<'a>(allocator: Option<&cf::Allocator>, url_string: &cf::String, base_url: Option<&URL>) -> Option<Retained<'a, URL>> {
    unsafe {
      CFURLCreateWithString(allocator, url_string, base_url)
    }
  }

  /// ```
  /// use cidre::cf;
  /// 
  /// let url = cf::URL::from_str("http://google.com").unwrap();
  /// 
  /// let scheme = url.copy_scheme().unwrap();
  /// 
  /// ```
  #[inline]
  pub fn from_str<'a>(str: &str) -> Option<Retained<'a, URL>> {
    Self::create_with_bytes(None, str.as_ptr(), str.len() as _, cf::StringEncoding::UTF8, None)
  }

  /// ```
  /// use cidre::cf;
  /// 
  /// let s1 = cf::String::from_str("https://github.com");
  /// let url = cf::URL::from_string(&s1).unwrap();
  /// 
  /// 
  /// ```
  #[inline]
  pub fn from_string<'a>(str: &cf::String) -> Option<Retained<'a, URL>> {
    Self::create_with_string(None, str, None)
  }

  /// ```
  /// use cidre::cf;
  /// 
  /// let s1 = cf::String::from_str("https://github.com");
  /// let url = cf::URL::from_string(&s1).unwrap();
  /// 
  /// let s2 = url.get_string();
  /// 
  /// assert!(s1.equal(s2));
  /// 
  /// ```
  #[inline]
  pub fn get_string(&self) -> &cf::String {
    unsafe {
      CFURLGetString(self)
    }
  }

  #[inline]
  pub fn get_base_url(&self) -> Option<&URL> {
    unsafe {
      CFURLGetBaseURL(self)
    }
  }

  #[inline]
  pub fn copy_scheme<'a>(&self) -> Option<Retained<'a, cf::String>> {
    unsafe {
      CFURLCopyScheme(self)
    }
  }
}

extern "C" {
  fn CFURLCreateWithBytes<'a>(allocator: Option<&cf::Allocator>, url_bytes: *const u8, length: cf::Index, encoding: cf::StringEncoding, base_url: Option<&URL>) -> Option<Retained<'a, URL>>;
  fn CFURLCreateWithString<'a>(allocator: Option<&cf::Allocator>, url_string: &cf::String, base_url: Option<&URL>) -> Option<Retained<'a, URL>>;

  fn CFURLGetString(anURL: &URL) -> &cf::String;
  fn CFURLGetBaseURL(anURL: &URL) -> Option<&URL>;

  fn CFURLCopyScheme<'a>(anURL: &URL) -> Option<Retained<'a, cf::String>>;
}
