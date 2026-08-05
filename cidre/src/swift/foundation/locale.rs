use crate::swift::{self, abi, value::define_swift_value};

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C" {
    #[link_name = "$s10Foundation6LocaleVMa"]
    fn locale_metadata();

    #[link_name = "$s10Foundation6LocaleV10identifierACSS_tcfC"]
    fn locale_init_with_identifier();

    #[link_name = "$s10Foundation6LocaleV10identifierSSvg"]
    fn locale_identifier();
}

define_swift_value!(
    /// `Foundation.Locale`.
    pub Locale, LocaleValue = accessor locale_metadata
);

unsafe impl Send for Locale {}
unsafe impl Sync for Locale {}

impl Locale {
    /// `Locale(identifier:)`.
    #[doc(alias = "Locale.init(identifier:)")]
    pub fn with_id(identifier: &str) -> Self {
        Self::with_swift_id(swift::String::from(identifier))
    }

    /// `Locale(identifier:)` from an already-built Swift string, which lets a
    /// caller reuse a `const` literal.
    #[doc(alias = "Locale.init(identifier:)")]
    pub fn with_swift_id(identifier: swift::String) -> Self {
        unsafe {
            let mut storage = Self::storage();
            abi::call_string_to_value(
                locale_init_with_identifier as *const (),
                identifier.into_raw(),
                storage.as_mut_ptr(),
            );
            Self::from_storage(storage)
        }
    }

    /// `Locale.identifier`.
    #[doc(alias = "Locale.identifier")]
    pub fn id(&self) -> swift::String {
        unsafe {
            swift::String::from_raw(abi::call_value_to_string(
                locale_identifier as *const (),
                self.as_ptr(),
            ))
        }
    }
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.id(), f)
    }
}

impl std::fmt::Debug for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Locale({})", self.id())
    }
}

impl From<&str> for Locale {
    fn from(identifier: &str) -> Self {
        Self::with_id(identifier)
    }
}
