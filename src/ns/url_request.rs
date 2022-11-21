use crate::{
    cf::{self, Retained},
    define_obj_type, ns,
};

define_obj_type!(URLRequest(ns::Id));
define_obj_type!(MutableURLRequest(URLRequest));

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
    /// ```
    /// use cidre::{cf, ns};
    /// let url = cf::URL::from_str("https://google.com").unwrap();
    /// let request = ns::URLRequest::with_url(&url);
    /// let request_url = request.url().unwrap();
    /// assert!(url.cf_string().equal(request_url.cf_string()));
    /// ```
    #[inline]
    pub fn with_url(url: &cf::URL) -> Retained<URLRequest> {
        unsafe { NSURLRequest_requestWithURL(url) }
    }

    #[inline]
    pub fn with_url_cache_policy_and_timeout(
        url: &cf::URL,
        cache_policy: CachePolicy,
        timeout_interval: cf::TimeInterval,
    ) -> Retained<URLRequest> {
        unsafe {
            NSURLRequest_requestWithURL_cachePolicy_timeoutInterval(
                url,
                cache_policy,
                timeout_interval,
            )
        }
    }

    #[inline]
    pub fn url(&self) -> Option<&cf::URL> {
        unsafe { NSURLRequest_rsel_URL(self) }
    }
}

/// enum is used to indicate whether the
/// user or developer specified the URL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
pub enum Attribution {
    ///  Indicates that the URL was specified
    /// by the developer. This is the default value for an ns::URLRequest when created.
    Developer = 0,

    /// Indicates that the URL was specified by
    /// the user.
    User = 1,
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSURLRequest_requestWithURL(url: &cf::URL) -> Retained<URLRequest>;
    fn NSURLRequest_requestWithURL_cachePolicy_timeoutInterval(
        url: &cf::URL,
        cache_policy: CachePolicy,
        timeout_interval: cf::TimeInterval,
    ) -> Retained<URLRequest>;

    fn NSURLRequest_rsel_URL(request: &URLRequest) -> Option<&cf::URL>;
}
