use crate::{cf, define_cf_type};

pub type Identifier = cf::String;

define_cf_type!(Locale(cf::Type));

impl Locale {

  /// ```
  /// use cidre::cf;
  /// 
  /// let system_loc = cf::Locale::system();
  /// 
  /// let id = system_loc.get_identifier();
  /// 
  /// 
  /// ```
  pub fn system<'a>() -> &'a Locale {
    unsafe {
      CFLocaleGetSystem()
    }
  }

  pub fn get_identifier(&self) -> &Identifier {
    unsafe {
      CFLocaleGetIdentifier(self)
    }
  }
}

extern "C" {

  fn CFLocaleGetSystem<'a>() -> &'a Locale;
  fn CFLocaleGetIdentifier(locale: &Locale) -> &Identifier;

}