use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(UUID(ns::Id), NS_UUID);

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_UUID: &'static objc::Class<UUID>;

}
