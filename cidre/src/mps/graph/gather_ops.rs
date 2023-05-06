use crate::{arc, mps::graph, ns, objc};

impl graph::Graph {
    #[objc::msg_send(gatherWithUpdatesTensor:indicesTensor:axis:batchDimensions:name:)]
    pub fn gather_ar(
        &self,
        updates: &graph::Tensor,
        indices: &graph::Tensor,
        axis: ns::UInteger,
        batch_dimensions: ns::UInteger,
        name: Option<&ns::String>,
    ) -> arc::Rar<graph::Tensor>;

    #[objc::rar_retain]
    pub fn gather(
        &self,
        updates: &graph::Tensor,
        indices: &graph::Tensor,
        axis: ns::UInteger,
        batch_dimensions: ns::UInteger,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(gatherAlongAxis:withUpdatesTensor:indicesTensor:name:)]
    pub fn gather_along_axis_ar(
        &self,
        axis: ns::Integer,
        updates: &graph::Tensor,
        indices: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::Rar<graph::Tensor>;

    #[objc::rar_retain]
    pub fn gather_along_axis(
        &self,
        axis: ns::Integer,
        updates: &graph::Tensor,
        indices: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;
}
