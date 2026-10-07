use crate::{arc, blocks, cg, define_obj_type, ns, objc, wk};

#[doc(alias = "WKMediaPlaybackState")]
#[repr(isize)]
pub enum MediaPlaybackState {
    None,
    Playing,
    Paused,
    Suspended,
}

#[doc(alias = "WKMediaCaptureState")]
#[repr(isize)]
pub enum MediaCaptureState {
    None,
    Active,
    Muted,
}

#[doc(alias = "WKFullscreenState")]
#[repr(isize)]
pub enum FullscreenState {
    NotInFullscreen,
    EnteringFullscreen,
    InFullscreen,
    ExitingFullscreen,
}

#[cfg(target_os = "ios")]
define_obj_type!(pub WebView(crate::ui::View), sym WKWebView);

#[cfg(target_os = "macos")]
define_obj_type!(pub WebView(ns::View), sym WKWebView);

impl WebView {
    #[objc::init(initWithFrame:configuration:)]
    pub fn init_with_frame_cfg(self, frame: cg::Rect, cfg: &wk::WebViewCfg) -> arc::R<WebView>;

    /// A copy of the configuration with which the web view was initialized
    #[objc::msg_send(configuration)]
    pub fn cfg(&self) -> arc::R<wk::WebViewCfg>;

    pub fn with_frame_cfg(frame: cg::Rect, cfg: &wk::WebViewCfg) -> arc::R<Self> {
        Self::alloc().init_with_frame_cfg(frame, cfg)
    }

    #[objc::msg_send(loadRequest:)]
    pub fn load_request(&mut self, request: &ns::UrlRequest) -> Option<arc::R<wk::Navigation>>;

    #[objc::msg_send(loadHTMLString:baseURL:)]
    pub fn load_html_string(
        &mut self,
        html: &ns::String,
        base_url: Option<&ns::Url>,
    ) -> Option<arc::R<wk::Navigation>>;

    #[objc::msg_send(reload)]
    pub fn reload(&mut self) -> Option<arc::R<wk::Navigation>>;

    /// Whether a swipe goes back and forward in the history.
    #[objc::msg_send(allowsBackForwardNavigationGestures)]
    pub fn allows_back_forward_navigation_gestures(&self) -> bool;

    #[objc::msg_send(setAllowsBackForwardNavigationGestures:)]
    pub fn set_allows_back_forward_navigation_gestures(&mut self, val: bool);

    /// Whether pressing a link previews its destination.
    #[objc::msg_send(allowsLinkPreview)]
    pub fn allows_link_preview(&self) -> bool;

    #[objc::msg_send(setAllowsLinkPreview:)]
    pub fn set_allows_link_preview(&mut self, val: bool);

    /// The color shown where the page does not reach, as when it is overscrolled.
    #[cfg(target_os = "macos")]
    #[objc::msg_send(underPageBackgroundColor)]
    #[objc::available(macos = 12.0)]
    pub fn under_page_bg_color(&self) -> arc::R<ns::Color>;

    /// `None` goes back to the page's own color.
    #[cfg(target_os = "macos")]
    #[objc::msg_send(setUnderPageBackgroundColor:)]
    #[objc::available(macos = 12.0)]
    pub fn set_under_page_bg_color(&mut self, val: Option<&ns::Color>);

    /// The color shown where the page does not reach, as when it is overscrolled.
    #[cfg(target_os = "ios")]
    #[objc::msg_send(underPageBackgroundColor)]
    #[objc::available(ios = 15.0)]
    pub fn under_page_bg_color(&self) -> arc::R<crate::ui::Color>;

    /// `None` goes back to the page's own color.
    #[cfg(target_os = "ios")]
    #[objc::msg_send(setUnderPageBackgroundColor:)]
    #[objc::available(ios = 15.0)]
    pub fn set_under_page_bg_color(&mut self, val: Option<&crate::ui::Color>);

    /// Edge insets, in the web view's coordinates, that shrink the layout viewport: the parts
    /// of the view covered by the client's own UI, such as a toolbar. All non-negative.
    #[cfg(target_os = "macos")]
    #[objc::msg_send(obscuredContentInsets)]
    #[objc::available(macos = 26.0)]
    pub fn obscured_content_insets(&self) -> ns::EdgeInsets;

