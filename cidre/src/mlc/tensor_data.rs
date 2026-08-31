use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "MLCTensorData")]
    pub TensorData(ns::Id)
);

impl TensorData {
    define_cls!(MLC_TENSOR_DATA);

    #[objc::msg_send(bytes)]
    pub fn bytes(&self) -> *const u8;

    #[objc::msg_send(bytes)]
    pub fn bytes_mut(&mut self) -> *mut u8;

    #[objc::msg_send(length)]
    pub fn len(&self) -> usize;

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// # Safety
    ///
    /// `bytes` must point to `length` initialized, readable bytes that remain
    /// alive and unchanged until the returned tensor data is dropped.
    #[objc::msg_send(dataWithImmutableBytesNoCopy:length:)]
    pub unsafe fn with_bytes_no_copy(bytes: *const u8, length: usize) -> arc::R<Self>;

    /// # Safety
    ///
    /// Every byte in `slice`, including padding, must be initialized. The slice
    /// must remain alive and unchanged until the returned tensor data is
    /// dropped.
    #[inline]
    pub unsafe fn with_slice_no_copy<T: Sized>(slice: &[T]) -> arc::R<Self> {
        unsafe { Self::with_bytes_no_copy(slice.as_ptr() as _, std::mem::size_of_val(slice)) }
    }
}

unsafe extern "C" {
    static MLC_TENSOR_DATA: &'static objc::Class<TensorData>;
}
