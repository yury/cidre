use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(Color(ns::Id), UI_COLOR);

unsafe impl Send for Color {}

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_COLOR: &'static objc::Class<Color>;
}
