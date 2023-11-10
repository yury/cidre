use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(View(ns::Id), UI_VIEW);

impl View {
    #[objc::msg_send(layer)]
    pub fn layer(&self) -> &ca::Layer;
}

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_VIEW: &'static objc::Class<View>;
}
