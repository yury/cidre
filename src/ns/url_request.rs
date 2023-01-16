use crate::{
    arc, define_obj_type, ns,
    objc::{msg_send, Obj},
};

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

    #[inline]
    pub fn url(&self) -> Option<&ns::URL> {
        unsafe { crate::objc::Obj::call0(self, crate::objc::msg_send::url) }
    }

    #[inline]
    pub fn cache_policy(&self) -> CachePolicy {
        unsafe { NSURLRequest_rsel_cachePolicy(self) }
    }

    #[inline]
    pub fn timeout_interval(&self) -> ns::TimeInterval {
        unsafe { NSURLRequest_rsel_timeoutInterval(self) }
    }

    #[inline]
    pub fn network_service_type(&self) -> NetworkServiceType {
        unsafe { NSURLRequest_rsel_networkServiceType(self) }
    }

    #[inline]
    pub fn allows_cellular_access(&self) -> bool {
        unsafe { NSURLRequest_rsel_allowsCellularAccess(self) }
    }

    #[inline]
    pub fn allows_expensive_network_access(&self) -> bool {
        unsafe { NSURLRequest_rsel_allowsExpensiveNetworkAccess(self) }
    }

    #[inline]
    pub fn allows_constrained_network_access(&self) -> bool {
        unsafe { NSURLRequest_rsel_allowsConstrainedNetworkAccess(self) }
    }

    pub fn assumes_http3_capable(&self) -> bool {
        unsafe { NSURLRequest_rsel_assumesHTTP3Capable(self) }
    }

    pub fn attribution(&self) -> Attribution {
        unsafe { NSURLRequest_rsel_attribution(self) }
    }

    pub fn requires_dns_sec_validation(&self) -> bool {
        unsafe { NSURLRequest_rsel_requiresDNSSECValidation(self) }
    }

    #[inline]
    pub fn http_method(&self) -> Option<&ns::String> {
        unsafe { NSURLRequest_rsel_HTTPMethod(self) }
    }

    #[inline]
    pub fn all_http_header_fields(&self) -> Option<&ns::Dictionary<ns::String, ns::String>> {
        unsafe { NSURLRequest_rsel_allHTTPHeaderFields(self) }
    }

    #[inline]
    pub fn value_for_http_header_field<'a>(&'a self, field: &ns::String) -> Option<&'a ns::String> {
        unsafe { NSURLRequest_rsel_valueForHTTPHeaderField(self, field) }
    }

    #[inline]
    pub fn http_body(&self) -> Option<&ns::Data> {
        unsafe { NSURLRequest_rsel_HTTPBody(self) }
    }

    #[inline]
    pub fn copy_mut(&self) -> arc::R<URLRequestMut> {
        unsafe { self.call0(msg_send::mutable_copy) }
    }
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

    fn NSURLRequest_rsel_cachePolicy(request: &URLRequest) -> CachePolicy;
    fn NSURLRequest_rsel_timeoutInterval(request: &URLRequest) -> ns::TimeInterval;
    fn NSURLRequest_rsel_networkServiceType(request: &URLRequest) -> NetworkServiceType;
    fn NSURLRequest_rsel_allowsCellularAccess(request: &URLRequest) -> bool;
    fn NSURLRequest_rsel_allowsExpensiveNetworkAccess(request: &URLRequest) -> bool;
    fn NSURLRequest_rsel_allowsConstrainedNetworkAccess(request: &URLRequest) -> bool;
    fn NSURLRequest_rsel_assumesHTTP3Capable(request: &URLRequest) -> bool;
    fn NSURLRequest_rsel_attribution(request: &URLRequest) -> Attribution;
    fn NSURLRequest_rsel_requiresDNSSECValidation(request: &URLRequest) -> bool;
    fn NSURLRequest_rsel_HTTPMethod(request: &URLRequest) -> Option<&ns::String>;
    fn NSURLRequest_rsel_allHTTPHeaderFields(
        request: &URLRequest,
    ) -> Option<&ns::Dictionary<ns::String, ns::String>>;
    fn NSURLRequest_rsel_valueForHTTPHeaderField<'a>(
        request: &'a URLRequest,
        field: &ns::String,
    ) -> Option<&'a ns::String>;
    fn NSURLRequest_rsel_HTTPBody(request: &URLRequest) -> Option<&ns::Data>;
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

    #[inline]
    pub fn set_url(&mut self, value: Option<&ns::URL>) {
        unsafe { self.call1(msg_send::set_url, value) }
    }

    #[inline]
    pub fn set_cache_policy(&mut self, value: CachePolicy) {
        unsafe { NSMutableURLRequest_wsel_setCachePolicy(self, value) }
    }

    #[inline]
    pub fn set_timeout_interval(&mut self, value: ns::TimeInterval) {
        unsafe { NSMutableURLRequest_wsel_setTimeoutInterval(self, value) }
    }

    #[inline]
    pub fn set_network_service_type(&mut self, value: NetworkServiceType) {
        unsafe { NSMutableURLRequest_wsel_setNetworkServiceType(self, value) }
    }

    #[inline]
    pub fn set_allows_cellular_access(&mut self, value: bool) {
        unsafe { NSMutableURLRequest_wsel_setAllowsCellularAccess(self, value) }
    }

    #[inline]
    pub fn set_allows_expensive_network_access(&mut self, value: bool) {
        unsafe { NSMutableURLRequest_wsel_setAllowsExpensiveNetworkAccess(self, value) }
    }

    #[inline]
    pub fn set_allows_constrained_network_access(&mut self, value: bool) {
        unsafe { NSMutableURLRequest_wsel_setAllowsConstrainedNetworkAccess(self, value) }
    }

    #[inline]
    pub fn set_assumes_http3_capable(&mut self, value: bool) {
        unsafe { NSMutableURLRequest_wsel_setAssumesHTTP3Capable(self, value) }
    }

    #[inline]
    pub fn set_attribution(&mut self, value: Attribution) {
        unsafe { NSMutableURLRequest_wsel_setAttribution(self, value) }
    }

    #[inline]
    pub fn set_requires_dns_sec_validation(&mut self, value: bool) {
        unsafe { NSMutableURLRequest_wsel_setRequiresDNSSECValidation(self, value) }
    }

    #[inline]
    pub fn set_http_method(&mut self, value: Option<&ns::String>) {
        unsafe { NSMutableURLRequest_wsel_setHTTPMethod(self, value) }
    }

    #[inline]
    pub fn set_all_http_header_fields(
        &mut self,
        value: Option<&ns::Dictionary<ns::String, ns::String>>,
    ) {
        unsafe { NSMutableURLRequest_wsel_setAllHTTPHeaderFields(self, value) }
    }

    #[inline]
    pub fn set_http_body(&mut self, value: Option<&ns::Data>) {
        unsafe { NSMutableURLRequest_wsel_setHTTPBody(self, value) }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSMutableURLRequest_requestWithURL(url: &ns::URL) -> arc::R<URLRequestMut>;

    fn NSMutableURLRequest_requestWithURL_cachePolicy_timeoutInterval(
        url: &ns::URL,
        cache_policy: CachePolicy,
        timeout_interval: ns::TimeInterval,
    ) -> arc::R<URLRequestMut>;

    fn NSMutableURLRequest_wsel_setCachePolicy(request: &URLRequestMut, value: CachePolicy);

    fn NSMutableURLRequest_wsel_setTimeoutInterval(
        request: &URLRequestMut,
        value: ns::TimeInterval,
    );

    fn NSMutableURLRequest_wsel_setNetworkServiceType(
        request: &URLRequestMut,
        value: NetworkServiceType,
    );

    fn NSMutableURLRequest_wsel_setAllowsCellularAccess(request: &URLRequestMut, value: bool);

    fn NSMutableURLRequest_wsel_setAllowsExpensiveNetworkAccess(
        request: &URLRequestMut,
        value: bool,
    );

    fn NSMutableURLRequest_wsel_setAllowsConstrainedNetworkAccess(
        request: &URLRequestMut,
        value: bool,
    );

    fn NSMutableURLRequest_wsel_setAssumesHTTP3Capable(request: &URLRequestMut, value: bool);

    fn NSMutableURLRequest_wsel_setAttribution(request: &URLRequestMut, value: Attribution);

    fn NSMutableURLRequest_wsel_setRequiresDNSSECValidation(request: &URLRequestMut, value: bool);

    fn NSMutableURLRequest_wsel_setHTTPMethod(request: &URLRequestMut, value: Option<&ns::String>);

    fn NSMutableURLRequest_wsel_setAllHTTPHeaderFields(
        request: &URLRequestMut,
        value: Option<&ns::Dictionary<ns::String, ns::String>>,
    );

    fn NSMutableURLRequest_wsel_setHTTPBody(request: &URLRequestMut, value: Option<&ns::Data>);
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
