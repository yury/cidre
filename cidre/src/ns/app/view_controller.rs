use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    pub ViewController(ns::Responder), NS_VIEW_CONTROLLER
);

#[link(name = "app", kind = "static")]
extern "C" {
    static NS_VIEW_CONTROLLER: &'static objc::Class<ViewController>;
}
