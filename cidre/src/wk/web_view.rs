use crate::{arc, blocks, cg, define_obj_type, ns, objc, wk};

#[cfg(target_os = "ios")]
define_obj_type!(pub WebView(crate::ui::View), WK_WEB_VIEW);

#[cfg(target_os = "macos")]
define_obj_type!(pub WebView(ns::View), WK_WEB_VIEW);

impl arc::A<WebView> {
    #[objc::msg_send(initWithFrame:configuration:)]
    pub fn init_with_frame_cfg(self, frame: cg::Rect, cfg: &wk::WebViewCfg) -> arc::R<WebView>;
}

impl WebView {
    pub fn with_frame_cfg(frame: cg::Rect, cfg: &wk::WebViewCfg) -> arc::R<Self> {
        Self::alloc().init_with_frame_cfg(frame, cfg)
    }

    #[objc::msg_send(loadRequest:)]
    pub fn load_request(&mut self, request: &ns::UrlRequest) -> Option<arc::R<wk::Navigation>>;

    #[objc::msg_send(setNavigationDelegate:)]
    pub fn set_nav_delegate<D: wk::NavigationDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(isInspectable)]
    pub fn is_inpectable(&self) -> bool;

    #[objc::msg_send(setInspectable:)]
    pub fn set_inpectable(&self, val: bool);

    #[objc::msg_send(URL)]
    pub fn url(&self) -> Option<arc::R<ns::Url>>;

    #[objc::msg_send(isLoading)]
    pub fn is_loading(&self) -> bool;

    #[objc::msg_send(estimatedProgress)]
    pub fn estimated_progress(&self) -> f64;

    #[objc::msg_send(stopLoading)]
    pub fn stop_loading(&mut self);

    #[objc::msg_send(evaluateJavaScript:completionHandler:)]
    fn eval_js_ch_block(&mut self, js: &ns::String, block: Option<&mut blocks::ResultCh<ns::Id>>);

    #[inline]
    pub fn eval_js_ch(
        &mut self,
        js: &ns::String,
        block: impl FnMut(Option<&ns::Id>, Option<&ns::Error>) + 'static + std::marker::Send,
    ) {
        let mut block = blocks::ResultCh::new2(block);
        self.eval_js_ch_block(js, Some(&mut block));
    }

    pub fn eval_js_no_ch(&mut self, js: &ns::String) {
        self.eval_js_ch_block(js, None);
    }
}

#[link(name = "wk", kind = "static")]
unsafe extern "C" {
    static WK_WEB_VIEW: &'static objc::Class<WebView>;
}
