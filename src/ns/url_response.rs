use crate::{
    cf::{self, Retained},
    define_obj_type, msg_send, ns,
};

define_obj_type!(URLResponse(ns::Id));
define_obj_type!(HTTPURLResponse(URLResponse));

impl URLResponse {
    /// ```
    /// use cidre::{cf, ns};
    /// let url = cf::URL::from_str("https://google.com").unwrap();
    /// let response = ns::URLResponse::with_url(&url);
    /// let response_url = response.url().unwrap();
    /// assert!(url.equal(response_url));
    /// ```
    #[inline]
    pub fn with_url(url: &cf::URL) -> Retained<Self> {
        Self::with_url_mime_type(url, None, 0, None)
    }

    #[inline]
    pub fn with_url_mime_type(
        url: &cf::URL,
        mime_type: Option<&cf::String>,
        expected_content_length: ns::Integer,
        text_encoding_name: Option<&cf::String>,
    ) -> Retained<Self> {
        unsafe {
            NSURLResponse_initWithURL_MIMEType_expectedContentLength_textEncodingName(
                url,
                mime_type,
                expected_content_length,
                text_encoding_name,
            )
        }
    }

    pub fn url(&self) -> Option<&cf::URL> {
        msg_send!("common", self, sel_URL)
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSURLResponse_initWithURL_MIMEType_expectedContentLength_textEncodingName(
        url: &cf::URL,
        mime_type: Option<&cf::String>,
        expectedContentLength: ns::Integer,
        textEncodingName: Option<&cf::String>,
    ) -> Retained<URLResponse>;
}
