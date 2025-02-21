use crate::{arc, cf, os};

#[doc(alias = "kSecClass")]
pub const fn class_key() -> &'static cf::String {
    unsafe { kSecClass }
}

pub mod class {
    use crate::cf;

    #[doc(alias = "kSecClassInternetPassword")]
    pub const fn internet_password() -> &'static cf::String {
        unsafe { kSecClassInternetPassword }
    }

    #[doc(alias = "kSecClassGenericPassword")]
    pub const fn generic_password() -> &'static cf::String {
        unsafe { kSecClassGenericPassword }
    }

    #[doc(alias = "kSecClassCertificate")]
    pub const fn certificate() -> &'static cf::String {
        unsafe { kSecClassCertificate }
    }

    #[doc(alias = "kSecClassKey")]
    pub const fn key() -> &'static cf::String {
        unsafe { kSecClassKey }
    }

    #[doc(alias = "kSecClassIdentity")]
    pub const fn identitiy() -> &'static cf::String {
        unsafe { kSecClassIdentity }
    }

    unsafe extern "C" {
        static kSecClassInternetPassword: &'static cf::String;
        static kSecClassGenericPassword: &'static cf::String;
        static kSecClassCertificate: &'static cf::String;
        static kSecClassKey: &'static cf::String;
        static kSecClassIdentity: &'static cf::String;
    }
}

#[doc(alias = "SecItemCopyMatching")]
pub fn matching(query: &cf::DictionaryOf<cf::String, cf::Type>) -> os::Result<arc::R<cf::Type>> {
    os::result_init(|res| unsafe { SecItemCopyMatching(query, res) })
}

#[link(name = "Security", kind = "framework")]
unsafe extern "C-unwind" {
    static kSecClass: &'static cf::String;

    fn SecItemCopyMatching(
        query: &cf::DictionaryOf<cf::String, cf::Type>,
        result: *mut arc::R<cf::Type>,
    ) -> os::Status;
}

/// Predefined search constants used to set values in a query
/// dictionary. You can specify a combination of search attributes and
/// item attributes when looking for matching items with the
/// SecItemCopyMatching function.
pub mod match_keys {
    use crate::{api, cf};

