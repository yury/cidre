use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(View(ns::Id), UI_VIEW);

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_VIEW: &'static objc::Class<View>;
}
