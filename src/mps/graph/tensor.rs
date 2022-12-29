use crate::{define_obj_type, mps, mps::graph, msg_send, ns};

define_obj_type!(Tensor(ns::Id));

impl Tensor {
    #[inline]
    pub fn shape(&self) -> Option<&mps::Shape> {
        msg_send!("mpsg", self, sel_shape)
    }

    #[inline]
    pub fn data_type(&self) -> mps::DataType {
        msg_send!("mpsg", self, sel_dataType)
    }

    #[inline]
    pub fn operation(&self) -> &graph::Operation {
        msg_send!("mpsg", self, sel_operation)
    }
}
