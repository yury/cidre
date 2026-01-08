use std::{ffi::c_void, ptr::slice_from_raw_parts};

use crate::{arc, define_obj_type, dispatch};

#[cfg(feature = "ns")]
use crate::ns;

#[cfg(feature = "blocks")]
use crate::blocks;

#[doc(alias = "dispatch_data_applier_t")]
#[cfg(feature = "blocks")]
pub type Applier<Attr> = blocks::Block<fn(&dispatch::Data, usize, *const u8, usize) -> bool, Attr>;

define_obj_type!(
    #[doc(alias = "dispatch_data_t")]
    pub Data(dispatch::Object)
);

unsafe impl Send for Data {}
unsafe impl Sync for Data {}

impl Data {
    /// The singleton dispatch data object representing a zero-length
    /// memory region.
    #[doc(alias = "dispatch_data_empty")]
    #[inline]
    pub fn empty() -> &'static Self {
        unsafe { &_dispatch_data_empty }
    }

    #[doc(alias = "dispatch_data_get_size")]
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { dispatch_data_get_size(self) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "dispatch_data_apply")]
    #[inline]
    pub fn apply_block(&self, applier: &mut dispatch::DataApplier<blocks::NoEsc>) -> bool {
        unsafe { dispatch_data_apply(self, applier) }
    }

    #[cfg(feature = "blocks")]
    #[doc(alias = "dispatch_data_apply")]
    #[inline]
    pub fn apply(
        &self,
        mut applier: impl FnMut(&dispatch::Data, usize, *const u8, usize) -> bool,
    ) -> bool {
        unsafe {
            let mut block = dispatch::DataApplier::<blocks::NoEsc>::stack4(&mut applier);
            dispatch_data_apply(self, &mut block)
        }
    }

    #[cfg(feature = "ns")]
    #[inline]
    pub fn as_ns(&self) -> &ns::Data {
        unsafe { std::mem::transmute(self) }
    }

    #[doc(alias = "dispatch_data_create_subrange")]
    #[inline]
    pub fn subrange(&self, offset: usize, length: usize) -> arc::R<Self> {
        unsafe { dispatch_data_create_subrange(self, offset, length) }
    }

    #[doc(alias = "dispatch_data_create_concat")]
    #[inline]
    pub fn concat(a: &Self, b: &Self) -> arc::R<Self> {
        unsafe { dispatch_data_create_concat(a, b) }
    }

    #[doc(alias = "dispatch_data_create")]
    #[cfg(feature = "blocks")]
    #[inline]
    pub fn copy_from_slice(data: &[u8]) -> arc::R<Self> {
        unsafe { dispatch_data_create(data.as_ptr(), data.len(), None, None) }
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn from_static(bytes: &'static [u8]) -> arc::R<Self> {
        let mut b = blocks::StaticBlock::new0(destructor_noop);
        unsafe { dispatch_data_create(bytes.as_ptr(), bytes.len(), None, Some(b.as_esc_mut())) }
    }

    #[cfg(feature = "blocks")]
    #[inline]
    pub fn with_bytes_no_copy(
        bytes: *const u8,
        len: usize,
        queue: Option<&dispatch::Queue>,
        destructor: &mut dispatch::Block<blocks::Esc>,
    ) -> arc::R<Self> {
        unsafe { dispatch_data_create(bytes, len, queue, Some(destructor)) }
    }

    #[inline]
    pub fn map(&self) -> Map {
        unsafe {
            let mut data = std::ptr::null();
            let mut len = 0;
            let map = dispatch_data_create_map(self, &mut data, &mut len);
            Map { map, data, len }
        }
    }
}
extern "C" fn destructor_noop(_ctx: *const c_void) {}

#[cfg(feature = "blocks")]
impl From<&'static [u8]> for arc::R<Data> {
    fn from(slice: &'static [u8]) -> arc::R<Data> {
        Data::from_static(slice)
    }
}

#[cfg(feature = "blocks")]
impl From<&'static str> for arc::R<Data> {
    fn from(slice: &'static str) -> arc::R<Data> {
        Data::from_static(slice.as_bytes())
    }
}

#[cfg(feature = "blocks")]
impl From<Vec<u8>> for arc::R<Data> {
    fn from(val: Vec<u8>) -> arc::R<Data> {
        let len = val.len();
        let ptr = val.as_ptr();
        if len == 0 {
            return Data::empty().retained();
        }

        let mut destruct = dispatch::Block::<blocks::Esc>::new0(move || {
            let _f = &val;
        });

        Data::with_bytes_no_copy(ptr, len, None, &mut destruct)
    }
}

