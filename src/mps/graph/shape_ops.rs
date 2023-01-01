use crate::{
    cf,
    mps::{self, graph},
};

impl graph::Graph {
    pub fn expand_dims(
        &self,
        tensor: &graph::Tensor,
        axis: isize,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor> {
        unsafe { rsel_expandDimsOfTensor_axis_name(self, tensor, axis, name) }
    }

    pub fn reshape(
        &self,
        tensor: &graph::Tensor,
        shape: &mps::Shape,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor> {
        unsafe { rsel_reshapeTensor_withShape_name(self, tensor, shape, name) }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_expandDimsOfTensor_axis_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        axis: isize,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor>;

    fn rsel_reshapeTensor_withShape_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        shape: &mps::Shape,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor>;
}
