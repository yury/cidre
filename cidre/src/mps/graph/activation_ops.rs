use crate::{arc, mps::graph, ns, objc};

impl graph::Graph {
    #[objc::msg_send(reLUWithTensor:name:)]
    pub fn relu(&self, tensor: &graph::Tensor, name: Option<&ns::String>) -> arc::R<graph::Tensor>;

    #[objc::msg_send(sigmoidWithTensor:name:)]
    pub fn sigmoid(
        &self,
        tensor: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(softMaxWithTensor:axis:name:)]
    pub fn soft_max(
        &self,
        tensor: &graph::Tensor,
        axis: ns::Integer,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;
}
