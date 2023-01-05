use crate::{arc, define_obj_type, mps, ns};

define_obj_type!(TensorData(ns::Id));

impl TensorData {
    pub fn shape(&self) -> mps::Shape {
        todo!()
    }

    pub fn data_type(&self) -> mps::DataType {
        todo!()
    }

    pub fn device(&self) -> &mps::graph::Device {
        todo!()
    }

    pub fn nd_array(&self) -> Option<arc::R<mps::NDArray>> {
        todo!()
    }
}
