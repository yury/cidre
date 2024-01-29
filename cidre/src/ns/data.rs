use std::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};

use crate::{arc, blocks, cf, define_obj_type, define_opts, ns, objc};

define_opts!(
    #[doc(alias = "NSDataReadingOptions")]
    pub ReadOpts(usize)
);

impl ReadOpts {
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

define_opts!(
    #[doc(alias = "NSDataWritingOptions")]
    pub WriteOpts(usize)
);

impl WriteOpts {
    pub const ATOMIC: Self = Self(1 << 0);
    pub const FILE_PROTECTION_NONE: Self = Self(0x10000000);
    pub const FILE_PROTECTION_COMPLETE: Self = Self(0x20000000);
    pub const FILE_PROTECTION_COMPLETE_UNLESS_OPEN: Self = Self(0x30000000);
    pub const FILE_PROTECTION_COMPLETE_UNTIL_FIRST_USER_AUTHENTICATION: Self = Self(0x40000000);
    pub const FILE_PROTECTION_MASK: Self = Self(0xf0000000);
}

define_opts!(
    #[doc(alias = "NSDataSearchOptions")]
    pub SearchOpts(usize)
);

impl SearchOpts {
    pub const BACKWARDS: Self = Self(1 << 0);
    pub const ANCHORED: Self = Self(1 << 1);
}

define_obj_type!(
    #[doc(alias = "NSData")]
    pub Data(ns::Id), NS_DATA
);

define_obj_type!(
    #[doc(alias = "NSMutableData")]
    pub DataMut(Data), NS_MUTABLE_DATA
);

impl arc::A<Data> {
    #[objc::msg_send(initWithContentsOfFile:options:error:)]
    pub fn init_with_contents_of_file_opts_err<'ear>(
        self,
        path: &ns::String,
        options: ReadOpts,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Data>>;

    #[objc::msg_send(initWithContentsOfURL:options:error:)]
    pub fn init_with_contents_of_url_opts_err<'ear>(
        self,
        url: &ns::Url,
        options: ReadOpts,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Data>>;
}

impl Data {
    #[inline]
    pub fn with_contents_of_file<'ear>(
        path: &ns::String,
        options: ReadOpts,
    ) -> Result<arc::R<Self>, &'ear ns::Error> {
        ns::if_none(|err| Self::alloc().init_with_contents_of_file_opts_err(path, options, err))
    }

    #[inline]
    pub fn with_contents_of_url<'ear>(
        url: &ns::Url,
        options: ReadOpts,
    ) -> Result<arc::R<Self>, &'ear ns::Error> {
        ns::if_none(|err| Self::alloc().init_with_contents_of_url_opts_err(url, options, err))
    }

    #[objc::msg_send(bytes)]
    pub fn bytes(&self) -> *const u8;

    #[objc::msg_send(length)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { &*slice_from_raw_parts(self.bytes(), self.len()) }
    }

    #[objc::msg_send(getBytes:length:)]
    pub fn get_bytes(&self, buffer: *mut u8, length: usize);

    #[objc::msg_send(getBytes:range:)]
    pub fn get_bytes_range(&self, buffer: *mut u8, range: ns::Range);

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
    #[objc::msg_send(writeToFile:atomically:)]
    pub fn write_to_file(&self, path: &ns::String, atomically: bool) -> bool;

    #[inline]
    pub fn write_at_path(&self, path: &ns::String, atomically: bool) -> Result<(), ()> {
        if self.write_to_file(path, atomically) {
            Ok(())
        } else {
            Err(())
        }
    }

    #[inline]
    pub fn as_cf(&self) -> &cf::Data {
        unsafe { std::mem::transmute(self) }
    }
}

/// NSExtendedData
impl Data {
    #[objc::msg_send(enumerateByteRangesUsingBlock:)]
    pub fn enumerate_byte_ranges_using_block(
        &self,
        block: &mut blocks::NoEscBlock<fn(*const u8, ns::Range, &mut bool)>,
    );

    #[inline]
    pub fn enum_ranges(&self, mut block: impl FnMut(*const u8, ns::Range, &mut bool)) {
        unsafe {
            let mut block = blocks::NoEscBlock::stack3(&mut block);
            self.enumerate_byte_ranges_using_block(&mut block)
        }
    }
}

impl DataMut {
    #[objc::msg_send(mutableBytes)]
    pub unsafe fn bytes_mut(&mut self) -> *mut u8;

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { &mut *slice_from_raw_parts_mut(self.bytes_mut(), self.len()) }
    }

    #[objc::msg_send(appendData:)]
    pub fn append_data(&mut self, data: &ns::Data);

    #[objc::msg_send(increaseLengthBy:)]
    pub fn increase_len_by(&mut self, extra_len: usize);

    pub fn as_cf_mut(&mut self) -> &mut cf::DataMut {
        unsafe { std::mem::transmute(self) }
    }
}

#[link(name = "ns", kind = "static")]
extern "C" {
    static NS_DATA: &'static objc::Class<Data>;
    static NS_MUTABLE_DATA: &'static objc::Class<DataMut>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let data = ns::Data::new();
        assert!(data.is_empty());

        let mut data = ns::DataMut::new();
        assert!(data.is_empty());

        let slice = data.as_mut_slice();
        assert!(slice.is_empty());

        data.increase_len_by(10);

        assert_eq!(10, data.len());
    }
}
