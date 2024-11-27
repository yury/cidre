use crate::{arc, define_obj_type, mps::graph, ns, objc};

define_obj_type!(
    #[doc(alias = "MPSGraphOperation")]
    pub Op(ns::Id)
);

impl Op {
    #[objc::msg_send(inputTensors)]
    pub fn input_tensors(&self) -> arc::R<ns::Array<graph::Tensor>>;

    #[objc::msg_send(outputTensors)]
    pub fn output_tensors(&self) -> arc::R<ns::Array<graph::Tensor>>;

    #[objc::msg_send(controlDependencies)]
    pub fn control_deps(&self) -> arc::R<ns::Array<Self>>;

    /// Graph on which the operation is defined
    #[objc::msg_send(graph)]
    pub fn graph(&self) -> arc::R<graph::Graph>;

    #[objc::msg_send(name)]
    pub fn name(&self) -> arc::R<ns::String>;
}
