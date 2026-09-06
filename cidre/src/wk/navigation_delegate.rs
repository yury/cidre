use crate::{ns, objc, wk};

#[doc(alias = "WKNavigationActionPolicy")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(isize)]
pub enum NavigationActionPolicy {
    Cancel = 0,
    Allow = 1,
    Download = 2,
}

#[objc::protocol(WKNavigationDelegate)]
pub trait NavigationDelegate {
    #[cfg(feature = "blocks")]
    #[objc::optional]
    #[objc::msg_send(webView:decidePolicyForNavigationAction:decisionHandler:)]
    fn web_view_decide_policy_for_navigation_action(
        &mut self,
        web_view: &mut wk::WebView,
        navigation_action: &wk::NavigationAction,
        decision_handler: &mut crate::blocks::EscBlock<fn(NavigationActionPolicy)>,
    );

    #[objc::optional]
    #[objc::msg_send(webView:didFinishNavigation:)]
    fn web_view_did_finish_navigation(
        &mut self,
        web_view: &mut wk::WebView,
        navigation: Option<&wk::Navigation>,
    );

    #[objc::optional]
    #[objc::msg_send(webView:didFailNavigation:withError:)]
    fn web_view_did_fail_navigation_err(
        &mut self,
        web_view: &mut wk::WebView,
        navigation: Option<&wk::Navigation>,
        error: &ns::Error,
    );
}
