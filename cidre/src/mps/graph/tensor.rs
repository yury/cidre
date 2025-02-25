use crate::{define_obj_type, mps, mps::graph, ns, objc};

define_obj_type!(pub Tensor(ns::Id));
impl Tensor {
    #[objc::msg_send(shape)]
    pub fn shape(&self) -> Option<&mps::Shape>;

    #[objc::msg_send(dataType)]
    pub fn data_type(&self) -> mps::DType;

    #[objc::msg_send(operation)]
    pub fn op(&self) -> &graph::Op;
}
