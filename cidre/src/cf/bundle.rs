use crate::{arc, cf, define_cf_type};

define_cf_type!(
    #[doc(alias = "CFBundle")]
    Bundle(cf::Type)
);

impl Bundle {
    #[doc(alias = "CFBundleGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFBundleGetTypeID() }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let bundle = cf::Bundle::main().unwrap();
    ///
    /// let url = bundle.bundle_url().unwrap();
    /// ```
    #[doc(alias = "CFBundleCopyBundleURL")]
    pub fn bundle_url(&self) -> Option<arc::R<cf::Url>> {
        unsafe { CFBundleCopyBundleURL(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let bundle = cf::Bundle::main().unwrap();
    /// ```
    #[doc(alias = "CFBundleGetMainBundle")]
    pub fn main() -> Option<&'static Bundle> {
        unsafe { CFBundleGetMainBundle() }
    }

    #[doc(alias = "CFBundleGetValueForInfoDictionaryKey")]
    pub fn value_for_info_dictionary<'a>(&'a self, key: &cf::String) -> Option<&'a cf::Type> {
        unsafe { CFBundleGetValueForInfoDictionaryKey(self, key) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C-unwind" {
    fn CFBundleGetTypeID() -> cf::TypeId;
    fn CFBundleGetMainBundle() -> Option<&'static Bundle>;
    fn CFBundleCopyBundleURL(bundle: &Bundle) -> Option<arc::R<cf::Url>>;

    fn CFBundleGetValueForInfoDictionaryKey<'a>(
        bundle: &'a Bundle,
        key: &cf::String,
    ) -> Option<&'a cf::Type>;
}
