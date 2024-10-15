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

    #[doc(alias = "CFBundleCopyResourceURLsOfType")]
    pub fn res_urls(
        &self,
        resource_type: Option<&cf::String>,
        subdir: Option<&cf::String>,
    ) -> Option<arc::R<cf::ArrayOf<cf::Url>>> {
        unsafe { CFBundleCopyResourceURLsOfType(self, resource_type, subdir) }
    }

    #[doc(alias = "CFBundleCopyResourceURL")]
    pub fn res_url(&self, resource_name: &cf::String) -> Option<arc::R<cf::Url>> {
        self.res_url_with_type_subdir(resource_name, None, None)
    }

    #[doc(alias = "CFBundleCopyResourceURL")]
    pub fn res_url_in_subdir(
        &self,
        resource_name: &cf::String,
        subdir: &cf::String,
    ) -> Option<arc::R<cf::Url>> {
        self.res_url_with_type_subdir(resource_name, None, Some(subdir))
    }

    #[doc(alias = "CFBundleCopyResourceURL")]
    pub fn res_url_with_type_subdir(
        &self,
        resource_name: &cf::String,
        resource_type: Option<&cf::String>,
        subdir: Option<&cf::String>,
    ) -> Option<arc::R<cf::Url>> {
        unsafe { CFBundleCopyResourceURL(self, resource_name, resource_type, subdir) }
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
    fn CFBundleCopyResourceURL(
        bundle: &Bundle,
        resource_name: &cf::String,
        resource_type: Option<&cf::String>,
        subdir_name: Option<&cf::String>,
    ) -> Option<arc::R<cf::Url>>;

    fn CFBundleCopyResourceURLsOfType(
        bundle: &Bundle,
        resource_type: Option<&cf::String>,
        subdir_name: Option<&cf::String>,
    ) -> Option<arc::R<cf::ArrayOf<cf::Url>>>;

    fn CFBundleGetValueForInfoDictionaryKey<'a>(
        bundle: &'a Bundle,
        key: &cf::String,
    ) -> Option<&'a cf::Type>;
}
