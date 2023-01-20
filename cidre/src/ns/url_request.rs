use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(URLRequest(ns::Id));
define_obj_type!(URLRequestMut(URLRequest));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum CachePolicy {
    UseProtocol = 0,
    ReloadIgnoringLocalCacheData = 1,
    ReturnCacheDataElseLoad = 2,
    ReturnCacheDataDontLoad = 3,
    ReloadIgnoringLocalAndRemoteCacheData = 4,
    ReloadRevalidatingCacheData = 5,
}

impl Default for CachePolicy {
    #[inline]
    fn default() -> Self {
        Self::UseProtocol
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum NetworkServiceType {
    // // Standard internet traffic
    Default = 0,

    /// Voice over IP control traffic
    VoIP = 1,

    /// Video traffic
    Video = 2,

    /// Background traffic
    Background = 3,

    /// Voice data
    Voice = 4,

    /// Responsive data
    ResponsiveData = 6,

    /// Multimedia Audio/Video Streaming
    AVStreaming = 8,

    /// Responsive Multimedia Audio/Video
    ResponsiveAV = 9,

    /// Call Signaling
    CallSignaling = 11,
}

impl URLRequest {
    /// ```no_run
    /// use cidre::ns;
    /// let url = ns::URL::with_str("https://google.com").unwrap();
    /// let request = ns::URLRequest::with_url(&url);
    /// let request_url = request.url().unwrap();
    /// assert!(url.abs_string().unwrap().eq(&request_url.abs_string().unwrap()));
    /// assert_eq!(request.cache_policy(), ns::URLRequestCachePolicy::UseProtocol);
    /// assert_eq!(request.timeout_interval(), 60f64);
    /// assert_eq!(request.network_service_type(), ns::URLRequestNetworkServiceType::Default);
    /// assert!(request.allows_cellular_access());
    /// assert!(request.allows_expensive_network_access());
    /// assert!(request.allows_constrained_network_access());
    /// assert!(!request.assumes_http3_capable());
    /// assert_eq!(request.attribution(), ns::URLRequestAttribution::Developer);
    /// assert!(!request.requires_dns_sec_validation());
    /// assert!(request.http_method().is_some());
    /// assert!(request.all_http_header_fields().is_none());
    /// assert!(request.http_body().is_none());
    /// ```
    #[inline]
    pub fn with_url(url: &ns::URL) -> arc::R<URLRequest> {
        unsafe { NSURLRequest_requestWithURL(url) }
    }

    #[inline]
    pub fn with_url_cache_policy_and_timeout(
        url: &ns::URL,
        cache_policy: CachePolicy,
        timeout_interval: ns::TimeInterval,
    ) -> arc::R<URLRequest> {
        unsafe {
            NSURLRequest_requestWithURL_cachePolicy_timeoutInterval(
                url,
                cache_policy,
                timeout_interval,
            )
        }
    }

    #[objc::msg_send(URL)]
    pub fn url(&self) -> Option<&ns::URL>;

    #[objc::msg_send(cachePolicy)]
    pub fn cache_policy(&self) -> CachePolicy;

    #[objc::msg_send(timeoutInterval)]
    pub fn timeout_interval(&self) -> ns::TimeInterval;

    #[objc::msg_send(networkServiceType)]
    pub fn network_service_type(&self) -> NetworkServiceType;

    #[objc::msg_send(allowsCellularAccess)]
    pub fn allows_cellular_access(&self) -> bool;

    #[objc::msg_send(allowsExpensiveNetworkAccess)]
    pub fn allows_expensive_network_access(&self) -> bool;

    #[objc::msg_send(allowsConstrainedNetworkAccess)]
    pub fn allows_constrained_network_access(&self) -> bool;

    #[objc::msg_send(assumesHTTP3Capable)]
    pub fn assumes_http3_capable(&self) -> bool;

    #[objc::msg_send(attribution)]
    pub fn attribution(&self) -> Attribution;

    #[objc::msg_send(requiresDNSSECValidation)]
    pub fn requires_dns_sec_validation(&self) -> bool;

    #[objc::msg_send(HTTPMethod)]
    pub fn http_method(&self) -> Option<&ns::String>;

    #[objc::msg_send(allHTTPHeaderFields)]
    pub fn all_http_header_fields(&self) -> Option<&ns::Dictionary<ns::String, ns::String>>;

    #[objc::msg_send(valueForHTTPHeaderField:)]
    pub fn value_for_http_header_field<'a>(&'a self, field: &ns::String) -> Option<&'a ns::String>;

    #[objc::msg_send(HTTPBody)]
    pub fn http_body(&self) -> Option<&ns::Data>;

    #[objc::msg_send(mutableCopy)]
    pub fn copy_mut_ar(&self) -> arc::Rar<URLRequestMut>;

    #[objc::rar_retain()]
    pub fn copy_mut(&self) -> arc::R<URLRequestMut>;
}

/// enum is used to indicate whether the
/// user or developer specified the URL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Attribution {
    /// Indicates that the URL was specified
    /// by the developer. This is the default value for an ns::URLRequest when created.
    Developer = 0,

