use crate::{arc, define_cls, define_obj_type, mlc, ns, objc};

define_obj_type!(pub TensorParameter(ns::Id));
impl TensorParameter {
    define_cls!(MLC_TENSOR_PARAMETER);

    #[objc::msg_send(tensor)]
    pub fn tensor(&self) -> &mlc::Tensor;

    #[objc::msg_send(isUpdatable)]
    pub fn is_updatable(&self) -> bool;

    #[objc::msg_send(setUpdatable:)]
    pub fn set_updatable(&mut self, val: bool);

    #[objc::msg_send(parameterWithTensor:)]
    pub fn with_tensor(tensor: &mlc::Tensor) -> arc::R<Self>;

    #[objc::msg_send(parameterWithTensor:optimizerData:)]
    pub fn with_tensor_optimizer_data(
        tensor: &mlc::Tensor,
        optimizer_data: Option<ns::Array<mlc::TensorData>>,
    ) -> arc::R<Self>;
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_TENSOR_PARAMETER: &'static objc::Class<TensorParameter>;
}
