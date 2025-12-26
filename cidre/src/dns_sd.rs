use std::ffi::c_void;

use crate::{define_opts, os};

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
    pub fn process_result(&self) -> os::Result {
        unsafe { DNSServiceProcessResult(self).result() }
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

#[doc(alias = "DNSServiceErrorType")]
pub type ServiceErrorType = os::Status;

/// kDNSServiceErr codes
pub mod err {
    use crate::os::Error;

    #[doc(alias = "kDNSServiceErr_Unknown")]
    pub const UKNOWN: Error = Error::new_unchecked(-65537);

    #[doc(alias = "kDNSServiceErr_NoSuchName")]
    pub const NO_SUCH_NAME: Error = Error::new_unchecked(-65538);

    #[doc(alias = "kDNSServiceErr_NoMemory")]
    pub const NO_MEMORY: Error = Error::new_unchecked(-65539);

    #[doc(alias = "kDNSServiceErr_BadParam")]
    pub const BAD_PARAM: Error = Error::new_unchecked(-65540);

    #[doc(alias = "kDNSServiceErr_BadReference")]
    pub const BAD_REFERENCE: Error = Error::new_unchecked(-65541);

    #[doc(alias = "kDNSServiceErr_BadState")]
    pub const BAD_STATE: Error = Error::new_unchecked(-65542);

    #[doc(alias = "kDNSServiceErr_BadFlags")]
    pub const BAD_FLAGS: Error = Error::new_unchecked(-65543);

    #[doc(alias = "kDNSServiceErr_Unsupported")]
    pub const UNSUPPORTED: Error = Error::new_unchecked(-65544);

    #[doc(alias = "kDNSServiceErr_NotInitialized")]
    pub const NOT_INITIALIZED: Error = Error::new_unchecked(-65545);

    #[doc(alias = "kDNSServiceErr_AlreadyRegistered")]
    pub const ALREADY_REGISTERED: Error = Error::new_unchecked(-65547);

    #[doc(alias = "kDNSServiceErr_NameConflict")]
    pub const NAME_CONFLICT: Error = Error::new_unchecked(-65548);

    #[doc(alias = "kDNSServiceErr_Invalid")]
    pub const INVALID: Error = Error::new_unchecked(-65549);

    #[doc(alias = "kDNSServiceErr_Firewall")]
    pub const FIREWALL: Error = Error::new_unchecked(-65550);

    /// Client library incompatible with daemon
    #[doc(alias = "kDNSServiceErr_Incompatible")]
    pub const INCOMPATIBLE: Error = Error::new_unchecked(-65551);

    #[doc(alias = "kDNSServiceErr_BadInterfaceIndex")]
    pub const BAD_INTERFACE_INDEX: Error = Error::new_unchecked(-65552);

    #[doc(alias = "kDNSServiceErr_Refused")]
    pub const REFUSED: Error = Error::new_unchecked(-65553);

    #[doc(alias = "kDNSServiceErr_NoSuchRecord")]
    pub const NO_SUCH_RECORD: Error = Error::new_unchecked(-65554);

    #[doc(alias = "kDNSServiceErr_NoAuth")]
    pub const NO_AUTH: Error = Error::new_unchecked(-65555);

    #[doc(alias = "kDNSServiceErr_NoSuchKey")]
    pub const NO_SUCH_KEY: Error = Error::new_unchecked(-65556);

    #[doc(alias = "kDNSServiceErr_NATTraversal")]
    pub const NAT_TRAVERSAL: Error = Error::new_unchecked(-65557);

    #[doc(alias = "kDNSServiceErr_DoubleNAT")]
    pub const DOUBLE_NAT: Error = Error::new_unchecked(-65558);

    #[doc(alias = "kDNSServiceErr_BadTime")]
    pub const BAD_TIME: Error = Error::new_unchecked(-65559);

    #[doc(alias = "kDNSServiceErr_BadSig")]
    pub const BAD_SIG: Error = Error::new_unchecked(-65560);

    #[doc(alias = "kDNSServiceErr_BadKey")]
    pub const BAD_KEY: Error = Error::new_unchecked(-65561);

    #[doc(alias = "kDNSServiceErr_Transient")]
    pub const TRANSIENT: Error = Error::new_unchecked(-65562);

    /// Background daemon not running
    #[doc(alias = "kDNSServiceErr_ServiceNotRunning")]
    pub const SERVICE_NOT_RUNNING: Error = Error::new_unchecked(-65563);

    /// NAT doesn't support PCP, NAT-PMP or UPnP
    #[doc(alias = "kDNSServiceErr_NATPortMappingUnsupported")]
    pub const NAT_PORT_MAPPING_UNSUPPORTED: Error = Error::new_unchecked(-65564);

    /// NAT supports PCP, NAT-PMP or UPnP, but it's disabled by the administrator
    #[doc(alias = "kDNSServiceErr_NATPortMappingDisabled")]
    pub const NAT_PORT_MAPPING_DISABLED: Error = Error::new_unchecked(-65565);

    /// No router currently configured (probably no network connectivity)
    #[doc(alias = "kDNSServiceErr_NoRouter")]
    pub const NO_ROUTER: Error = Error::new_unchecked(-65566);

    #[doc(alias = "kDNSServiceErr_PollingMode")]
    pub const POLLING_MODE: Error = Error::new_unchecked(-65567);

    #[doc(alias = "kDNSServiceErr_Timeout")]
    pub const TIMEOUT: Error = Error::new_unchecked(-65568);

    /// Connection to daemon returned a SO_ISDEFUNCT error result
    #[doc(alias = "kDNSServiceErr_DefunctConnection")]
    pub const DEFUNCT_CONNECTION: Error = Error::new_unchecked(-65569);

    #[doc(alias = "kDNSServiceErr_PolicyDenied")]
    pub const POLICY_DENIED: Error = Error::new_unchecked(-65570);

    #[doc(alias = "kDNSServiceErr_NotPermitted")]
    pub const NOT_PERMITTED: Error = Error::new_unchecked(-65571);

    #[doc(alias = "kDNSServiceErr_StaleData")]
    pub const STALE_DATA: Error = Error::new_unchecked(-65572);
}

unsafe extern "C-unwind" {
    fn DNSServiceRefSockFD(service: &Service) -> Sock;
    fn DNSServiceProcessResult(service: &Service) -> ServiceErrorType;
    fn DNSServiceRefDeallocate(service: &Service);
}
