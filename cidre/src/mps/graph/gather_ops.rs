use crate::{arc, mps::graph, ns};

impl graph::Graph {
    #[inline]
    pub fn gather(
        &self,
        updates: &graph::Tensor,
        indices: &graph::Tensor,
        axis: ns::UInteger,
        batch_dimensions: ns::UInteger,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe {
            rsel_gatherWithUpdatesTensor_indicesTensor_axis_batchDimensions_name(
                self,
                updates,
                indices,
                axis,
                batch_dimensions,
                name,
            )
        }
    }

    #[inline]
    pub fn gather_along_axis(
        &self,
        axis: ns::Integer,
        updates: &graph::Tensor,
        indices: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe {
            rsel_gatherAlongAxis_withUpdatesTensor_indicesTensor_name(
                self, axis, updates, indices, name,
            )
        }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_gatherWithUpdatesTensor_indicesTensor_axis_batchDimensions_name(
        graph: &graph::Graph,
        updates: &graph::Tensor,
        indices: &graph::Tensor,
        axis: ns::UInteger,
        batch_dimensions: ns::UInteger,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_gatherAlongAxis_withUpdatesTensor_indicesTensor_name(
        graph: &graph::Graph,
        axis: ns::Integer,
        updates: &graph::Tensor,
        indices: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

}
