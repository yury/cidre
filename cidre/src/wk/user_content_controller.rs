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
}
