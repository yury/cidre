use std::{os::unix::prelude::OsStrExt, path::Path};

use crate::{arc, cf, define_cf_type};

#[derive(Debug, PartialEq, Eq)]
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

    /// CFURLCreateWithBytes
    #[inline]
    pub fn with_bytes_in(
        url_bytes: *const u8,
        length: cf::Index,
        encoding: cf::StringEncoding,
        base_url: Option<&URL>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<URL>> {
        unsafe { CFURLCreateWithBytes(allocator, url_bytes, length, encoding, base_url) }
    }

    #[inline]
    pub fn with_cf_string_in(
        url_string: &cf::String,
        base_url: Option<&URL>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<URL>> {
        unsafe { CFURLCreateWithString(allocator, url_string, base_url) }
    }

    #[inline]
    pub fn with_file_system_path_in(
        file_path: &cf::String,
        path_style: PathStyle,
        is_directory: bool,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<URL>> {
        unsafe { CFURLCreateWithFileSystemPath(allocator, file_path, path_style, is_directory) }
    }

    #[inline]
    pub fn with_path(path: &Path, is_dir: bool) -> Option<arc::R<URL>> {
        let bytes = path.as_os_str().as_bytes();
        let encoding = cf::StringEncoding::system_encoding();
        let Some(path) = cf::String::create_with_bytes_no_copy_in(
            bytes,
            encoding,
            false,
            cf::Allocator::null(),
            None,
        ) else {
            return None;
        };
        cf::URL::with_file_system_path_in(&path, PathStyle::Posix, is_dir, None)
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let url = cf::URL::from_str("http://google.com").unwrap();
    ///
    /// let scheme = url.scheme().unwrap();
    ///
    /// ```
    #[inline]
    pub fn from_str(str: &str) -> Option<arc::R<URL>> {
        Self::with_bytes_in(
            str.as_ptr(),
            str.len() as _,
            cf::StringEncoding::UTF8,
            None,
            None,
        )
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let s1 = cf::String::from_str("https://github.com");
    /// let url = cf::URL::from_cf_string(&s1).unwrap();
    ///
    /// assert!(url.can_be_decomposed());
    ///
    /// ```
    #[inline]
    pub fn from_cf_string(str: &cf::String) -> Option<arc::R<URL>> {
        Self::with_cf_string_in(str, None, None)
    }

    /// Returns the URL's string. Percent-escape sequences are not removed
    ///
    /// # Examples
    /// ```
    /// use cidre::cf;
    ///
    /// let s1 = cf::String::from_str("https://github.com");
    /// let url = cf::URL::from_cf_string(&s1).unwrap();
    ///
    /// let s2 = url.cf_string();
    ///
    /// assert!(s1.equal(s2));
    ///
    /// ```
    #[inline]
    pub fn cf_string(&self) -> &cf::String {
        unsafe { CFURLGetString(self) }
    }

    #[inline]
    pub fn base_url(&self) -> Option<&URL> {
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
    /// let scheme = url.scheme().unwrap();
    ///
    /// let https = cf::String::from_str_no_copy("https");
    /// assert!(https.equal(&scheme));
    /// ```
    #[inline]
    pub fn scheme(&self) -> Option<arc::R<cf::String>> {
        unsafe { CFURLCopyScheme(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let url1 = cf::URL::from_str("https://localhost:3000").unwrap();
    ///
    /// assert_eq!(3000, url1.port());

    /// let url2 = cf::URL::from_str("https://localhost").unwrap();
    ///
    /// assert_eq!(-1, url2.port());
    ///
    /// ```
    #[inline]
    pub fn port(&self) -> i32 {
        unsafe { CFURLGetPortNumber(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFURLGetTypeID() -> cf::TypeId;
    fn CFURLCreateWithBytes(
        allocator: Option<&cf::Allocator>,
        url_bytes: *const u8,
        length: cf::Index,
        encoding: cf::StringEncoding,
        base_url: Option<&URL>,
    ) -> Option<arc::R<URL>>;
    fn CFURLCreateWithString(
        allocator: Option<&cf::Allocator>,
        url_string: &cf::String,
        base_url: Option<&URL>,
    ) -> Option<arc::R<URL>>;

    fn CFURLCreateWithFileSystemPath(
        allocator: Option<&cf::Allocator>,
        file_path: &cf::String,
        path_style: PathStyle,
        is_directory: bool,
    ) -> Option<arc::R<URL>>;

    fn CFURLGetString(anURL: &URL) -> &cf::String;
    fn CFURLGetBaseURL(anURL: &URL) -> Option<&URL>;

    fn CFURLCanBeDecomposed(anURL: &URL) -> bool;
    fn CFURLCopyScheme(anURL: &URL) -> Option<arc::R<cf::String>>;

    fn CFURLGetPortNumber(anURL: &URL) -> i32;
}
