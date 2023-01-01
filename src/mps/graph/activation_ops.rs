use crate::{cf, mps::graph};

impl graph::Graph {
    #[inline]
    pub fn sigmoid(
        &self,
        tensor: &graph::Tensor,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor> {
        unsafe { rsel_sigmoidWithTensor_name(self, tensor, name) }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_sigmoidWithTensor_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor>;
}
