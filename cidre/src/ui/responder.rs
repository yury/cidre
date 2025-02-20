use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIResponder")]
    pub Responder(ns::Id),
    UI_RESPONDER
);

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_RESPONDER: &'static objc::Class<Responder>;
}
