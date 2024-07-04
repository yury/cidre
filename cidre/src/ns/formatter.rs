use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSFormatter")]
    pub Formatter(ns::Id)
);

impl Formatter {
    #[objc::msg_send(stringForObjectValue:)]
    pub fn string_for_obj_value(&self, obj: &ns::Id) -> Option<arc::R<ns::String>>;
}
