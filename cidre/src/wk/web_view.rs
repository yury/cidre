use crate::{arc, blocks, define_obj_type, ns, objc, wk};

#[cfg(target_os = "ios")]
define_obj_type!(pub WebView(crate::ui::View), WK_WEB_VIEW);

#[cfg(target_os = "macos")]
define_obj_type!(pub WebView(ns::View), WK_WEB_VIEW);

impl WebView {
    #[objc::msg_send(loadRequest:)]
    pub fn load_request_ar(&mut self, request: &ns::UrlRequest)
        -> Option<arc::Rar<wk::Navigation>>;

    #[objc::rar_retain]
    pub fn load_request(&mut self, request: &ns::UrlRequest) -> Option<arc::R<wk::Navigation>>;

    #[objc::msg_send(setNavigationDelegate:)]
    pub fn set_nav_delegate<D: wk::NavigationDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(title)]
    pub fn title(&self) -> &ns::String;

    #[objc::msg_send(isInspectable)]
    pub fn is_inpectable(&self) -> bool;

    #[objc::msg_send(setInspectable:)]
    pub fn set_inpectable(&self, val: bool);

    #[objc::msg_send(URL)]
    pub fn url(&self) -> Option<&ns::Url>;

    #[objc::msg_send(isLoading)]
    pub fn is_loading(&self) -> bool;

    #[objc::msg_send(estimatedProgress)]
    pub fn estimated_progress(&self) -> f64;

    #[objc::msg_send(stopLoading)]
    pub fn stop_loading(&mut self);

    #[objc::msg_send(evaluateJavaScript:completionHandler:)]
    fn _eval_js_ch(&mut self, js: &ns::String, block: *mut std::ffi::c_void);

    pub fn eval_js_ch<'a, F>(&mut self, js: &ns::String, block: &'static mut blocks::Block<F>)
    where
        F: FnOnce(Option<&'a ns::Id>, Option<&'a ns::Error>),
    {
        self._eval_js_ch(js, block.as_mut_ptr());
    }

    pub fn eval_js<F>(&mut self, js: &ns::String) {
        self._eval_js_ch(js, std::ptr::null_mut());
    }
}

#[link(name = "wk", kind = "static")]
extern "C" {
    static WK_WEB_VIEW: &'static objc::Class<WebView>;
}
