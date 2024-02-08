use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIImage")]
    pub Image(ns::Id),
    UI_IMAGE
);

impl Image {}

#[link(name = "ui", kind = "static")]
extern "C" {
    static UI_IMAGE: &'static objc::Class<Image>;
}
