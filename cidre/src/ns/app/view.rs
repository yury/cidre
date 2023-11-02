use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(View(ns::Id), NS_VIEW);

#[link(name = "app", kind = "static")]
extern "C" {
    static NS_VIEW: &'static objc::Class<View>;
}
