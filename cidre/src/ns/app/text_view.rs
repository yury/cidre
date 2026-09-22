use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSTextView")]
    pub TextView(ns::Text),
    NS_TEXT_VIEW
);

unsafe extern "C" {
    static NS_TEXT_VIEW: &'static objc::Class<TextView>;
}
