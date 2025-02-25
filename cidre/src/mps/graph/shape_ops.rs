use crate::{
    arc,
    mps::{self, graph},
    ns, objc,
};

impl graph::Graph {
    #[objc::msg_send(broadcastTensor:toShape:name:)]
    pub fn broadcast(
        &self,
        tensor: &graph::Tensor,
        shape: &mps::Shape,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    pub fn concat(
        &self,
        tensors: &[&graph::Tensor],
        dimension: ns::Integer,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        let tensors = ns::Array::from_slice(tensors);
        self.concat_tensors(&tensors, dimension, name)
    }

    #[objc::msg_send(concatTensors:dimension:name:)]
    pub fn concat_tensors(
        &self,
        tensors: &ns::Array<graph::Tensor>,
        dimension: ns::Integer,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(expandDimsOfTensor:axis:name:)]
    pub fn expand_dims(
        &self,
        tensor: &graph::Tensor,
        axis: isize,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(expandDimsOfTensor:axes:name:)]
    pub fn expand_dims_axes(
        &self,
        tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(reshapeTensor:withShape:name:)]
    pub fn reshape(
        &self,
        tensor: &graph::Tensor,
        shape: &mps::Shape,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(transposeTensor:dimension:withDimension:name:)]
    pub fn transpose_with_dimension(
        &self,
        tensor: &graph::Tensor,
        dimention: ns::UInteger,
        with_dimention: ns::UInteger,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(castTensor:toType:name:)]
    pub fn cast(
        &self,
        tensor: &graph::Tensor,
        to_type: mps::DType,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(sliceTensor:dimension:start:length:name:)]
    pub fn slice_tensor(
        &self,
        tensor: &graph::Tensor,
        dimension: ns::UInteger,
        start: ns::Integer,
        length: ns::Integer,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;
}
