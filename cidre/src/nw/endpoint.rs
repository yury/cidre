use std::ffi::{c_char, c_void, CStr, CString};

use crate::{arc, define_obj_type, ns};

define_obj_type!(
    #[doc(alias = "nw_endpoint")]
    #[doc(alias = "nw_endpoint_t")]
    pub Endpoint(ns::Id)
);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum EndpointType {
    /// An undefined endpoint type.
    #[doc(alias = "nw_endpoint_type_invalid")]
    Invalid = 0,

    /// An endpoint represented as an IP address and port.
    #[doc(alias = "nw_endpoint_type_address")]
    Addr = 1,

    /// An endpoint represented as a hostname and port.
    #[doc(alias = "nw_endpoint_type_host")]
    Host = 2,

    /// An endpoint represented as a Bonjour service.
    #[doc(alias = "nw_endpoint_type_bonjour_service")]
    BonjourService = 3,

    /// An endpoint represented as a URL, with host and port values inferred from the URL.
    #[doc(alias = "nw_endpoint_type_url")]
    Url = 4,
}

impl Endpoint {
    #[doc(alias = "nw_endpoint_get_type")]
    #[inline]
    pub fn type_(&self) -> EndpointType {
        unsafe { nw_endpoint_get_type(self) }
    }
}

/// Host Endpoints
impl Endpoint {
    #[doc(alias = "nw_endpoint_create_host")]
    #[inline]
    pub fn create_host(hostname: &CStr, port: &CStr) -> Option<arc::R<Self>> {
        unsafe { nw_endpoint_create_host(hostname.as_ptr(), port.as_ptr()) }
    }

    #[doc(alias = "nw_endpoint_get_hostname")]
    #[inline]
    pub fn hostname(&self) -> Option<&CStr> {
        unsafe {
            let ptr = nw_endpoint_get_hostname(self);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "nw_endpoint_copy_port_string")]
    #[inline]
    pub fn port_c_string(&self) -> Option<CString> {
        unsafe {
            let ptr = nw_endpoint_copy_port_string(self);
            if ptr.is_null() {
                None
            } else {
                Some(CString::from_raw(ptr))
            }
        }
    }

    #[doc(alias = "nw_endpoint_get_port")]
    #[inline]
    pub fn port(&self) -> u16 {
        unsafe { nw_endpoint_get_port(self) }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C" {
    fn nw_endpoint_get_type(endpoint: &Endpoint) -> EndpointType;
    fn nw_endpoint_create_host(
        hostname: *const c_char,
        port: *const c_char,
    ) -> Option<arc::R<Endpoint>>;
    fn nw_endpoint_get_hostname(endpoint: &Endpoint) -> *const c_char;
    fn nw_endpoint_copy_port_string(endpoint: &Endpoint) -> *mut c_char;
    fn nw_endpoint_get_port(endpoint: &Endpoint) -> u16;
}

/// Address Endpoints
impl Endpoint {}

#[link(name = "Network", kind = "framework")]
extern "C" {
    fn nw_endpoint_create_address(address: *const c_void);
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use crate::nw;
    #[test]
    fn basics() {
        let host = CString::new("localhost").unwrap();
        let port = CString::new("8000").unwrap();
        let endpoint = nw::Endpoint::create_host(&host, &port).unwrap();

        assert_eq!(endpoint.type_(), nw::EndpointType::Host);
        assert_eq!(endpoint.hostname().unwrap(), host.as_c_str());
        assert_eq!(endpoint.port(), 8000);
        assert_eq!(endpoint.port_c_string().unwrap(), port);
    }
}
