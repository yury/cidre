use crate::{arc, cf, define_cf_type};

pub type Id = cf::String;

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
    /// let id = system_loc.id();
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
    /// let id = loc.id();
    /// ```
    pub fn current() -> arc::R<Locale> {
        unsafe { CFLocaleCopyCurrent() }
    }

    pub fn id(&self) -> &Id {
        unsafe { CFLocaleGetIdentifier(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFLocaleGetSystem() -> &'static Locale;
    fn CFLocaleGetIdentifier(locale: &Locale) -> &Id;
    fn CFLocaleCopyCurrent() -> arc::R<Locale>;
    fn CFLocaleGetTypeID() -> cf::TypeId;
}