    /// Specifies a dictionary key whose value is a
    /// SecPolicyRef. If provided, returned certificates or identities must
    /// verify with this policy.
    #[doc(alias = "kSecMatchPolicy")]
    pub const fn policy() -> &'static cf::String {
        unsafe { kSecMatchPolicy }
    }

    /// Specifies a dictionary key whose value is a
    /// CFArray of SecKeychainItemRef items. If provided, returned items will be
    /// limited to the subset which are contained in this list.
    #[doc(alias = "kSecMatchItemList")]
    pub const fn item_list() -> &'static cf::String {
        unsafe { kSecMatchItemList }
    }

    /// Specifies a dictionary key whose value is a
    /// CFArray of X.500 names (of type CFDataRef). If provided, returned
    /// certificates or identities will be limited to those whose
    /// certificate chain contains one of the issuers provided in this list.
    #[doc(alias = "kSecMatchIssuers")]
    pub const fn issuers() -> &'static cf::String {
        unsafe { kSecMatchIssuers }
    }

    /// Specifies a dictionary key whose
    /// value is a &cf::String containing an RFC822 email address. If
    /// provided, returned certificates or identities will be limited to those
    /// that contain the address in their subject or subject alternative name.
    #[doc(alias = "kSecMatchEmailAddressIfPresent")]
    pub const fn email_address_if_present() -> &'static cf::String {
        unsafe { kSecMatchEmailAddressIfPresent }
    }

    /// Specifies a dictionary key whose value
    /// is a &cf::String. If provided, returned certificates or identities
    /// will be limited to those containing this string in the subject.
    #[doc(alias = "kSecMatchSubjectContains")]
    pub const fn subject_contains() -> &'static cf::String {
        unsafe { kSecMatchSubjectContains }
    }

    /// Specifies a dictionary key whose value
    /// is a &cf::String. If provided, returned internet passwords will be limited to those which
    /// have a server host that is equal to or a subdomain of this string. This filter only works on
    /// the Data Protection Keychain on macOS.
    #[doc(alias = "kSecMatchHostOrSubdomainOfHost")]
    #[api::available(macos = 15.0, ios = 18.0)]
    pub fn host_or_subdomain_of_host() -> &'static cf::String {
        unsafe { kSecMatchHostOrSubdomainOfHost }
    }

    /// macOS only. Specifies a dictionary key whose value
    /// is a &cf::String. If provided, returned certificates or identities
    /// will be limited to those with subject names that start with this string.
    #[doc(alias = "kSecMatchSubjectStartsWith")]
    #[api::available(macos = 10.7)]
    pub fn subject_starts_with() -> &'static cf::String {
        unsafe { kSecMatchSubjectStartsWith }
    }

    /// macOS only. Specifies a dictionary key whose value
    /// is a &cf::String. If provided, returned certificates or identities
    /// will be limited to those with subject names that end with this string.
    #[doc(alias = "kSecMatchSubjectEndsWith")]
    #[api::available(macos = 10.7)]
    pub fn subject_ends_with() -> &'static cf::String {
        unsafe { kSecMatchSubjectEndsWith }
    }

    /// macOS only. Specifies a dictionary key whose
    /// value is a &cf::String. If provided, returned certificates or identities
    /// will be limited to those matching this string exactly in the subject.
    #[doc(alias = "kSecMatchSubjectWholeString")]
    #[api::available(macos = 10.7)]
    pub fn subject_whole_string() -> &'static cf::String {
        unsafe { kSecMatchSubjectWholeString }
    }

    /// Specifies a dictionary key whose value
    /// is a CFBooleanRef. If this value is kCFBooleanFalse, or is not
    /// provided, then case-sensitive string matching is performed.
    #[doc(alias = "kSecMatchCaseInsensitive")]
    pub const fn case_insesitive() -> &'static cf::String {
        unsafe { kSecMatchCaseInsensitive }
    }

    /// macOS only. Specifies a dictionary key whose
    /// value is a CFBooleanRef. If this value is kCFBooleanFalse, or is not
    /// provided, then diacritic-sensitive string matching is performed.
    #[doc(alias = "kSecMatchDiacriticInsensitive")]
    #[api::available(macos = 10.7)]
    pub fn diacritic_case_insesitive() -> &'static cf::String {
        unsafe { kSecMatchDiacriticInsensitive }
    }

    /// macOS only. Specifies a dictionary key whose
    /// value is a CFBooleanRef. If this value is kCFBooleanFalse, or is not
    /// provided, then string matching is width-sensitive (e.g. 'a' != 0xFF41).
    #[doc(alias = "kSecMatchWidthInsensitive")]
    #[api::available(macos = 10.7)]
    pub fn width_insesitive() -> &'static cf::String {
        unsafe { kSecMatchWidthInsensitive }
    }

    /// Specifies a dictionary key whose value is
    /// a CFBooleanRef. If provided with a value of kCFBooleanTrue, only
    /// certificates which can be verified back to a trusted anchor will be
    /// returned. If this value is kCFBooleanFalse, or is not provided, then
    /// both trusted and untrusted certificates may be returned.
    #[doc(alias = "kSecMatchTrustedOnly")]
    pub const fn trusted_only() -> &'static cf::String {
        unsafe { kSecMatchTrustedOnly }
    }

    /// Specifies a dictionary key whose value is
    /// of type CFDateRef. If provided, returned keys, certificates or
    /// identities will be limited to those which are valid for the given date.
    /// Pass a value of kCFNull to indicate the current date.
    #[doc(alias = "kSecMatchValidOnDate")]
    pub const fn valid_on_date() -> &'static cf::String {
        unsafe { kSecMatchValidOnDate }
    }

    /// Specifies a dictionary key whose value is a
    /// CFNumberRef. If provided, this value specifies the maximum number of
    /// results to return. If not provided, results are limited to the first
    /// item found. Predefined values are provided for a single item
    /// (kSecMatchLimitOne) and all matching items (kSecMatchLimitAll).
    #[doc(alias = "kSecMatchLimit")]
    pub const fn limit() -> &'static cf::String {
        unsafe { kSecMatchLimit }
    }

    #[link(name = "Security", kind = "framework")]
    #[api::weak]
    unsafe extern "C" {
        static kSecMatchPolicy: &'static cf::String;
        static kSecMatchItemList: &'static cf::String;
        static kSecMatchIssuers: &'static cf::String;
        static kSecMatchEmailAddressIfPresent: &'static cf::String;
        static kSecMatchSubjectContains: &'static cf::String;
        #[api::available(macos = 15.0, ios = 18.0)]
        static kSecMatchHostOrSubdomainOfHost: &'static cf::String;
        #[api::available(macos = 10.7)]
        static kSecMatchSubjectStartsWith: &'static cf::String;
        #[api::available(macos = 10.7)]
        static kSecMatchSubjectEndsWith: &'static cf::String;
        #[api::available(macos = 10.7)]
        static kSecMatchSubjectWholeString: &'static cf::String;
        static kSecMatchCaseInsensitive: &'static cf::String;
        #[api::available(macos = 10.7)]
        static kSecMatchDiacriticInsensitive: &'static cf::String;
        #[api::available(macos = 10.7)]
        static kSecMatchWidthInsensitive: &'static cf::String;
        static kSecMatchTrustedOnly: &'static cf::String;
        static kSecMatchValidOnDate: &'static cf::String;
        static kSecMatchLimit: &'static cf::String;
    }
}

