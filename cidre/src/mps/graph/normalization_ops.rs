use crate::{arc, mps::graph, ns, objc};

impl graph::Graph {
    #[objc::msg_send(meanOfTensor:axes:name:)]
    pub fn mean_ar(
        &self,
        tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::Rar<graph::Tensor>;

    #[objc::rar_retain]
    pub fn mean(
        &self,
        tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(varianceOfTensor:axes:name:)]
    pub fn variance_ar(
        &self,
        tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::Rar<graph::Tensor>;

    #[objc::rar_retain]
    pub fn variance(
        &self,
        tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(varianceOfTensor:meanTensor:axes:name:)]
    pub fn variance_mean_ar(
        &self,
        tensor: &graph::Tensor,
        mean: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::Rar<graph::Tensor>;

    #[objc::rar_retain]
    pub fn variance_mean(
        &self,
        tensor: &graph::Tensor,
        mean: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(normalizationWithTensor:meanTensor:varianceTensor:gammaTensor:betaTensor:epsilon:name:)]
    pub fn normalize_ar(
        &self,
        tensor: &graph::Tensor,
        mean: &graph::Tensor,
        variance: &graph::Tensor,
        gamma: Option<&graph::Tensor>,
        beta: Option<&graph::Tensor>,
        epsilon: f32,
        name: Option<&ns::String>,
    ) -> arc::Rar<graph::Tensor>;

    #[objc::rar_retain]
    pub fn normalize(
        &self,
        tensor: &graph::Tensor,
        mean: &graph::Tensor,
        variance: &graph::Tensor,
        gamma: Option<&graph::Tensor>,
        beta: Option<&graph::Tensor>,
        epsilon: f32,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;
}
