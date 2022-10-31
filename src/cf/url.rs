use crate::{cf, cf::Retained, define_cf_type};

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
        allocator: Option<&cf::Allocator>,
        url_bytes: *const u8,
        length: cf::Index,
        encoding: cf::StringEncoding,
        base_url: Option<&URL>,
    ) -> Option<Retained<URL>> {
        unsafe { CFURLCreateWithBytes(allocator, url_bytes, length, encoding, base_url) }
    }

    #[inline]
    pub fn with_string_in(
        allocator: Option<&cf::Allocator>,
        url_string: &cf::String,
        base_url: Option<&URL>,
    ) -> Option<Retained<URL>> {
        unsafe { CFURLCreateWithString(allocator, url_string, base_url) }
    }

    #[inline]
    pub fn with_file_system_path_in(
        allocator: Option<&cf::Allocator>,
        file_path: &cf::String,
        path_style: PathStyle,
        is_directory: bool,
    ) -> Option<Retained<URL>> {
        unsafe { CFURLCreateWithFileSystemPath(allocator, file_path, path_style, is_directory) }
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
    pub fn from_str(str: &str) -> Option<Retained<URL>> {
        Self::with_bytes_in(
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
    pub fn from_string(str: &cf::String) -> Option<Retained<URL>> {
        Self::with_string_in(None, str, None)
    }

    /// Returns the URL's string. Percent-escape sequences are not removed
    ///
    /// # Examples
    /// ```
    /// use cidre::cf;
    ///
    /// let s1 = cf::String::from_str("https://github.com");
    /// let url = cf::URL::from_string(&s1).unwrap();
    ///
    /// let s2 = url.string();
    ///
    /// assert!(s1.equal(s2));
    ///
    /// ```
    #[inline]
    pub fn string(&self) -> &cf::String {
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
    pub fn scheme(&self) -> Option<Retained<cf::String>> {
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
    ) -> Option<Retained<URL>>;
    fn CFURLCreateWithString(
        allocator: Option<&cf::Allocator>,
        url_string: &cf::String,
        base_url: Option<&URL>,
    ) -> Option<Retained<URL>>;

    fn CFURLCreateWithFileSystemPath(
        allocator: Option<&cf::Allocator>,
        file_path: &cf::String,
        path_style: PathStyle,
        is_directory: bool,
    ) -> Option<Retained<URL>>;

    fn CFURLGetString(anURL: &URL) -> &cf::String;
    fn CFURLGetBaseURL(anURL: &URL) -> Option<&URL>;

    fn CFURLCanBeDecomposed(anURL: &URL) -> bool;
    fn CFURLCopyScheme(anURL: &URL) -> Option<Retained<cf::String>>;

    fn CFURLGetPortNumber(anURL: &URL) -> i32;
}
