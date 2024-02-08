use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSImage")]
    pub Image(ns::Id),
    NS_IMAGE
);

impl Image {}

#[link(name = "app", kind = "static")]
extern "C" {
    static NS_IMAGE: &'static objc::Class<Image>;
}
