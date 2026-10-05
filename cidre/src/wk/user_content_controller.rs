use crate::{arc, define_obj_type, ns, objc, wk};

define_obj_type!(
    pub UserContentController(ns::Id),
    sym WKUserContentController
);

impl UserContentController {
    #[objc::msg_send(userScripts)]
    pub fn user_scripts(&self) -> arc::R<ns::Array<wk::UserScript>>;

    #[objc::msg_send(addUserScript:)]
    pub fn add_user_script(&mut self, val: &wk::UserScript);

    #[objc::msg_send(removeAllUserScripts)]
    pub fn remove_all_user_scripts(&mut self);

    /// Pages post to `handler` with `window.webkit.messageHandlers.<name>.postMessage(body)`.
    /// The controller keeps `handler`.
    #[objc::msg_send(addScriptMessageHandler:name:)]
    pub fn add_script_msg_handler<H: wk::ScriptMessageHandler>(
        &mut self,
        handler: &H,
        name: &ns::String,
    );

    #[objc::msg_send(removeScriptMessageHandlerForName:)]
    pub fn remove_script_msg_handler(&mut self, name: &ns::String);

    #[objc::msg_send(removeAllScriptMessageHandlers)]
    #[objc::available(macos = 11.0, ios = 14.0)]
    pub fn remove_all_script_msg_handlers(&mut self);
}
