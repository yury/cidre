use crate::{cf, define_cf_type};

use super::Retained;

define_cf_type!(Bundle(cf::Type));

impl Bundle {
    /// ```
    /// use cidre::cf;
    ///
    /// let bundle = cf::Bundle::get_main().unwrap();
    ///
    /// let url = bundle.copy_bundle_url().unwrap();
    /// ```
    pub fn copy_bundle_url<'a>(&self) -> Option<Retained<'a, cf::URL>> {
        unsafe { CFBundleCopyBundleURL(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let bundle = cf::Bundle::get_main().unwrap();
    /// ```
    pub fn get_main<'a>() -> Option<&'a Bundle> {
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

extern "C" {
    fn CFBundleGetTypeID() -> cf::TypeId;
    fn CFBundleGetMainBundle<'a>() -> Option<&'a Bundle>;
    fn CFBundleCopyBundleURL<'a>(bundle: &Bundle) -> Option<Retained<'a, cf::URL>>;

    fn CFBundleGetValueForInfoDictionaryKey<'a>(
        bundle: &'a Bundle,
        key: &cf::String,
    ) -> Option<&'a cf::Type>;
}
