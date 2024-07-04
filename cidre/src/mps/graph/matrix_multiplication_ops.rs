use crate::{arc, mps::graph, ns, objc};

impl graph::Graph {
    #[objc::msg_send(matrixMultiplicationWithPrimaryTensor:secondaryTensor:name:)]
    pub fn mat_mul(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;
}
