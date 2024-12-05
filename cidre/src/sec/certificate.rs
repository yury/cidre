use crate::{arc, cf, os, sec};

impl sec::Cert {
    #[doc(alias = "SecCertificateGetTypeID")]
    pub fn get_type_id() -> cf::TypeId {
        unsafe { SecCertificateGetTypeID() }
    }

    #[doc(alias = "SecCertificateCreateWithData")]
    pub fn with_data_in(
        data: &cf::Data,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe { SecCertificateCreateWithData(allocator, data) }
    }

    #[doc(alias = "SecCertificateCreateWithData")]
    pub fn with_data(data: &cf::Data) -> Option<arc::R<Self>> {
        unsafe { SecCertificateCreateWithData(None, data) }
    }

    /// the DER representation of an X.509 certificate.
    #[doc(alias = "SecCertificateCopyData")]
    pub fn data(&self) -> arc::R<cf::Data> {
        unsafe { SecCertificateCopyData(self) }
    }

    /// Return a simple string which hopefully represents a human
    /// understandable summary.
    #[doc(alias = "SecCertificateCopySubjectSummary")]
    pub fn subject_summary(&self) -> Option<arc::R<cf::String>> {
        unsafe { SecCertificateCopySubjectSummary(self) }
    }

    #[doc(alias = "SecCertificateCopyCommonName")]
    pub fn common_name(&self) -> os::Result<arc::R<cf::String>> {
        let mut res = None;
        unsafe { SecCertificateCopyCommonName(self, &mut res).to_result_unchecked(res) }
    }

    #[doc(alias = "SecCertificateCopyEmailAddresses")]
    pub fn emails(&self) -> os::Result<arc::R<cf::ArrayOf<cf::String>>> {
        let mut res = None;
        unsafe { SecCertificateCopyEmailAddresses(self, &mut res).to_result_unchecked(res) }
    }

    #[doc(alias = "SecCertificateCopyNormalizedIssuerSequence")]
    #[inline]
    pub fn normalized_issuer_sequence(&self) -> Option<arc::R<cf::Data>> {
        unsafe { SecCertificateCopyNormalizedIssuerSequence(self) }
    }

    #[doc(alias = "SecCertificateCopyNormalizedSubjectSequence")]
    #[inline]
    pub fn normalized_subject_sequence(&self) -> Option<arc::R<cf::Data>> {
        unsafe { SecCertificateCopyNormalizedSubjectSequence(self) }
    }

    #[doc(alias = "SecCertificateCopyKey")]
    #[inline]
    pub fn key(&self) -> Option<arc::R<sec::Key>> {
        unsafe { SecCertificateCopyKey(self) }
    }

    #[doc(alias = "SecCertificateCopyValues")]
    #[cfg(target_os = "macos")]
    pub fn all_values(
        &self,
    ) -> Result<
        arc::R<cf::DictionaryOf<cf::String, cf::DictionaryOf<cf::String, cf::Type>>>,
        arc::R<cf::Error>,
    > {
        cf::if_none(|err| unsafe { SecCertificateCopyValues(self, None, err) })
    }

    #[doc(alias = "SecCertificateCopyValues")]
    #[cfg(target_os = "macos")]
    pub fn values(
        &self,
        keys: &cf::ArrayOf<cf::String>,
    ) -> Result<
        arc::R<cf::DictionaryOf<cf::String, cf::DictionaryOf<cf::String, cf::Type>>>,
        arc::R<cf::Error>,
    > {
        cf::if_none(|err| unsafe { SecCertificateCopyValues(self, Some(keys), err) })
    }

    #[doc(alias = "SecCertificateCopyLongDescription")]
    #[cfg(target_os = "macos")]
    pub fn long_desc(&self) -> Result<arc::R<cf::String>, arc::R<cf::Error>> {
        cf::if_none(|err| unsafe { SecCertificateCopyLongDescription(None, self, err) })
    }

    #[doc(alias = "SecCertificateCopyShortDescription")]
    #[cfg(target_os = "macos")]
    pub fn short_desc(&self) -> Result<arc::R<cf::String>, arc::R<cf::Error>> {
        cf::if_none(|err| unsafe { SecCertificateCopyShortDescription(None, self, err) })
    }
}

#[link(name = "Security", kind = "framework")]
extern "C-unwind" {
    fn SecCertificateGetTypeID() -> cf::TypeId;

    fn SecCertificateCreateWithData(
        allocator: Option<&cf::Allocator>,
        data: &cf::Data,
    ) -> Option<arc::R<sec::Cert>>;

    fn SecCertificateCopyData(cert: &sec::Cert) -> arc::R<cf::Data>;

    fn SecCertificateCopySubjectSummary(cert: &sec::Cert) -> Option<arc::R<cf::String>>;

    fn SecCertificateCopyCommonName(
        cert: &sec::Cert,
        common_name: &mut Option<arc::R<cf::String>>,
    ) -> os::Status;

    fn SecCertificateCopyEmailAddresses(
        cert: &sec::Cert,
        email_addresses: &mut Option<arc::R<cf::ArrayOf<cf::String>>>,
    ) -> os::Status;

    fn SecCertificateCopyNormalizedIssuerSequence(cert: &sec::Cert) -> Option<arc::R<cf::Data>>;

    fn SecCertificateCopyNormalizedSubjectSequence(cert: &sec::Cert) -> Option<arc::R<cf::Data>>;

    fn SecCertificateCopyKey(cert: &sec::Cert) -> Option<arc::R<sec::Key>>;

    #[cfg(target_os = "macos")]
    fn SecCertificateCopyValues(
        crt: &sec::Cert,
        keys: Option<&cf::ArrayOf<cf::String>>,
        error: *mut Option<arc::R<cf::Error>>,
    ) -> Option<arc::R<cf::DictionaryOf<cf::String, cf::DictionaryOf<cf::String, cf::Type>>>>;

    #[cfg(target_os = "macos")]
    fn SecCertificateCopyLongDescription(
        alloc: Option<&cf::Allocator>,
        cert: &sec::Cert,
        error: *mut Option<arc::R<cf::Error>>,
    ) -> Option<arc::R<cf::String>>;

    #[cfg(target_os = "macos")]
    fn SecCertificateCopyShortDescription(
        alloc: Option<&cf::Allocator>,
        cert: &sec::Cert,
        error: *mut Option<arc::R<cf::Error>>,
    ) -> Option<arc::R<cf::String>>;
}