#[cfg(feature = "blocks")]
impl From<Box<[u8]>> for arc::R<Data> {
    fn from(val: Box<[u8]>) -> arc::R<Data> {
        let len = val.len();
        let ptr = val.as_ptr();
        if len == 0 {
            return Data::empty().retained();
        }

        let mut destruct = dispatch::Block::<blocks::Esc>::new0(move || {
            let _f = &val;
        });

        Data::with_bytes_no_copy(ptr, len, None, &mut destruct)
    }
}

#[link(name = "System", kind = "dylib")]
unsafe extern "C-unwind" {
    static _dispatch_data_empty: Data;

    #[cfg(feature = "blocks")]
    fn dispatch_data_create(
        buffer: *const u8,
        size: usize,
        queue: Option<&dispatch::Queue>,
        destructor: Option<&mut dispatch::Block<blocks::Esc>>,
    ) -> arc::R<Data>;

    fn dispatch_data_get_size(data: &Data) -> usize;
    fn dispatch_data_create_subrange(data: &Data, offset: usize, length: usize) -> arc::R<Data>;
    fn dispatch_data_create_concat(data1: &Data, data2: &Data) -> arc::R<Data>;
    #[cfg(feature = "blocks")]
    fn dispatch_data_apply(data: &Data, applier: &mut dispatch::DataApplier<blocks::NoEsc>)
    -> bool;

    fn dispatch_data_create_map(
        data: &Data,
        buffer_ptr: *mut *const u8,
        size: *mut usize,
    ) -> arc::R<Data>;
}

#[derive(Debug)]
pub struct Map {
    map: arc::R<Data>,
    data: *const u8,
    len: usize,
}

impl Map {
    #[inline]
    pub fn data(&self) -> &Data {
        &self.map
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self
    }
}

impl Clone for Map {
    fn clone(&self) -> Self {
        Self {
            map: self.map.retained(),
            data: self.data,
            len: self.len,
        }
    }
}

impl std::ops::Deref for Map {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*slice_from_raw_parts(self.data, self.len) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{arc, dispatch};

    #[test]
    fn basics() {
        let data = dispatch::Data::empty();
        assert!(data.is_empty());

        let data = data.subrange(5, 10);
        assert!(data.is_empty());

        let data = dispatch::Data::concat(&data, &data);

        let data = data.as_ns();
        assert!(data.is_empty());
    }

    #[test]
    fn statics() {
        let data1 = dispatch::Data::from_static(b"data1");
        let data2 = dispatch::Data::from_static(b"data2");
        assert_eq!(data1.len(), 5);
        assert_eq!(data2.len(), 5);

        let data3 = dispatch::Data::concat(&data1, &data2);
        assert_eq!(data3.len(), 10);
        assert_eq!(data3.as_ns().len(), 10);
        let mut ranges = vec![];
        data3.as_ns().enum_ranges(|ptr, range, _done| {
            assert!(!ptr.is_null());
            ranges.push(range);
        });
        assert_eq!(ranges.len(), 2);
        ranges.clear();
        data3.as_ns().enum_ranges(|ptr, range, done| {
            assert!(!ptr.is_null());
            ranges.push(range);
            *done = true;
        });
        assert_eq!(ranges.len(), 1);
    }

    #[test]
    fn from() {
        {
            let b: Box<[u8]> = Box::new(*b"12345");
            let data = arc::R::<dispatch::Data>::from(b);
            assert_eq!(data.len(), 5);
        }
        {
            let vec = vec![1; 1000];
            let data = arc::R::<dispatch::Data>::from(vec);
            assert_eq!(data.len(), 1000);
        }
        {
            let data = arc::R::<dispatch::Data>::from("hello!");
            assert_eq!(data.len(), 6);
        }
    }

    #[test]
    fn map() {
        let data1 = dispatch::Data::from_static(b"data1");
        let data2 = dispatch::Data::from_static(b"data2");
        let data3 = dispatch::Data::concat(&data1, &data2);

        let mut regions_offsets = vec![];
        data3.apply(|_data, offset, _ptr, _size| {
            regions_offsets.push(offset);
            true
        });

        assert_eq!(regions_offsets.len(), 2);

        let map = data3.map();
        let slice = map.as_slice();
        assert_eq!(data3.len(), slice.len());

        assert_eq!(slice[0], b"d"[0]);
        assert_eq!(slice[5], b"d"[0]);

        let slice = &map[..];
        assert_eq!(slice[1], b"a"[0]);
        assert_eq!(slice[6], b"a"[0]);

        let empty_map = dispatch::Data::empty().map();
        let slice = &empty_map[..];
        assert!(slice.is_empty());
    }
}
