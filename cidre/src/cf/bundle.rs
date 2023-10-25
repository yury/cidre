use crate::{arc, cf, define_cf_type};

define_cf_type!(Bundle(cf::Type));

impl Bundle {
    /// ```
    /// use cidre::cf;
    ///
    /// let bundle = cf::Bundle::main().unwrap();
    ///
    /// let url = bundle.bundle_url().unwrap();
    /// ```
    pub fn bundle_url(&self) -> Option<arc::R<cf::Url>> {
        unsafe { CFBundleCopyBundleURL(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let bundle = cf::Bundle::main().unwrap();
    /// ```
    pub fn main() -> Option<&'static Bundle> {
        unsafe { CFBundleGetMainBundle() }
    }

    pub fn get_value_for_info_dictionary<'a>(&'a self, key: &cf::String) -> Option<&'a cf::Type> {
        unsafe { CFBundleGetValueForInfoDictionaryKey(self, key) }
    }

    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFBundleGetTypeID() }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFBundleGetTypeID() -> cf::TypeId;
    fn CFBundleGetMainBundle() -> Option<&'static Bundle>;
    fn CFBundleCopyBundleURL(bundle: &Bundle) -> Option<arc::R<cf::Url>>;

    fn CFBundleGetValueForInfoDictionaryKey<'a>(
        bundle: &'a Bundle,
        key: &cf::String,
    ) -> Option<&'a cf::Type>;
}
