use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSColorSpace")]
    pub ColorSpace(ns::Id),
    NS_COLOR_SPACE
);

unsafe extern "C" {
    static NS_COLOR_SPACE: &'static objc::Class<ColorSpace>;
}
