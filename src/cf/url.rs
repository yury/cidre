use crate::{cf, define_cf_type};

use super::Retained;

#[repr(isize)]
pub enum PathStyle {
    Posix = 0,
    // HFS = 1,
    Windows = 2,
}

define_cf_type!(URL(cf::Type));

impl URL {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFURLGetTypeID() }
    }

    #[inline]
    pub fn create_with_bytes<'a>(
        allocator: Option<&cf::Allocator>,
        url_bytes: *const u8,
        length: cf::Index,
        encoding: cf::StringEncoding,
        base_url: Option<&URL>,
    ) -> Option<Retained<'a, URL>> {
        unsafe { CFURLCreateWithBytes(allocator, url_bytes, length, encoding, base_url) }
    }

    #[inline]
    pub fn create_with_string<'a>(
        allocator: Option<&cf::Allocator>,
        url_string: &cf::String,
        base_url: Option<&URL>,
    ) -> Option<Retained<'a, URL>> {
        unsafe { CFURLCreateWithString(allocator, url_string, base_url) }
    }

    #[inline]
    pub fn create_with_file_system_path<'a>(
        allocator: Option<&cf::Allocator>,
        file_path: &cf::String,
        path_style: PathStyle,
        is_directory: bool,
    ) -> Option<Retained<'a, URL>> {
        unsafe { CFURLCreateWithFileSystemPath(allocator, file_path, path_style, is_directory) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let url = cf::URL::from_str("http://google.com").unwrap();
    ///
    /// let scheme = url.copy_scheme().unwrap();
    ///
    /// ```
    #[inline]
    pub fn from_str<'a>(str: &str) -> Option<Retained<'a, URL>> {
        Self::create_with_bytes(
            None,
            str.as_ptr(),
            str.len() as _,
            cf::StringEncoding::UTF8,
            None,
        )
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let s1 = cf::String::from_str("https://github.com");
    /// let url = cf::URL::from_string(&s1).unwrap();
    ///
    /// assert!(url.can_be_decomposed());
    ///
    /// ```
    #[inline]
    pub fn from_string<'a>(str: &cf::String) -> Option<Retained<'a, URL>> {
        Self::create_with_string(None, str, None)
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let s1 = cf::String::from_str("https://github.com");
    /// let url = cf::URL::from_string(&s1).unwrap();
    ///
    /// let s2 = url.get_string();
    ///
    /// assert!(s1.equal(s2));
    ///
    /// ```
    #[inline]
    pub fn get_string(&self) -> &cf::String {
        unsafe { CFURLGetString(self) }
    }

    #[inline]
    pub fn get_base_url(&self) -> Option<&URL> {
        unsafe { CFURLGetBaseURL(self) }
    }

    #[inline]
    pub fn can_be_decomposed(&self) -> bool {
        unsafe { CFURLCanBeDecomposed(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let url = cf::URL::from_str("https://localhost:3000").unwrap();
    /// let scheme = url.copy_scheme().unwrap();
    ///
    /// let https = cf::String::from_str_no_copy("https");
    /// assert!(https.equal(&scheme));
    /// ```
    #[inline]
    pub fn copy_scheme<'a>(&self) -> Option<Retained<'a, cf::String>> {
        unsafe { CFURLCopyScheme(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let url1 = cf::URL::from_str("https://localhost:3000").unwrap();
    ///
    /// assert_eq!(3000, url1.get_port());

    /// let url2 = cf::URL::from_str("https://localhost").unwrap();
    ///
    /// assert_eq!(-1, url2.get_port());
    ///
    /// ```
    #[inline]
    pub fn get_port(&self) -> i32 {
        unsafe { CFURLGetPortNumber(self) }
    }
}

extern "C" {
    fn CFURLGetTypeID() -> cf::TypeId;
    fn CFURLCreateWithBytes<'a>(
        allocator: Option<&cf::Allocator>,
        url_bytes: *const u8,
        length: cf::Index,
        encoding: cf::StringEncoding,
        base_url: Option<&URL>,
    ) -> Option<Retained<'a, URL>>;
    fn CFURLCreateWithString<'a>(
        allocator: Option<&cf::Allocator>,
        url_string: &cf::String,
        base_url: Option<&URL>,
    ) -> Option<Retained<'a, URL>>;

    fn CFURLCreateWithFileSystemPath<'a>(
        allocator: Option<&cf::Allocator>,
        file_path: &cf::String,
        path_style: PathStyle,
        is_directory: bool,
    ) -> Option<Retained<'a, URL>>;

    fn CFURLGetString(anURL: &URL) -> &cf::String;
    fn CFURLGetBaseURL(anURL: &URL) -> Option<&URL>;

    fn CFURLCanBeDecomposed(anURL: &URL) -> bool;
    fn CFURLCopyScheme<'a>(anURL: &URL) -> Option<Retained<'a, cf::String>>;

    fn CFURLGetPortNumber(anURL: &URL) -> i32;
}
