use crate::{
    arc, define_obj_type, ns,
    objc::{msg_send, Obj},
};

define_obj_type!(URL(ns::Id));

impl URL {
    #[inline]
    pub fn file_url_relative_to(
        path: &ns::String,
        is_dir: bool,
        relative_to: Option<&ns::URL>,
    ) -> arc::R<Self> {
        unsafe { NSURL_fileURLWithPath_isDirectory_relativeToURL(path, is_dir, relative_to) }
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
        unsafe { NSURL_URLWithString_relativeToURL(str, relative_to) }
    }

    #[inline]
    pub fn with_string(str: &ns::String) -> Option<arc::R<Self>> {
        unsafe { NSURL_URLWithString_relativeToURL(str, None) }
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

    #[inline]
    pub fn abs_string(&self) -> Option<&ns::String> {
        unsafe { self.call0(msg_send::absolute_string) }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSURL_fileURLWithPath_isDirectory_relativeToURL(
        path: &ns::String,
        is_dir: bool,
        relative_to: Option<&ns::URL>,
    ) -> arc::R<ns::URL>;

    fn NSURL_URLWithString_relativeToURL(
        str: &ns::String,
        relative_to: Option<&ns::URL>,
    ) -> Option<arc::R<ns::URL>>;
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
