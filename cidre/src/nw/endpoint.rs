use std::ffi::{c_char, CStr, CString};

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

/// Bonjour Endpoints
impl Endpoint {
    pub fn create_bonjour_service(
        name: &CStr,
        type_: &CStr,
        domain: &CStr,
    ) -> Option<arc::R<Self>> {
        unsafe {
            nw_endpoint_create_bonjour_service(name.as_ptr(), type_.as_ptr(), domain.as_ptr())
        }
    }

    #[doc(alias = "nw_endpoint_get_bonjour_service_name")]
    #[inline]
    pub fn bonjour_service_name(&self) -> Option<&CStr> {
        unsafe {
            let ptr = nw_endpoint_get_bonjour_service_name(self);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "nw_endpoint_get_bonjour_service_type")]
    #[inline]
    pub fn bonjour_service_type(&self) -> Option<&CStr> {
        unsafe {
            let ptr = nw_endpoint_get_bonjour_service_type(self);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "nw_endpoint_get_bonjour_service_domain")]
    #[inline]
    pub fn bonjour_service_domain(&self) -> Option<&CStr> {
        unsafe {
            let ptr = nw_endpoint_get_bonjour_service_domain(self);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C" {
    fn nw_endpoint_create_bonjour_service(
        name: *const c_char,
        type_: *const c_char,
        domain: *const c_char,
    ) -> Option<arc::R<Endpoint>>;

    fn nw_endpoint_get_bonjour_service_name(endpoint: &Endpoint) -> *const c_char;
    fn nw_endpoint_get_bonjour_service_type(endpoint: &Endpoint) -> *const c_char;
    fn nw_endpoint_get_bonjour_service_domain(endpoint: &Endpoint) -> *const c_char;
}

/// Url Endpoints
impl Endpoint {
    #[doc(alias = "nw_endpoint_create_url")]
    #[inline]
    pub fn create_url(url: &CStr) -> Option<arc::R<Self>> {
        unsafe { nw_endpoint_create_url(url.as_ptr()) }
    }

    #[doc(alias = "nw_endpoint_get_url")]
    #[inline]
    pub fn url(&self) -> Option<&CStr> {
        unsafe {
            let ptr = nw_endpoint_get_url(self);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr))
            }
        }
    }

    #[doc(alias = "nw_endpoint_copy_txt_record")]
    #[inline]
    pub fn txt_record(&self) -> Option<CString> {
        unsafe {
            let ptr = nw_endpoint_copy_txt_record(self);
            if ptr.is_null() {
                None
            } else {
                Some(CString::from_raw(ptr))
            }
        }
    }

    #[doc(alias = "nw_endpoint_get_signature")]
    #[inline]
    pub fn signature(&self) -> Option<&[u8]> {
        unsafe {
            let mut size = 0;
            let ptr = nw_endpoint_get_signature(self, &mut size);
            if ptr.is_null() {
                None
            } else {
                Some(&*std::ptr::slice_from_raw_parts(ptr, size))
            }
        }
    }
}

#[link(name = "Network", kind = "framework")]
extern "C" {
    fn nw_endpoint_create_url(url: *const c_char) -> Option<arc::R<Endpoint>>;
    fn nw_endpoint_get_url(endpoint: &Endpoint) -> *const c_char;
    fn nw_endpoint_copy_txt_record(endpoint: &Endpoint) -> *mut c_char;
    fn nw_endpoint_get_signature(
        endpoint: &Endpoint,
        out_signature_length: &mut usize,
    ) -> *const u8;
}

#[cfg(test)]
mod tests {
    use crate::nw;
    use std::ffi::CString;

    #[test]
    fn host() {
        let host = CString::new("localhost").unwrap();
        let port = CString::new("8000").unwrap();
        let endpoint = nw::Endpoint::create_host(&host, &port).unwrap();

        assert_eq!(endpoint.type_(), nw::EndpointType::Host);
        assert_eq!(endpoint.hostname().unwrap(), host.as_c_str());
        assert_eq!(endpoint.port(), 8000);
        assert_eq!(endpoint.port_c_string().unwrap(), port);
    }

    #[test]
    fn bonjour() {
        let name = CString::new("example").unwrap();
        let type_ = CString::new("_what._udp").unwrap();
        let domain = CString::new("local").unwrap();
        let endpoint = nw::Endpoint::create_bonjour_service(&name, &type_, &domain).unwrap();

        assert_eq!(endpoint.type_(), nw::EndpointType::BonjourService);
        assert_eq!(endpoint.hostname(), None);
        assert_eq!(endpoint.port(), 0);
        assert_eq!(endpoint.bonjour_service_name().unwrap(), name.as_c_str());
        assert_eq!(endpoint.bonjour_service_type().unwrap(), type_.as_c_str());
        assert_eq!(
            endpoint.bonjour_service_domain().unwrap(),
            domain.as_c_str()
        );
    }

    #[test]
    fn url() {
        let url = CString::new("https:://ya.ru").unwrap();
        let endpoint = nw::Endpoint::create_url(&url).unwrap();
        assert_eq!(endpoint.type_(), nw::EndpointType::Url);
        assert_eq!(endpoint.url().unwrap(), url.as_c_str());

        assert!(endpoint.txt_record().is_none());
        assert!(endpoint.signature().is_none());
    }
}
