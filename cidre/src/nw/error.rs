use crate::{arc, cf, define_obj_type, ns};

define_obj_type!(
    /// A network error object with a domain and error code.
    #[doc(alias = "nw_error")]
    #[doc(alias = "nw_error_t")]
    pub Error(ns::Id)
);

#[doc(alias = "nw_error_domain_t")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
    Tls = 3,
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
    pub fn cf_error(&self) -> arc::R<cf::Error> {
        unsafe { nw_error_copy_cf_error(self) }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C" {
    fn nw_error_get_error_domain(error: &Error) -> ErrorDomain;
    fn nw_error_get_error_code(error: &Error) -> i32;
    fn nw_error_copy_cf_error(error: &Error) -> arc::R<cf::Error>;
}
