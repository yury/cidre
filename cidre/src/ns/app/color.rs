use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(pub Color(ns::Id), NS_COLOR);

impl Color {}

unsafe impl Send for Color {}

#[link(name = "app", kind = "static")]
extern "C" {
    static NS_COLOR: &'static objc::Class<Color>;
}
