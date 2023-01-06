use crate::{arc, mps::graph, ns};

impl graph::Graph {
    #[inline]
    pub fn mat_mul(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe {
            rsel_matrixMultiplicationWithPrimaryTensor_secondaryTensor_name(
                self, primary, secondary, name,
            )
        }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_matrixMultiplicationWithPrimaryTensor_secondaryTensor_name(
        graph: &graph::Graph,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

}
