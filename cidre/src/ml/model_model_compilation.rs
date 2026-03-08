use crate::{arc, ml, ns, objc};

impl ml::Model {
    #[cfg(not(target_os = "watchos"))]
    #[objc::msg_send(compileModelAtURL:error:)]
    pub unsafe fn compile_model_at_url_err<'ear>(
        model_url: &ns::Url,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ns::Url>>;

    #[cfg(not(target_os = "watchos"))]
    #[objc::available(macos = 10.13, ios = 11.0, tvos = 11.0)]
    pub fn compile_model_at_url<'ear>(model_url: &ns::Url) -> ns::Result<'ear, arc::R<ns::Url>> {
        ns::if_none(|err| unsafe { Self::compile_model_at_url_err(model_url, err) })
    }
}
