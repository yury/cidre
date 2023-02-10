use crate::{arc, mps::graph, ns, objc};

impl graph::Graph {
    #[objc::msg_send(matrixMultiplicationWithPrimaryTensor:secondaryTensor:name:)]
    pub fn mat_mul_ar(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> &'ar graph::Tensor;

    #[objc::rar_retain]
    pub fn mat_mul(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;
}
