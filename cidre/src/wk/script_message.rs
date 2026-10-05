use crate::{arc, define_obj_type, ns, objc, wk};

define_obj_type!(
    #[doc(alias = "WKScriptMessage")]
    pub ScriptMessage(ns::Id)
);

impl ScriptMessage {
    /// What the page posted: an `ns::Number`, `ns::String`, `ns::Date`, `ns::Array`,
    /// `ns::Dictionary` or `ns::Null`.
    #[objc::msg_send(body)]
    pub fn body(&self) -> arc::R<ns::Id>;

    #[objc::msg_send(webView)]
    pub fn web_view(&self) -> Option<arc::R<wk::WebView>>;

    /// The handler's name it was posted to.
    #[objc::msg_send(name)]
    pub fn name(&self) -> arc::R<ns::String>;
}

#[objc::protocol(WKScriptMessageHandler)]
pub trait ScriptMessageHandler: objc::Obj {
    #[objc::msg_send(userContentController:didReceiveScriptMessage:)]
    fn user_content_controller_did_receive_script_msg(
        &mut self,
        controller: &mut wk::UserContentController,
        msg: &wk::ScriptMessage,
    );
}

define_obj_type!(
    pub AnyScriptMessageHandler(ns::Id)
);

impl ScriptMessageHandler for AnyScriptMessageHandler {}
