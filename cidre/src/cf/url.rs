use std::{os::unix::prelude::OsStrExt, path::Path};

use crate::{arc, cf, define_cf_type};

#[cfg(feature = "ns")]
use crate::ns;

#[doc(alias = "CFURLPathStyle")]
#[derive(Debug, PartialEq, Eq)]
#[repr(isize)]
pub enum PathStyle {
    Posix = 0,
    // HFS = 1,
    Windows = 2,
}

define_cf_type!(
    #[doc(alias = "CFURL")]
    Url(cf::Type)
);

impl Url {
    #[doc(alias = "CFURLGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CFURLGetTypeID() }
    }

    #[doc(alias = "CFURLCreateWithBytes")]
    #[inline]
    pub fn with_bytes_in(
        url_bytes: *const u8,
        length: cf::Index,
        encoding: cf::StringEncoding,
        base_url: Option<&Url>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Url>> {
        unsafe { CFURLCreateWithBytes(allocator, url_bytes, length, encoding, base_url) }
    }

    #[doc(alias = "CFURLCreateWithString")]
    #[inline]
    pub fn with_cf_string_in(
        url_string: &cf::String,
        base_url: Option<&Url>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Url>> {
        unsafe { CFURLCreateWithString(allocator, url_string, base_url) }
    }

    #[doc(alias = "CFURLCreateWithFileSystemPath")]
    #[inline]
    pub fn with_file_system_path_in(
        file_path: &cf::String,
        path_style: PathStyle,
        is_directory: bool,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Url>> {
        unsafe { CFURLCreateWithFileSystemPath(allocator, file_path, path_style, is_directory) }
    }

    #[inline]
    pub fn with_path(path: &Path, is_dir: bool) -> Option<arc::R<Url>> {
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
        cf::Url::with_file_system_path_in(&path, PathStyle::Posix, is_dir, None)
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let url = cf::Url::from_str("http://google.com").unwrap();
    ///
    /// let scheme = url.scheme().unwrap();
    /// ```
    #[inline]
    pub fn from_str(str: &str) -> Option<arc::R<Url>> {
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
    /// let url = cf::Url::from_cf_string(&s1).unwrap();
    ///
    /// assert!(url.can_be_decomposed());
    /// ```
    #[inline]
    pub fn from_cf_string(str: &cf::String) -> Option<arc::R<Url>> {
        Self::with_cf_string_in(str, None, None)
    }

    /// Returns the Url's string. Percent-escape sequences are not removed
    ///
    /// # Examples
    /// ```
    /// use cidre::cf;
    ///
    /// let s1 = cf::String::from_str("https://github.com");
    /// let url = cf::Url::from_cf_string(&s1).unwrap();
    ///
    /// let s2 = url.cf_string();
    ///
    /// assert!(s1.equal(s2));
    ///
    /// ```
    #[doc(alias = "CFURLGetString")]
    #[inline]
    pub fn cf_string(&self) -> &cf::String {
        unsafe { CFURLGetString(self) }
    }

    #[doc(alias = "CFURLGetBaseURL")]
    #[inline]
    pub fn base_url(&self) -> Option<&Url> {
        unsafe { CFURLGetBaseURL(self) }
    }

    #[doc(alias = "CFURLCanBeDecomposed")]
    #[inline]
    pub fn can_be_decomposed(&self) -> bool {
        unsafe { CFURLCanBeDecomposed(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let url = cf::Url::from_str("https://localhost:3000").unwrap();
    /// let scheme = url.scheme().unwrap();
    ///
    /// let https = cf::String::from_str("https");
    /// assert!(https.equal(&scheme));
    /// ```
    #[doc(alias = "CFURLCopyScheme")]
    #[inline]
    pub fn scheme(&self) -> Option<arc::R<cf::String>> {
        unsafe { CFURLCopyScheme(self) }
    }

    /// ```
    /// use cidre::cf;
    ///
    /// let url1 = cf::Url::from_str("https://localhost:3000").unwrap();
    ///
    /// assert_eq!(3000, url1.port());
    /// let url2 = cf::Url::from_str("https://localhost").unwrap();
    ///
    /// assert_eq!(-1, url2.port());
    ///
    /// ```
    #[doc(alias = "CFURLGetPortNumber")]
    #[inline]
    pub fn port(&self) -> i32 {
        unsafe { CFURLGetPortNumber(self) }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns(&self) -> &ns::Url {
        unsafe { std::mem::transmute(self) }
    }

    #[doc(alias = "CFURLCopyAbsoluteURL")]
    #[inline]
    pub fn abs_url(&self) -> Option<arc::R<Self>> {
        unsafe { CFURLCopyAbsoluteURL(self) }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C-unwind" {
    fn CFURLGetTypeID() -> cf::TypeId;

    fn CFURLCreateWithBytes(
        allocator: Option<&cf::Allocator>,
        url_bytes: *const u8,
        length: cf::Index,
        encoding: cf::StringEncoding,
        base_url: Option<&Url>,
    ) -> Option<arc::R<Url>>;

    fn CFURLCreateWithString(
        allocator: Option<&cf::Allocator>,
        url_string: &cf::String,
        base_url: Option<&Url>,
    ) -> Option<arc::R<Url>>;

    fn CFURLCreateWithFileSystemPath(
        allocator: Option<&cf::Allocator>,
        file_path: &cf::String,
        path_style: PathStyle,
        is_directory: bool,
    ) -> Option<arc::R<Url>>;

    fn CFURLGetString(anURL: &Url) -> &cf::String;
    fn CFURLGetBaseURL(anURL: &Url) -> Option<&Url>;
    fn CFURLCanBeDecomposed(anURL: &Url) -> bool;
    fn CFURLCopyScheme(anURL: &Url) -> Option<arc::R<cf::String>>;
    fn CFURLGetPortNumber(anURL: &Url) -> i32;
    fn CFURLCopyAbsoluteURL(relativeURL: &Url) -> Option<arc::R<cf::Url>>;
}
