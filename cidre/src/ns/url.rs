use crate::{arc, define_cls, define_obj_type, ns, objc};

use super::Class;

define_obj_type!(URL(ns::Id));

impl arc::A<URL> {
    #[objc::msg_send(initFileURLWithPath:isDirectory:relativeToURL:)]
    pub fn init_with_path_is_dir_relative_to_url(
        self,
        path: &ns::String,
        is_dir: bool,
        relative_to: Option<&ns::URL>,
    ) -> arc::R<URL>;

    #[objc::msg_send(initWithString:relativeToURL:)]
    pub fn init_with_string_relative_to(
        self,
        string: &ns::String,
        relative_to: Option<&ns::URL>,
    ) -> Option<arc::R<ns::URL>>;
}

impl URL {
    define_cls!(NS_URL);

    #[inline]
    pub fn file_url_relative_to(
        path: &ns::String,
        is_dir: bool,
        relative_to: Option<&ns::URL>,
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
        relative_to: Option<&ns::URL>,
    ) -> Option<arc::R<Self>> {
        Self::alloc().init_with_string_relative_to(str, relative_to)
    }

    #[inline]
    pub fn with_string(str: &ns::String) -> Option<arc::R<Self>> {
        Self::alloc().init_with_string_relative_to(str, None)
    }

    #[inline]
    pub fn with_str_relative_to(str: &str, relative_to: Option<&ns::URL>) -> Option<arc::R<Self>> {
        let string = ns::String::with_str_no_copy(str);
        Self::with_string_relative_to(&string, relative_to)
    }

    #[inline]
    pub fn with_str(str: &str) -> Option<arc::R<Self>> {
        let string = ns::String::with_str_no_copy(str);
        Self::with_string_relative_to(&string, None)
    }

    #[objc::msg_send(absoluteString)]
    pub fn abs_string_ar(&self) -> Option<&'ar ns::String>;

    #[objc::rar_retain()]
    pub fn abs_string(&self) -> Option<arc::R<ns::String>>;
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_URL: &'static Class<URL>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let url = ns::URL::with_fs_path_str("/tmp", true);
        let abs_str = url.abs_string().unwrap();

        let url2 = ns::URL::with_str("file:///tmp/").unwrap();
        let abs_str2 = url2.abs_string().unwrap();

        assert!(abs_str.eq(&abs_str2));
    }
}
