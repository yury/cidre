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
    pub fn normalized_issuer_sequence(&self) -> Option<arc::R<cf::Data>> {
        unsafe { SecCertificateCopyNormalizedIssuerSequence(self) }
    }

    #[doc(alias = "SecCertificateCopyNormalizedSubjectSequence")]
    pub fn normalized_subject_sequence(&self) -> Option<arc::R<cf::Data>> {
        unsafe { SecCertificateCopyNormalizedSubjectSequence(self) }
    }

    #[doc(alias = "SecCertificateCopyKey")]
    pub fn key(&self) -> Option<arc::R<sec::Key>> {
        unsafe { SecCertificateCopyKey(self) }
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
}
