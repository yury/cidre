use std::ffi::c_void;

use crate::{arc, define_obj_type, ns, objc::Obj};

#[doc(alias = "NSComparisonResult")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ComparisonResult {
    Ascending = -1,
    Same = 0,
    Descending = 1,
}

impl From<std::cmp::Ordering> for ComparisonResult {
    fn from(value: std::cmp::Ordering) -> Self {
        let value = value as i8 as isize;
        unsafe { std::mem::transmute(value) }
    }
}

impl From<ComparisonResult> for std::cmp::Ordering {
    fn from(value: ComparisonResult) -> Self {
        let value = value as isize as i8;
        unsafe { std::mem::transmute(value) }
    }
}

#[cfg(feature = "blocks")]
#[doc(alias = "NSComparator")]
pub type Comparator<T, Attr> = crate::blocks::Block<fn(a: &T, b: &T) -> ns::ComparisonResult, Attr>;

#[doc(alias = "NSNotFound")]
pub const NOT_FOUND: ns::Integer = ns::Integer::MAX;

define_obj_type!(pub ExceptionName(ns::String));

#[doc(alias = "NSStringFromSelector")]
#[inline]
pub fn string_from_selector(sel: &ns::objc::Sel) -> arc::R<ns::String> {
    unsafe { arc::rar_retain(NSStringFromSelector(sel)) }
}

#[doc(alias = "NSSelectorFromString")]
#[inline]
pub fn selector_from_ns_string(name: &ns::String) -> Option<&'static ns::objc::Sel> {
    unsafe { NSSelectorFromString(name) }
}

#[doc(alias = "NSStringFromClass")]
#[inline]
pub fn string_from_class<T: Obj>(cls: &ns::objc::Class<T>) -> arc::R<ns::String> {
    unsafe { arc::rar_retain(NSStringFromClass(cls as *const ns::objc::Class<T> as _)) }
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
    unsafe { arc::rar_retain(NSStringFromProtocol(proto)) }
}

#[link(name = "Foundation", kind = "framework")]
unsafe extern "C-unwind" {
    fn NSStringFromSelector(sel: &ns::objc::Sel) -> arc::Rar<ns::String>;
    fn NSSelectorFromString(name: &ns::String) -> Option<&'static ns::objc::Sel>;

    fn NSStringFromClass(cls: *const c_void) -> arc::Rar<ns::String>;
    fn NSClassFromString(name: &ns::String) -> *const c_void;
    fn NSProtocolFromString(name: &ns::String) -> Option<&'static ns::objc::Protocol>;
    fn NSStringFromProtocol(proto: &ns::objc::Protocol) -> arc::Rar<ns::String>;
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