pub mod match_limit {
    use crate::cf;

    #[doc(alias = "kSecMatchLimitOne")]
    pub const fn one() -> &'static cf::String {
        unsafe { kSecMatchLimitOne }
    }

    #[doc(alias = "kSecMatchLimitAll")]
    pub const fn all() -> &'static cf::String {
        unsafe { kSecMatchLimitAll }
    }

    #[link(name = "Security", kind = "framework")]
    unsafe extern "C" {
        static kSecMatchLimitOne: &'static cf::String;
        static kSecMatchLimitAll: &'static cf::String;
    }
}

pub mod return_data {
    use crate::cf;

    /// Specifies a dictionary key whose value is of type
    /// CFBooleanRef. A value of kCFBooleanTrue indicates that the data of
    /// an item (CFDataRef) should be returned. For keys and password
    /// items, data is secret (encrypted) and may require the user to enter
    /// a password for access.
    #[doc(alias = "kSecReturnData")]
    pub const fn data() -> &'static cf::String {
        unsafe { kSecReturnData }
    }

    /// Specifies a dictionary key whose value is
    /// of type CFBooleanRef. A value of kCFBooleanTrue indicates that the
    /// (non-encrypted) attributes of an item (CFDictionaryRef) should be
    /// returned.
    #[doc(alias = "kSecReturnAttributes")]
    pub const fn attributes() -> &'static cf::String {
        unsafe { kSecReturnAttributes }
    }

    /// Specifies a dictionary key whose value is a
    /// CFBooleanRef. A value of kCFBooleanTrue indicates that a reference
    /// should be returned. Depending on the item class requested, the
    /// returned reference(s) may be of type SecKeychainItemRef, SecKeyRef,
    /// SecCertificateRef, or SecIdentityRef. Note that returning references is
    /// supported only for Certificate, Key or Identity items on iOS, watchOS and
    /// tvOS. Similarly, returning references is supported only for Certificate, Key
    /// or Identity items on macOS when either kSecUseDataProtectionKeychain
    /// is set to true or kSecAttrSynchronizable is set to true.
    #[doc(alias = "kSecReturnRef")]
    pub const fn cf_ref() -> &'static cf::String {
        unsafe { kSecReturnRef }
    }

    /// Specifies a dictionary key whose value
    /// is of type CFBooleanRef. A value of kCFBooleanTrue indicates that a
    /// persistent reference to an item (CFDataRef) should be returned.
    #[doc(alias = "kSecReturnPersistentRef")]
    pub const fn persistent_ref() -> &'static cf::String {
        unsafe { kSecReturnPersistentRef }
    }

    #[link(name = "Security", kind = "framework")]
    unsafe extern "C" {

        static kSecReturnData: &'static cf::String;
        static kSecReturnAttributes: &'static cf::String;
        static kSecReturnRef: &'static cf::String;
        static kSecReturnPersistentRef: &'static cf::String;
    }
}

#[cfg(target_os = "macos")]
#[cfg(test)]
mod tests {
    use crate::{arc, cf, sec};

    #[test]
    fn basics_certs() {
        let query = cf::DictionaryOf::with_keys_values(
            &[
                sec::class_key(),
                sec::match_keys::limit(),
                sec::match_keys::subject_starts_with(),
            ],
            &[
                sec::class::certificate().as_type_ref(),
                sec::match_limit::all(),
                cf::str!(c"Apple Development"),
            ],
        );
        let certs = sec::item_matching(&query).unwrap();
        // certs.show();
        assert_eq!(certs.get_type_id(), cf::Array::type_id());
        let certs: arc::R<cf::ArrayOf<sec::Cert>> = unsafe { std::mem::transmute(certs) };
        let keys = cf::ArrayOf::from_slice(&[
            sec::cert_oids::x509_v1_subject_name(),
            sec::cert_oids::x509_v1_issuer_name(),
        ]);
        for c in certs.iter() {
            // c.show();
            let desc = c.long_desc().unwrap();
            desc.show();
            let desc = c.short_desc().unwrap();
            desc.show();
            let vals = c.all_values().unwrap();
            assert!(!vals.is_empty());
            let vals = c.values(&keys).unwrap();
            assert!(!vals.is_empty());
        }
    }
}
