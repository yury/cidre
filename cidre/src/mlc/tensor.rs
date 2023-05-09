use crate::{define_obj_type, ns, objc};

define_obj_type!(Tensor(ns::Id));
impl Tensor {
    /// A unique number to identify each tensor.  Assigned when the tensor is created.
    #[objc::msg_send(tensorID)]
    pub fn tensor_id(&self) -> usize;
}
