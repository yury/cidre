use std::{mem::transmute, ptr::slice_from_raw_parts};

use crate::{arc, cf, define_obj_type, define_options, msg_send, ns};

define_options!(ReadingOptions(usize));

impl ReadingOptions {
    /// Hint to map the file in if possible and safe
    #[doc(alias = "NSDataReadingMappedIfSafe")]
    pub const MAPPED_IF_SAFE: Self = Self(1 << 0);

    /// Hint to get the file not to be cached in the kernel
    #[doc(alias = "NSDataReadingUncached")]
    pub const UNCACHED: Self = Self(1 << 1);

    /// Hint to map the file in if possible. This takes precedence over NSDataReadingMappedIfSafe if both are given.
    #[doc(alias = "NSDataReadingMappedAlways")]
    pub const MAPPED_ALWAYS: Self = Self(1 << 3);
}

define_options!(WritingOptions(usize));

impl WritingOptions {
    pub const ATOMIC: Self = Self(1 << 0);
    pub const FILE_PROTECTION_NONE: Self = Self(0x10000000);
    pub const FILE_PROTECTION_COMPLETE: Self = Self(0x20000000);
    pub const FILE_PROTECTION_COMPLETE_UNLESS_OPEN: Self = Self(0x30000000);
    pub const FILE_PROTECTION_COMPLETE_UNTIL_FIRST_USER_AUTHENTICATION: Self = Self(0x40000000);
    pub const FILE_PROTECTION_MASK: Self = Self(0xf0000000);
}

define_options!(SearchOptions(usize));

impl SearchOptions {
    pub const BACKWARDS: Self = Self(1 << 0);
    pub const ANCHORED: Self = Self(1 << 1);
}

define_obj_type!(Data(ns::Id));

impl Data {
    #[inline]
    pub unsafe fn with_contents_of_file_options_error(
        path: &cf::String,
        options: ReadingOptions,
        error: &mut Option<&cf::Error>,
    ) -> Option<arc::R<Self>> {
        NSData_dataWithContentsOfFile_options_error(path, options, error)
    }

    #[inline]
    pub fn with_contents_of_file_options(
        path: &cf::String,
        options: ReadingOptions,
    ) -> Result<arc::R<Self>, &cf::Error> {
        unsafe {
            let mut error = None;
            let res = Self::with_contents_of_file_options_error(path, options, &mut error);
            match res {
                Some(r) => Ok(r),
                None => Err(transmute(error)),
            }
        }
    }

    #[inline]
    pub unsafe fn with_contents_of_url_options_error(
        url: &cf::URL,
        options: ReadingOptions,
        error: &mut Option<&cf::Error>,
    ) -> Option<arc::R<Self>> {
        NSData_dataWithContentsOfURL_options_error(url, options, error)
    }

    #[inline]
    pub fn with_contents_of_url_options(
        url: &cf::URL,
        options: ReadingOptions,
    ) -> Result<arc::R<Self>, &cf::Error> {
        unsafe {
            let mut error = None;
            let res = Self::with_contents_of_url_options_error(url, options, &mut error);
            match res {
                Some(r) => Ok(r),
                None => Err(transmute(error)),
            }
        }
    }

    #[inline]
    pub fn length(&self) -> usize {
        msg_send!("ns", self, ns_length)
    }

    #[inline]
    pub fn bytes(&self) -> *const u8 {
        msg_send!("ns", self, ns_bytes)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length()
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { &*slice_from_raw_parts(self.bytes(), self.len()) }
    }

    /// Writes the contents of the receiver to the file specified by a given path.
    ///
    /// Writes the contents of the receiver to the file specified by path (overwriting any existing file at path).
    /// path is written in the default C-string encoding if possible (that is,
    /// if no information would be lost), in the Unicode encoding otherwise.
    ///
    /// If flag is YES, the receiver is written to an auxiliary file, and then the auxiliary file is renamed to path.
    /// If flag is NO, the receiver is written directly to path. The YES option guarantees that path,
    /// if it exists at all, won’t be corrupted even if the system should crash during writing.
    ///
    /// If path contains a tilde (~) character, you must expand it with stringByExpandingTildeInPath
    /// before invoking this method.
    #[inline]
    pub fn write_to_file(&self, path: &cf::String, atomically: bool) -> bool {
        unsafe { rsel_writeToFile_atomically(self, path, atomically) }
    }

    #[inline]
    pub fn write_at_path(&self, path: &cf::String, atomically: bool) -> Result<(), ()> {
        if self.write_to_file(path, atomically) {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    fn NSData_dataWithContentsOfFile_options_error(
        path: &cf::String,
        options: ReadingOptions,
        error: &mut Option<&cf::Error>,
    ) -> Option<arc::R<Data>>;

    fn NSData_dataWithContentsOfURL_options_error(
        url: &cf::URL,
        options: ReadingOptions,
        error: &mut Option<&cf::Error>,
    ) -> Option<arc::R<Data>>;

    fn rsel_writeToFile_atomically(id: &ns::Id, path: &cf::String, atomically: bool) -> bool;
}
