use std::ffi::c_void;

use crate::{arc, define_obj_type, dispatch, ns};

#[cfg(feature = "blocks")]
use crate::blocks;

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
    pub fn apply<'a, F>(&self, applier: &mut blocks::Block<F>) -> bool
    where
        F: FnMut(&'a dispatch::Data, usize, *const u8, usize) -> bool,
    {
        unsafe { dispatch_data_apply(self, applier.as_mut_ptr()) }
    }

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
    #[inline]
    pub fn copy_from_slice(data: &[u8]) -> arc::R<Self> {
        unsafe { dispatch_data_create(data.as_ptr(), data.len(), None, std::ptr::null_mut()) }
    }

    #[inline]
    pub fn from_static(bytes: &'static [u8]) -> arc::R<Self> {
        let mut b = blocks::fn0(destructor_noop);
        unsafe { dispatch_data_create(bytes.as_ptr(), bytes.len(), None, b.as_mut_ptr()) }
    }

    #[inline]
    pub fn with_bytes_no_copy<F: FnOnce()>(
        bytes: *const u8,
        len: usize,
        queue: Option<&dispatch::Queue>,
        destructor: &'static mut blocks::Block<F>,
    ) -> arc::R<Self> {
        unsafe { dispatch_data_create(bytes, len, queue, destructor.as_mut_ptr()) }
    }
}
extern "C" fn destructor_noop(_ctx: *const c_void) {}

impl From<&'static [u8]> for arc::R<Data> {
    fn from(slice: &'static [u8]) -> arc::R<Data> {
        Data::from_static(slice)
    }
}

impl From<&'static str> for arc::R<Data> {
    fn from(slice: &'static str) -> arc::R<Data> {
        Data::from_static(slice.as_bytes())
    }
}

impl From<Vec<u8>> for arc::R<Data> {
    fn from(val: Vec<u8>) -> arc::R<Data> {
        let len = val.len();
        let ptr = val.as_ptr();
        if len == 0 {
            return Data::empty().retained();
        }

        let destruct = blocks::once0(move || {
            std::mem::drop(val);
        });

        Data::with_bytes_no_copy(ptr, len, None, destruct.escape())
    }
}

impl From<Box<[u8]>> for arc::R<Data> {
    fn from(val: Box<[u8]>) -> arc::R<Data> {
        let len = val.len();
        let ptr = val.as_ptr();
        if len == 0 {
            return Data::empty().retained();
        }

        let destruct = blocks::once0(move || {
            std::mem::drop(val);
        });

        Data::with_bytes_no_copy(ptr, len, None, destruct.escape())
    }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    static _dispatch_data_empty: Data;

    fn dispatch_data_create(
        buffer: *const u8,
        size: usize,
        queue: Option<&dispatch::Queue>,
        destructor: *mut c_void,
    ) -> arc::R<Data>;

    fn dispatch_data_get_size(data: &Data) -> usize;
    fn dispatch_data_create_subrange(data: &Data, offset: usize, length: usize) -> arc::R<Data>;
    fn dispatch_data_create_concat(data1: &Data, data2: &Data) -> arc::R<Data>;
    #[cfg(feature = "blocks")]
    fn dispatch_data_apply(data: &Data, appier: *mut c_void) -> bool;
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
}