    /// Indicates that the URL was specified by
    /// the user.
    User = 1,
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSURLRequest_requestWithURL(url: &ns::URL) -> arc::R<URLRequest>;
    fn NSURLRequest_requestWithURL_cachePolicy_timeoutInterval(
        url: &ns::URL,
        cache_policy: CachePolicy,
        timeout_interval: ns::TimeInterval,
    ) -> arc::R<URLRequest>;
}

impl URLRequestMut {
    #[inline]
    pub fn with_url(url: &ns::URL) -> arc::R<Self> {
        unsafe { NSMutableURLRequest_requestWithURL(url) }
    }

    #[inline]
    pub fn with_url_cache_policy_and_timeout(
        url: &ns::URL,
        cache_policy: CachePolicy,
        timeout_interval: ns::TimeInterval,
    ) -> arc::R<Self> {
        unsafe {
            NSMutableURLRequest_requestWithURL_cachePolicy_timeoutInterval(
                url,
                cache_policy,
                timeout_interval,
            )
        }
    }

    #[objc::msg_send(setURL:)]
    pub fn set_url(&mut self, value: Option<&ns::URL>);

    #[objc::msg_send(setCachePolicy:)]
    pub fn set_cache_policy(&mut self, value: CachePolicy);

    #[objc::msg_send(setTimeoutInterval:)]
    pub fn set_timeout_interval(&mut self, value: ns::TimeInterval);

    #[objc::msg_send(setNetworkServiceType:)]
    pub fn set_network_service_type(&mut self, value: NetworkServiceType);

    #[objc::msg_send(setAllowsCellularAccess:)]
    pub fn set_allows_cellular_access(&mut self, value: bool);

    #[objc::msg_send(setAllowsExpensiveNetworkAccess:)]
    pub fn set_allows_expensive_network_access(&mut self, value: bool);

    #[objc::msg_send(setAllowsConstrainedNetworkAccess:)]
    pub fn set_allows_constrained_network_access(&mut self, value: bool);

    #[objc::msg_send(setAssumesHTTP3Capable:)]
    pub fn set_assumes_http3_capable(&mut self, value: bool);

    #[objc::msg_send(setAttribution:)]
    pub fn set_attribution(&mut self, value: Attribution);

    #[objc::msg_send(setRequiresDNSSECValidation:)]
    pub fn set_requires_dns_sec_validation(&mut self, value: bool);

    #[objc::msg_send(setHTTPMethod:)]
    pub fn set_http_method(&mut self, value: Option<&ns::String>);

    #[objc::msg_send(setAllHTTPHeaderFields:)]
    pub fn set_all_http_header_fields(
        &mut self,
        value: Option<&ns::Dictionary<ns::String, ns::String>>,
    );

    #[objc::msg_send(setHTTPBody:)]
    pub fn set_http_body(&mut self, value: Option<&ns::Data>);
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSMutableURLRequest_requestWithURL(url: &ns::URL) -> arc::R<URLRequestMut>;

    fn NSMutableURLRequest_requestWithURL_cachePolicy_timeoutInterval(
        url: &ns::URL,
        cache_policy: CachePolicy,
        timeout_interval: ns::TimeInterval,
    ) -> arc::R<URLRequestMut>;

}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let mut request =
            ns::URLRequest::with_url(&ns::URL::with_str("https://google.com").unwrap()).copy_mut();
        request.set_timeout_interval(61f64);
        assert_eq!(request.timeout_interval(), 61f64);
        request.set_cache_policy(ns::URLRequestCachePolicy::ReloadRevalidatingCacheData);
        assert_eq!(
            request.cache_policy(),
            ns::URLRequestCachePolicy::ReloadRevalidatingCacheData
        );

        request.set_network_service_type(ns::URLRequestNetworkServiceType::AVStreaming);
        assert_eq!(
            request.network_service_type(),
            ns::URLRequestNetworkServiceType::AVStreaming
        );
    }

    #[test]
    fn basics1() {
        let url = ns::URL::with_str("https://google.com").unwrap();
        let request = ns::URLRequest::with_url(&url);
        let request_url = request.url().unwrap();
        assert!(url
            .abs_string()
            .unwrap()
            .eq(&request_url.abs_string().unwrap()));
        assert_eq!(
            request.cache_policy(),
            ns::URLRequestCachePolicy::UseProtocol
        );
        assert_eq!(request.timeout_interval(), 60f64);
        assert_eq!(
            request.network_service_type(),
            ns::URLRequestNetworkServiceType::Default
        );
        assert!(request.allows_cellular_access());
        assert!(request.allows_expensive_network_access());
        assert!(request.allows_constrained_network_access());
        assert!(!request.assumes_http3_capable());
        assert_eq!(request.attribution(), ns::URLRequestAttribution::Developer);
        assert!(!request.requires_dns_sec_validation());
        assert!(request.http_method().is_some());
        assert!(request.all_http_header_fields().is_none());
        assert!(request.http_body().is_none());
    }
}
