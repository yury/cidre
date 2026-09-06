use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "WKNavigationAction")]
    pub NavigationAction(ns::Id)
);

impl NavigationAction {
    #[objc::msg_send(request)]
    pub fn request(&self) -> arc::R<ns::UrlRequest>;
}