    #[cfg(target_os = "macos")]
    #[objc::msg_send(setObscuredContentInsets:)]
    #[objc::available(macos = 26.0)]
    pub fn set_obscured_content_insets(&mut self, val: ns::EdgeInsets);

    /// Edge insets, in the web view's coordinates, that shrink the layout viewport: the parts
    /// of the view covered by the client's own UI, such as a navigation bar. All non-negative.
    #[cfg(target_os = "ios")]
    #[objc::msg_send(obscuredContentInsets)]
    #[objc::available(ios = 26.0)]
    pub fn obscured_content_insets(&self) -> crate::ui::EdgeInsets;

    #[cfg(target_os = "ios")]
    #[objc::msg_send(setObscuredContentInsets:)]
    #[objc::available(ios = 26.0)]
    pub fn set_obscured_content_insets(&mut self, val: crate::ui::EdgeInsets);

    /// The scroll view the page scrolls in.
    #[cfg(target_os = "ios")]
    #[objc::msg_send(scrollView)]
    pub fn scroll_view(&self) -> arc::R<crate::ui::ScrollView>;

    #[objc::msg_send(setNavigationDelegate:)]
    pub fn set_nav_delegate<D: wk::NavigationDelegate>(&mut self, val: Option<&D>);

    #[objc::msg_send(title)]
    pub fn title(&self) -> arc::R<ns::String>;

    #[objc::msg_send(isInspectable)]
    pub fn is_inpectable(&self) -> bool;

    #[objc::msg_send(setInspectable:)]
    pub fn set_inpectable(&mut self, val: bool);

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

    #[objc::msg_send(cameraCaptureState)]
    #[objc::available(macos = 12.0, ios = 15.0)]
    pub fn cam_capture_state(&self) -> wk::MediaCaptureState;

    #[objc::msg_send(setCameraCaptureState:completionHandler:)]
    pub fn set_cam_capture_state_ch_block(
        &mut self,
        val: wk::MediaCaptureState,
        block: Option<&mut blocks::CompletionBlock>,
    );

    #[objc::available(macos = 12.0, ios = 15.0)]
    pub fn set_cam_capture_state_ch(
        &mut self,
        val: wk::MediaCaptureState,
        block: impl FnMut() + 'static + std::marker::Send,
    ) {
        let mut block = blocks::CompletionBlock::new0(block);
        self.set_cam_capture_state_ch_block(val, Some(&mut block));
    }

    #[cfg(feature = "async")]
    #[objc::available(macos = 12.0, ios = 15.0)]
    pub async fn set_cam_capture_state(&mut self, val: wk::MediaCaptureState) {
        let (fut, mut block) = blocks::comp0();
        self.set_cam_capture_state_ch_block(val, Some(&mut block));
        fut.await
    }

    #[objc::msg_send(microphoneCaptureState)]
    #[objc::available(macos = 12.0, ios = 15.0)]

    pub fn mic_capture_state(&self) -> wk::MediaCaptureState;
    #[objc::msg_send(setMicrophoneCaptureState:completionHandler:)]
    pub fn set_mic_capture_state_ch_block(
        &mut self,
        val: wk::MediaCaptureState,
        block: Option<&mut blocks::CompletionBlock>,
    );

    #[objc::available(macos = 12.0, ios = 15.0)]
    pub fn set_mic_capture_state_ch(
        &mut self,
        val: wk::MediaCaptureState,
        block: impl FnMut() + 'static + std::marker::Send,
    ) {
        let mut block = blocks::CompletionBlock::new0(block);
        self.set_mic_capture_state_ch_block(val, Some(&mut block));
    }

    #[cfg(feature = "async")]
    #[objc::available(macos = 12.0, ios = 15.0)]
    pub async fn set_mic_capture_state(&mut self, val: wk::MediaCaptureState) {
        let (fut, mut block) = blocks::comp0();
        self.set_mic_capture_state_ch_block(val, Some(&mut block));
        fut.await
    }

    #[objc::msg_send(fullscreenState)]
    #[objc::available(macos = 13.0, ios = 16.0)]
    pub fn fullscreen_state(&self) -> wk::FullscreenState;
}
