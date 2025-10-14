use crate::{api, arc, cf, sec};

impl sec::Policy {
    #[doc(alias = "SecPolicyGetTypeID")]
    pub fn type_id() -> cf::TypeId {
        unsafe { SecPolicyGetTypeID() }
    }

    /// Basic X509-style certificate evaluation.
    #[doc(alias = "kSecPolicyAppleX509Basic")]
    pub fn apple_x509_basic() -> &'static cf::String {
        unsafe { kSecPolicyAppleX509Basic }
    }

    /// Basic X509 plus host name verification per RFC 2818.
    #[doc(alias = "kSecPolicyAppleSSL")]
    pub fn apple_ssl() -> &'static cf::String {
        unsafe { kSecPolicyAppleSSL }
    }

    /// Basic X509 plus email address verification and KeyUsage enforcement per RFC 2632.
    #[doc(alias = "kSecPolicyAppleSMIME")]
    pub fn apple_smime() -> &'static cf::String {
        unsafe { kSecPolicyAppleSMIME }
    }

    /// Extensible Authentication Protocol. Functionally identical to SSL policy.
    /// A separate OID is provided to facilitate per-policy, per-certificate trust settings using the SecTrust mechanism.
    #[doc(alias = "kSecPolicyAppleEAP")]
    pub fn apple_eap() -> &'static cf::String {
        unsafe { kSecPolicyAppleEAP }
    }

    /// Policy for use in IPsec communication. Functionally identical to SSL policy.
    /// A separate OID is provided to facilitate per-policy, per-certificate trust settings using the SecTrust mechanism.
    #[doc(alias = "kSecPolicyAppleIPsec")]
    pub fn apple_ip_sec() -> &'static cf::String {
        unsafe { kSecPolicyAppleIPsec }
    }

    /// Kerberos Pkinit client certificate validation.
    #[doc(alias = "kSecPolicyApplePKINITClient")]
    #[api::available(macos = 10.7)]
    pub fn apple_pkinit_client() -> &'static cf::String {
        unsafe { kSecPolicyApplePKINITClient }
    }

    /// Kerberos Pkinit server certificate validation.
    #[doc(alias = "kSecPolicyApplePKINITServer")]
    #[api::available(macos = 10.7)]
    pub fn apple_pkinit_server() -> &'static cf::String {
        unsafe { kSecPolicyApplePKINITServer }
    }

    /// Policy for use in evaluating Apple code signing certificates.
    #[doc(alias = "kSecPolicyAppleCodeSigning")]
    pub fn apple_code_signing() -> &'static cf::String {
        unsafe { kSecPolicyAppleCodeSigning }
    }

    /// Policy for use in evaluating Mac App Store receipts.
    #[doc(alias = "kSecPolicyMacAppStoreReceipt")]
    pub fn mac_app_store_receipt() -> &'static cf::String {
        unsafe { kSecPolicyMacAppStoreReceipt }
    }

    /// Policy for use in evaluating Apple ID certificates.
    #[doc(alias = "kSecPolicyAppleIDValidation")]
    pub fn apple_id_validation() -> &'static cf::String {
        unsafe { kSecPolicyAppleIDValidation }
    }

    /// Policy that causes evaluation of the validity of the time stamp on a signature.
    /// This can be used to allow verification that a certificate was valid at the time that something
    /// was signed with that certificate even if the certificate is no longer valid.
    #[doc(alias = "kSecPolicyAppleTimeStamping")]
    pub fn apple_time_stamping() -> &'static cf::String {
        unsafe { kSecPolicyAppleTimeStamping }
    }

    #[doc(alias = "kSecPolicyAppleRevocation")]
    pub fn apple_revocation() -> &'static cf::String {
        unsafe { kSecPolicyAppleRevocation }
    }

    #[doc(alias = "kSecPolicyApplePassbookSigning")]
    pub fn apple_passbook_signing() -> &'static cf::String {
        unsafe { kSecPolicyApplePassbookSigning }
    }

    #[doc(alias = "kSecPolicyApplePayIssuerEncryption")]
    pub fn apple_pay_issuer_encryption() -> &'static cf::String {
        unsafe { kSecPolicyApplePayIssuerEncryption }
    }

    #[doc(alias = "SecPolicyCreateRevocation")]
    pub fn revocation() -> Option<arc::R<Self>> {
        unsafe { SecPolicyCreateRevocation() }
    }

    /// Returns a policy object for evaluating SSL certificate chains.
    #[doc(alias = "SecPolicyCreateSSL")]
    pub fn ssl(server: bool, hostname: Option<&cf::String>) -> arc::R<Self> {
        unsafe { SecPolicyCreateSSL(server, hostname) }
    }

    /// Returns a policy object for the default X.509 policy.
    #[doc(alias = "SecPolicyCreateBasicX509")]
    pub fn basic_x509() -> arc::R<Self> {
        unsafe { SecPolicyCreateBasicX509() }
    }

    #[doc(alias = "SecPolicyCreateWithProperties")]
    pub fn with_props(
        policy_identifier: &cf::Type,
        props: Option<&cf::Dictionary>,
    ) -> Option<arc::R<Self>> {
        unsafe { SecPolicyCreateWithProperties(policy_identifier, props) }
    }
}

#[link(name = "Security", kind = "framework")]
#[api::weak]
unsafe extern "C-unwind" {
    fn SecPolicyGetTypeID() -> cf::TypeId;
    fn SecPolicyCreateWithProperties(
        policy_identifier: &cf::Type,
        props: Option<&cf::Dictionary>,
    ) -> Option<arc::R<sec::Policy>>;
    fn SecPolicyCreateRevocation() -> Option<arc::R<sec::Policy>>;
    fn SecPolicyCreateSSL(server: bool, hostname: Option<&cf::String>) -> arc::R<sec::Policy>;
    fn SecPolicyCreateBasicX509() -> arc::R<sec::Policy>;

    static kSecPolicyAppleX509Basic: &'static cf::String;
    static kSecPolicyAppleSSL: &'static cf::String;
    static kSecPolicyAppleSMIME: &'static cf::String;
    static kSecPolicyAppleEAP: &'static cf::String;
    static kSecPolicyAppleIPsec: &'static cf::String;

    #[api::available(macos = 10.7)]
    static kSecPolicyApplePKINITClient: &'static cf::String;
    #[api::available(macos = 10.7)]
    static kSecPolicyApplePKINITServer: &'static cf::String;

    static kSecPolicyAppleCodeSigning: &'static cf::String;
    static kSecPolicyMacAppStoreReceipt: &'static cf::String;
    static kSecPolicyAppleIDValidation: &'static cf::String;
    static kSecPolicyAppleTimeStamping: &'static cf::String;
    static kSecPolicyAppleRevocation: &'static cf::String;
    static kSecPolicyApplePassbookSigning: &'static cf::String;
    static kSecPolicyApplePayIssuerEncryption: &'static cf::String;
}

#[cfg(test)]
mod tests {
    use crate::sec;

    #[test]
    fn basics() {
        let policy = sec::Policy::with_props(sec::Policy::apple_code_signing(), None).unwrap();
        policy.show();
    }
}
