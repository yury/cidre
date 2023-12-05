use std::ffi::c_void;

use crate::{arc, define_obj_type, ns, objc::Obj};

define_obj_type!(pub ExceptionName(ns::String));

#[inline]
pub fn ns_string_from_selector(sel: &ns::objc::Sel) -> arc::R<ns::String> {
    unsafe { NSStringFromSelector(sel) }
}

#[inline]
pub fn ns_selector_from_ns_string(name: &ns::String) -> Option<&'static ns::objc::Sel> {
    unsafe { NSSelectorFromString(name) }
}

#[inline]
pub fn ns_string_from_class<T: Obj>(cls: &ns::objc::Class<T>) -> arc::R<ns::String> {
    unsafe { NSStringFromClass(cls as *const ns::objc::Class<T> as _) }
}

#[inline]
pub fn ns_class_from_ns_string(name: &ns::String) -> Option<&'static ns::objc::Class<ns::Id>> {
    unsafe { std::mem::transmute(NSClassFromString(name)) }
}

extern "C" {
    fn NSStringFromSelector(sel: &ns::objc::Sel) -> arc::R<ns::String>;
    fn NSSelectorFromString(name: &ns::String) -> Option<&'static ns::objc::Sel>;

    fn NSStringFromClass(cls: *const c_void) -> arc::R<ns::String>;
    fn NSClassFromString(name: &ns::String) -> *const c_void;
}

#[cfg(target_arch = "aarch64")]
#[cfg(test)]
mod tests {
    use crate::{ns, wk};

    use super::ns_class_from_ns_string;

    #[test]
    fn basics() {
        let name = ns::String::with_str("WKWebView");
        let cls = ns_class_from_ns_string(&name).unwrap();
        assert!(cls.as_type_ref().equal(wk::WebView::cls().as_type_ref()));
    }
}
