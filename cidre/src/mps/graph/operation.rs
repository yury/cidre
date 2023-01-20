use crate::{define_obj_type, mps::graph, ns, objc};

define_obj_type!(Operation(ns::Id));

impl Operation {
    #[objc::msg_send2(inputTensors)]
    pub fn input_tensors(&self) -> &ns::Array<graph::Tensor>;

    #[objc::msg_send2(outputTensors)]
    pub fn output_tensors(&self) -> &ns::Array<graph::Tensor>;

    #[objc::msg_send2(controlDependencies)]
    pub fn control_deps(&self) -> &ns::Array<Self>;

    /// Graph on which the operation is defined
    #[objc::msg_send2(graph)]
    pub fn graph(&self) -> &graph::Graph;

    #[objc::msg_send2(name)]
    pub fn name(&self) -> &ns::String;
}
