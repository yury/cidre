use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "UIFont")]
    pub Font(ns::Id),
    UI_FONT
);

#[link(name = "ui", kind = "static")]
unsafe extern "C" {
    static UI_FONT: &'static objc::Class<Font>;
}
