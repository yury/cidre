use crate::{api, arc, cf, define_obj_type, ns};

define_obj_type!(
    /// A network error object with a domain and error code.
    #[doc(alias = "nw_error")]
    #[doc(alias = "nw_error_t")]
    pub Error(ns::Id)
);

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

#[doc(alias = "nw_error_domain_t")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
#[repr(i32)]
pub enum ErrorDomain {
    #[doc(alias = "nw_error_domain_invalid")]
    Invalid = 0,

    /// The error code will be a POSIX error as defined in sys::ErrNo
    #[doc(alias = "nw_error_domain_posix")]
    Posix = 1,

    /// The error code will be a DNSServiceErrorType error as defined in dnssd
    #[doc(alias = "nw_error_domain_dns")]
    Dns = 2,

    /// The error code will be a TLS error as defined in Sec::Base
    #[doc(alias = "nw_error_domain_tls")]
    Tls = 3,

    /// The error code will be a Wi-Fi Aware error.
    #[doc(alias = "nw_error_domain_wifi_aware")]
    WifiAware = 4,
}

impl Error {
    #[doc(alias = "nw_error_get_error_domain")]
    #[inline]
    pub fn domain(&self) -> ErrorDomain {
        unsafe { nw_error_get_error_domain(self) }
    }

    #[doc(alias = "nw_error_get_error_code")]
    #[inline]
    pub fn code(&self) -> i32 {
        unsafe { nw_error_get_error_code(self) }
    }

    #[doc(alias = "nw_error_copy_cf_error")]
    #[inline]
    pub fn cf_error(&self) -> arc::Retained<cf::Error> {
        unsafe { nw_error_copy_cf_error(self) }
    }
}

impl cf::ErrorDomain {
    #[doc(alias = "kNWErrorDomainPOSIX")]
    #[inline]
    pub fn nw_posix() -> &'static Self {
        unsafe { kNWErrorDomainPOSIX }
    }

    #[doc(alias = "kNWErrorDomainDNS")]
    #[inline]
    pub fn nw_dns() -> &'static Self {
        unsafe { kNWErrorDomainDNS }
    }

    #[doc(alias = "kNWErrorDomainTLS")]
    #[inline]
    pub fn nw_tls() -> &'static Self {
        unsafe { kNWErrorDomainTLS }
    }

    #[doc(alias = "kNWErrorDomainWiFiAware")]
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
    #[inline]
    pub fn nw_wifi_aware() -> &'static Self {
        unsafe { kNWErrorDomainWiFiAware }
    }
}

#[link(name = "Network", kind = "framework")]
#[api::weak]
unsafe extern "C-unwind" {
    fn nw_error_get_error_domain(error: &Error) -> ErrorDomain;
    fn nw_error_get_error_code(error: &Error) -> i32;
    fn nw_error_copy_cf_error(error: &Error) -> arc::R<cf::Error>;

    static kNWErrorDomainPOSIX: &'static cf::ErrorDomain;
    static kNWErrorDomainDNS: &'static cf::ErrorDomain;
    static kNWErrorDomainTLS: &'static cf::ErrorDomain;

    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
    static kNWErrorDomainWiFiAware: &'static cf::ErrorDomain;
}
