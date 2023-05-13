use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(TensorData(ns::Id));
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

    #[objc::cls_msg_send(dataWithImmutableBytesNoCopy:length:)]
    pub fn with_bytes_no_copy_ar(bytes: *const u8, length: usize) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_bytes_no_copy(bytes: *const u8, length: usize) -> arc::R<Self>;

    pub fn with_slice_no_copy_ar(slice: &[u8]) -> arc::Rar<Self> {
        Self::with_bytes_no_copy_ar(slice.as_ptr(), slice.len())
    }

    #[objc::cls_rar_retain]
    pub fn with_slice_no_copy(slice: &[u8]) -> arc::R<Self>;
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_TENSOR_DATA: &'static objc::Class<TensorData>;
}
