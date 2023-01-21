use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(URLResponse(ns::Id));
define_obj_type!(HTTPURLResponse(URLResponse));

impl arc::A<URLResponse> {
    #[objc::msg_send(initWithURL:MIMEType:expectedContentLength:textEncodingName:)]
    fn init_with_url_mimt_type(
        self,
        url: &ns::URL,
        mime_type: Option<&ns::String>,
        expected_content_length: ns::Integer,
        text_encoding_name: Option<&ns::String>,
    ) -> arc::R<URLResponse>;
}

impl URLResponse {
    define_cls!(NS_URL_RESPONSE);
    /// ```no_run
    /// use cidre::ns;
    /// let url = ns::URL::with_str("https://google.com").unwrap();
    /// let response = ns::URLResponse::with_url(&url);
    /// let response_url = response.url().unwrap();
    /// assert!(url.is_equal(&response_url));
    /// ```
    #[inline]
    pub fn with_url(url: &ns::URL) -> arc::R<Self> {
        Self::alloc().init_with_url_mimt_type(url, None, 0, None)
    }

    #[inline]
    pub fn with_url_mime_type(
        url: &ns::URL,
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
    pub fn url(&self) -> Option<&ns::URL>;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_URL_RESPONSE: &'static objc::Class<URLResponse>;
}

#[cfg(test)]
mod test {
    use crate::ns;

    #[test]
    fn basics() {
        let url = ns::URL::with_str("https://google.com").unwrap();
        let response = ns::URLResponse::with_url(&url);
        let response_url = response.url().unwrap();
        assert!(url.is_equal(&response_url));
    }
}