// https://github.com/aosm/Security/blob/master/Security/libsecurity_keychain/lib/CertificateValues.cpp
pub mod oids {
    use crate::cf;

    #[doc(alias = "kSecOIDX509V1SubjectName")]
    pub const fn x509_v1_subject_name() -> &'static cf::String {
        unsafe { kSecOIDX509V1SubjectName }
    }

    #[doc(alias = "kSecOIDX509V1IssuerName")]
    pub const fn x509_v1_issuer_name() -> &'static cf::String {
        unsafe { kSecOIDX509V1IssuerName }
    }

    // 2.5.4.10
    #[doc(alias = "kSecOIDOrganizationName")]
    pub const fn organization_name() -> &'static cf::String {
        unsafe { kSecOIDOrganizationName }
    }

    // 2.5.4.11
    #[doc(alias = "kSecOIDOrganizationalUnitName")]
    pub const fn organizational_unit_name() -> &'static cf::String {
        unsafe { kSecOIDOrganizationalUnitName }
    }

    #[link(name = "Security", kind = "framework")]
    extern "C" {
        static kSecOIDX509V1SubjectName: &'static cf::String;
        static kSecOIDX509V1IssuerName: &'static cf::String;

        static kSecOIDOrganizationName: &'static cf::String;
        static kSecOIDOrganizationalUnitName: &'static cf::String;
    }
}

#[cfg(target_os = "macos")]
pub mod prop_keys {
    use crate::cf;

    #[doc(alias = "kSecPropertyKeyType")]
    pub const fn type_() -> &'static cf::String {
        unsafe { kSecPropertyKeyType }
    }

    #[doc(alias = "kSecPropertyKeyLabel")]
    pub const fn label() -> &'static cf::String {
        unsafe { kSecPropertyKeyLabel }
    }

    #[doc(alias = "kSecPropertyKeyLocalizedLabel")]
    pub const fn localized_label() -> &'static cf::String {
        unsafe { kSecPropertyKeyLocalizedLabel }
    }

    #[doc(alias = "kSecPropertyKeyValue")]
    pub const fn value() -> &'static cf::String {
        unsafe { kSecPropertyKeyValue }
    }

    #[link(name = "Security", kind = "framework")]
    extern "C" {
        static kSecPropertyKeyType: &'static cf::String;
        static kSecPropertyKeyLabel: &'static cf::String;
        static kSecPropertyKeyLocalizedLabel: &'static cf::String;
        static kSecPropertyKeyValue: &'static cf::String;
    }
}

#[cfg(target_os = "macos")]
pub mod prop_types {
    use crate::cf;

    #[doc(alias = "kSecPropertyTypeWarning")]
    pub const fn warning() -> &'static cf::String {
        unsafe { kSecPropertyTypeWarning }
    }

    #[doc(alias = "kSecPropertyTypeSuccess")]
    pub const fn success() -> &'static cf::String {
        unsafe { kSecPropertyTypeSuccess }
    }

    #[doc(alias = "kSecPropertyTypeSection")]
    pub const fn section() -> &'static cf::String {
        unsafe { kSecPropertyTypeSection }
    }

    #[doc(alias = "kSecPropertyTypeData")]
    pub const fn data() -> &'static cf::String {
        unsafe { kSecPropertyTypeData }
    }

    #[doc(alias = "kSecPropertyTypeString")]
    pub const fn string() -> &'static cf::String {
        unsafe { kSecPropertyTypeString }
    }

    #[doc(alias = "kSecPropertyTypeURL")]
    pub const fn url() -> &'static cf::String {
        unsafe { kSecPropertyTypeURL }
    }

    #[doc(alias = "kSecPropertyTypeDate")]
    pub const fn date() -> &'static cf::String {
        unsafe { kSecPropertyTypeDate }
    }

    #[doc(alias = "kSecPropertyTypeArray")]
    pub const fn array() -> &'static cf::String {
        unsafe { kSecPropertyTypeArray }
    }

    #[doc(alias = "kSecPropertyTypeNumber")]
    pub const fn number() -> &'static cf::String {
        unsafe { kSecPropertyTypeNumber }
    }

    #[link(name = "Security", kind = "framework")]
    extern "C" {
        static kSecPropertyTypeWarning: &'static cf::String;
        static kSecPropertyTypeSuccess: &'static cf::String;
        static kSecPropertyTypeSection: &'static cf::String;
        static kSecPropertyTypeData: &'static cf::String;
        static kSecPropertyTypeString: &'static cf::String;
        static kSecPropertyTypeURL: &'static cf::String;
        static kSecPropertyTypeDate: &'static cf::String;
        static kSecPropertyTypeArray: &'static cf::String;
        static kSecPropertyTypeNumber: &'static cf::String;
    }
}
