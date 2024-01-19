use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    pub Responder(ns::Id),
    NS_RESPONDER
);

#[link(name = "app", kind = "static")]
extern "C" {
    static NS_RESPONDER: &'static objc::Class<Responder>;
}
