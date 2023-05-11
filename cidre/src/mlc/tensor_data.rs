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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_TENSOR_DATA: &'static objc::Class<TensorData>;
}
