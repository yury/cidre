use crate::{ns, objc, wk};

#[objc::protocol(WKNavigationDelegate)]
pub trait NavigationDelegate {
    #[objc::optional]
    #[objc::msg_send(webView:didFinishNavigation:)]
    fn web_view_did_finish_navigation(
        &mut self,
        web_view: &mut wk::WebView,
        navigation: Option<&wk::Navigation>,
    );

    #[objc::optional]
    #[objc::msg_send(webView:didFinishNavigation:withError:)]
    fn web_view_did_fail_navigation_err(
        &mut self,
        web_view: &mut wk::WebView,
        navigation: Option<&wk::Navigation>,
        error: &ns::Error,
    );
}
