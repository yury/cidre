use crate::{define_obj_type, mlc, ns, objc};

define_obj_type!(Tensor(ns::Id));
impl Tensor {
    /// A unique number to identify each tensor.  Assigned when the tensor is created.
    #[objc::msg_send(tensorID)]
    pub fn tensor_id(&self) -> usize;

    #[objc::msg_send(descriptor)]
    pub fn descriptor(&self) -> &mlc::TensorDescriptor;

    #[objc::msg_send(data)]
    pub fn data(&self) -> Option<&ns::Data>;

    #[objc::msg_send(label)]
    pub fn label(&self) -> &ns::String;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, value: &ns::String);

    #[objc::msg_send(device)]
    pub fn device(&self) -> Option<&mlc::Device>;
}
