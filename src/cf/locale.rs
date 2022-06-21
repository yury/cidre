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
    pub fn system<'get>() -> &'get Locale {
        unsafe { CFLocaleGetSystem() }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let loc = cf::Locale::current();
    ///
    /// let id = loc.get_identifier();
    /// ```
    pub fn current<'copy>() -> cf::Retained<'copy, Locale> {
        unsafe { CFLocaleCopyCurrent() }
    }

    pub fn get_identifier(&self) -> &Identifier {
        unsafe { CFLocaleGetIdentifier(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]    
extern "C" {
    fn CFLocaleGetSystem<'get>() -> &'get Locale;
    fn CFLocaleGetIdentifier(locale: &Locale) -> &Identifier;
    fn CFLocaleCopyCurrent<'copy>() -> cf::Retained<'copy, Locale>;

}
