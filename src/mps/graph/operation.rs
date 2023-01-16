use crate::{
    cf, define_obj_type,
    mps::graph,
    msg_send, ns,
    objc::{msg_send, Obj},
};

define_obj_type!(Operation(ns::Id));

impl Operation {
    #[inline]
    pub fn input_tensors(&self) -> &cf::ArrayOf<graph::Tensor> {
        msg_send!("mpsg", self, sel_inputTensors)
    }

    #[inline]
    pub fn output_tensors(&self) -> &cf::ArrayOf<graph::Tensor> {
        msg_send!("mpsg", self, sel_outputTensors)
    }

    #[inline]
    pub fn control_deps(&self) -> &cf::ArrayOf<Self> {
        msg_send!("mpsg", self, sel_controlDependencies)
    }

    /// Graph on which the operation is defined
    #[inline]
    pub fn graph(&self) -> &graph::Graph {
        msg_send!("mpsg", self, sel_graph)
    }

    #[inline]
    pub fn name(&self) -> &cf::String {
        unsafe { self.call0(msg_send::name) }
    }
}
