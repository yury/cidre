use crate::{define_obj_type, mps, ns, objc};

define_obj_type!(pub TensorData(ns::Id));

impl TensorData {
    #[objc::msg_send(shape)]
    pub fn shape(&self) -> mps::Shape;

    #[objc::msg_send(dataType)]
    pub fn data_type(&self) -> mps::DataType;

    #[objc::msg_send(device)]
    pub fn device(&self) -> &mps::graph::Device;
}
