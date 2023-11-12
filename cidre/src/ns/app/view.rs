#[cfg(feature = "ca")]
use crate::ca;
use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(View(ns::Id), NS_VIEW);

impl View {
    #[cfg(feature = "ca")]
    #[objc::msg_send(layer)]
    pub fn layer(&self) -> Option<&ca::Layer>;
}

#[link(name = "app", kind = "static")]
extern "C" {
    static NS_VIEW: &'static objc::Class<View>;
}
