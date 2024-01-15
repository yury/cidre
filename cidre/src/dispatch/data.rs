use std::ffi::c_void;

use crate::{arc, define_obj_type, dispatch, ns};

#[cfg(feature = "blocks")]
use crate::blocks;

define_obj_type!(
    #[doc(alias = "dispatch_data_t")]
    pub Data(dispatch::Object)
);

impl Data {
    /// The singleton dispatch data object representing a zero-length
    /// memory region.
    #[doc(alias = "dispatch_data_empty")]
    #[inline]
    pub fn empty() -> &'static Self {
        unsafe { &_dispatch_data_empty }
    }

    #[doc(alias = "dispatch_data_create_concat")]
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { dispatch_data_get_size(self) }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[cfg(feature = "blocks")]
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
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    static _dispatch_data_empty: Data;

    fn dispatch_data_get_size(data: &Data) -> usize;
    fn dispatch_data_create_subrange(data: &Data, offset: usize, length: usize) -> arc::R<Data>;
    fn dispatch_data_create_concat(data1: &Data, data2: &Data) -> arc::R<Data>;
    #[cfg(feature = "blocks")]
    fn dispatch_data_apply(data: &Data, appier: *mut c_void) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::dispatch;

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
}
