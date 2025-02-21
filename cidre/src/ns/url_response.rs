use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(pub UrlResponse(ns::Id));
define_obj_type!(pub HttpUrlResponse(UrlResponse));

impl arc::A<UrlResponse> {
    #[objc::msg_send(initWithURL:MIMEType:expectedContentLength:textEncodingName:)]
    fn init_with_url_mimt_type(
        self,
        url: &ns::Url,
        mime_type: Option<&ns::String>,
        expected_content_length: ns::Integer,
        text_encoding_name: Option<&ns::String>,
    ) -> arc::R<UrlResponse>;
}

impl UrlResponse {
    define_cls!(NS_URL_RESPONSE);
    /// ```
    /// use cidre::ns;
    /// let url = ns::Url::with_str("https://google.com").unwrap();
    /// let response = ns::UrlResponse::with_url(&url);
    /// let response_url = response.url().unwrap();
    /// assert!(url.is_equal(&response_url));
    /// ```
    #[inline]
    pub fn with_url(url: &ns::Url) -> arc::R<Self> {
        Self::alloc().init_with_url_mimt_type(url, None, 0, None)
    }

    #[inline]
    pub fn with_url_mime_type(
        url: &ns::Url,
        mime_type: Option<&ns::String>,
        expected_content_length: ns::Integer,
        text_encoding_name: Option<&ns::String>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_url_mimt_type(
            url,
            mime_type,
            expected_content_length,
            text_encoding_name,
        )
    }

    #[objc::msg_send(URL)]
    pub fn url(&self) -> Option<arc::R<ns::Url>>;
}

#[link(name = "ns", kind = "static")]
unsafe extern "C" {
    static NS_URL_RESPONSE: &'static objc::Class<UrlResponse>;
}

#[cfg(test)]
mod test {
    use crate::ns;

    #[test]
    fn basics() {
        let url = ns::Url::with_str("https://google.com").unwrap();
        let response = ns::UrlResponse::with_url(&url);
        let response_url = response.url().unwrap();
        assert!(url.is_equal(&response_url));
    }
}
