use std::ffi::c_void;

use crate::{arc, define_obj_type, ns, objc::Obj};

define_obj_type!(pub ExceptionName(ns::String));

#[doc(alias = "NSStringFromSelector")]
#[inline]
pub fn string_from_selector(sel: &ns::objc::Sel) -> arc::R<ns::String> {
    unsafe { NSStringFromSelector(sel) }
}

#[doc(alias = "NSSelectorFromString")]
#[inline]
pub fn selector_from_ns_string(name: &ns::String) -> Option<&'static ns::objc::Sel> {
    unsafe { NSSelectorFromString(name) }
}

#[doc(alias = "NSStringFromClass")]
#[inline]
pub fn string_from_class<T: Obj>(cls: &ns::objc::Class<T>) -> arc::R<ns::String> {
    unsafe { NSStringFromClass(cls as *const ns::objc::Class<T> as _) }
}

#[doc(alias = "NSClassFromString")]
#[inline]
pub fn class_from_ns_string<T: Obj>(name: &ns::String) -> Option<&'static ns::objc::Class<T>> {
    unsafe { std::mem::transmute(NSClassFromString(name)) }
}

#[doc(alias = "NSProtocolFromString")]
#[inline]
pub fn protocol_from_ns_string(name: &ns::String) -> Option<&'static ns::objc::Protocol> {
    unsafe { NSProtocolFromString(name) }
}

#[doc(alias = "NSStringFromProtocol")]
#[inline]
pub fn string_from_protocol(proto: &ns::objc::Protocol) -> arc::R<ns::String> {
    unsafe { NSStringFromProtocol(proto) }
}

#[link(name = "Foundation", kind = "framework")]
extern "C-unwind" {
    fn NSStringFromSelector(sel: &ns::objc::Sel) -> arc::R<ns::String>;
    fn NSSelectorFromString(name: &ns::String) -> Option<&'static ns::objc::Sel>;

    fn NSStringFromClass(cls: *const c_void) -> arc::R<ns::String>;
    fn NSClassFromString(name: &ns::String) -> *const c_void;
    fn NSProtocolFromString(name: &ns::String) -> Option<&'static ns::objc::Protocol>;
    fn NSStringFromProtocol(proto: &ns::objc::Protocol) -> arc::R<ns::String>;
}

#[cfg(target_arch = "aarch64")]
#[cfg(test)]
mod tests {
    use crate::{ns, wk};

    #[test]
    fn basics() {
        let name = ns::str!(c"WKWebView");
        let cls = ns::class_from_ns_string::<ns::Id>(&name).unwrap();
        assert!(cls.as_type_ref().equal(wk::WebView::cls().as_type_ref()));
    }
}
