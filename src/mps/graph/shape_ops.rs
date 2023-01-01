use crate::{cf, mps::graph};

impl graph::Graph {
    pub fn expand_dims(
        &self,
        tensor: &graph::Tensor,
        axis: isize,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor> {
        unsafe { rsel_expandDimsOfTensor_axis_name(self, axis, name) }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_expandDimsOfTensor_axis_name(
        graph: &graph::Graph,
        axis: isize,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor>;
}
