use crate::{arc, define_cls, define_obj_type, ns, objc};

#[cfg(feature = "cf")]
use crate::cf;

use super::Class;

define_obj_type!(pub ResourceKey(ns::String));

define_obj_type!(pub Url(ns::Id));

impl arc::A<Url> {
    #[objc::msg_send(initFileURLWithPath:isDirectory:relativeToURL:)]
    pub fn init_with_path_is_dir_relative_to_url(
        self,
        path: &ns::String,
        is_dir: bool,
        relative_to: Option<&ns::Url>,
    ) -> arc::R<Url>;

    #[objc::msg_send(initWithString:relativeToURL:)]
    pub fn init_with_string_relative_to(
        self,
        string: &ns::String,
        relative_to: Option<&ns::Url>,
    ) -> Option<arc::R<ns::Url>>;
}

impl Url {
    define_cls!(NS_URL);

    #[inline]
    pub fn file_url_relative_to(
        path: &ns::String,
        is_dir: bool,
        relative_to: Option<&ns::Url>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_path_is_dir_relative_to_url(path, is_dir, relative_to)
    }

    #[inline]
    pub fn with_fs_path_string(path: &ns::String, is_dir: bool) -> arc::R<Self> {
        Self::file_url_relative_to(path, is_dir, None)
    }

    #[inline]
    pub fn with_fs_path_str(path: &str, is_dir: bool) -> arc::R<Self> {
        let string = ns::String::with_str_no_copy(path);
        Self::file_url_relative_to(&string, is_dir, None)
    }

    #[inline]
    pub fn with_string_relative_to(
        str: &ns::String,
        relative_to: Option<&ns::Url>,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_string_relative_to(str, relative_to)
    }

    #[inline]
    pub fn with_string(str: &ns::String) -> Option<arc::R<Self>> {
        Self::alloc().init_with_string_relative_to(str, None)
    }

    #[inline]
    pub fn with_str_relative_to(str: &str, relative_to: Option<&ns::Url>) -> Option<arc::R<Self>> {
        let string = ns::String::with_str_no_copy(str);
        Self::with_string_relative_to(&string, relative_to)
    }

    #[inline]
    pub fn with_str(str: &str) -> Option<arc::R<Self>> {
        let string = ns::String::with_str_no_copy(str);
        Self::with_string_relative_to(&string, None)
    }

    #[objc::msg_send(absoluteString)]
    pub fn abs_string_ar(&self) -> Option<arc::Rar<ns::String>>;

    #[objc::rar_retain()]
    pub fn abs_string(&self) -> Option<arc::R<ns::String>>;

    #[cfg(feature = "cf")]
    #[inline]
    pub fn as_cf(&self) -> &cf::Url {
        unsafe { std::mem::transmute(self) }
    }
}

unsafe impl Send for Url {}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_URL: &'static Class<Url>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let url = ns::Url::with_fs_path_str("/tmp", true);
        let abs_str = url.abs_string().unwrap();

        let url2 = ns::Url::with_str("file:///tmp/").unwrap();
        let abs_str2 = url2.abs_string().unwrap();

        assert!(abs_str.eq(&abs_str2));
    }
}
