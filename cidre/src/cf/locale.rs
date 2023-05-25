use crate::{arc, cf, define_cf_type};

pub type Identifier = cf::String;

define_cf_type!(Locale(cf::Type));

impl Locale {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFLocaleGetTypeID() }
    }
    /// ```
    /// use cidre::cf;
    ///
    /// let system_loc = cf::Locale::system();
    ///
    /// let id = system_loc.get_identifier();
    ///
    ///
    /// ```
    pub fn system() -> &'static Locale {
        unsafe { CFLocaleGetSystem() }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let loc = cf::Locale::current();
    ///
    /// let id = loc.get_identifier();
    /// ```
    pub fn current() -> arc::R<Locale> {
        unsafe { CFLocaleCopyCurrent() }
    }

    pub fn get_identifier(&self) -> &Identifier {
        unsafe { CFLocaleGetIdentifier(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFLocaleGetSystem() -> &'static Locale;
    fn CFLocaleGetIdentifier(locale: &Locale) -> &Identifier;
    fn CFLocaleCopyCurrent() -> arc::R<Locale>;
    fn CFLocaleGetTypeID() -> cf::TypeId;
}
