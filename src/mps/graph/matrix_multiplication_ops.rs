use crate::{arc, cf, mps::graph};

impl graph::Graph {
    #[inline]
    pub fn matrix_multiplication(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&cf::String>,
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
        name: Option<&cf::String>,
    ) -> arc::R<graph::Tensor>;

}
