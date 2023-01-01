use crate::{cf, mps::graph};

impl graph::Graph {
    pub fn addition(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor> {
        unsafe {
            rsel_additionWithPrimaryTensor_secondaryTensor_name(self, primary, secondary, name)
        }
    }
}

extern "C" {
    fn rsel_additionWithPrimaryTensor_secondaryTensor_name(
        graph: &graph::Graph,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor>;
}
