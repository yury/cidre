use std::ffi::c_void;

use crate::define_opts;

pub type Sock = i32;

#[repr(transparent)]
pub struct Service(c_void);

#[repr(transparent)]
pub struct Record(c_void);

#[repr(transparent)]
pub struct ServiceAttribute(c_void);

impl Service {
    /// Maximum length, in bytes, of a service name represented as a
    /// literal C-String, including the terminating NULL at the end.
    pub const MAX_SERVICE_NAME: usize = 64;

    /// Maximum length, in bytes, of a domain name represented as an *escaped* C-String
    /// including the final trailing dot, and the C-String terminating NULL at the end.
    pub const MAX_DOMAIN_NAME: usize = 1009;

    /// Access underlying Unix domain socket for an initialized DNSServiceRef.
    #[doc(alias = "DNSServiceRefSockFD")]
    pub fn sock_fd(&self) -> Sock {
        unsafe { DNSServiceRefSockFD(self) }
    }

    #[doc(alias = "DNSServiceProcessResult")]
    pub fn process_result(&self) -> Result<(), ServiceErrorType> {
        let res = unsafe { DNSServiceProcessResult(self) };
        if res.0 == 0 {
            Ok(())
        } else {
            Err(res)
        }
    }

    #[doc(alias = "DNSServiceRefDeallocate")]
    pub fn deallocate(self) {
        unsafe { DNSServiceRefDeallocate(&self) }
    }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum ServiceAaaaPolicyName {
    #[doc(alias = "kDNSServiceAAAAPolicyNone")]
    None = 0,
    /// If AAAA record doesn't exist, query for A.
    #[doc(alias = "kDNSServiceAAAAPolicyFallback")]
    Fallback = 1,
}

define_opts!(pub ServiceFlags(u32));

define_opts!(pub ServiceProtocol(u32));

#[derive(Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct ServiceErrorType(i32);

extern "C-unwind" {
    fn DNSServiceRefSockFD(service: &Service) -> Sock;
    fn DNSServiceProcessResult(service: &Service) -> ServiceErrorType;
    fn DNSServiceRefDeallocate(service: &Service);
}
