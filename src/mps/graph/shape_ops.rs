use crate::{
    arc,
    mps::{self, graph},
    ns,
};

impl graph::Graph {
    #[inline]
    pub fn broadcast(
        &self,
        tensor: &graph::Tensor,
        shape: &mps::Shape,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_broadcastTensor_toShape_name(self, tensor, shape, name) }
    }

    #[inline]
    pub fn concat(
        &self,
        tensors: &[&graph::Tensor],
        dimension: ns::Integer,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        let tensors = ns::Array::from_slice(tensors);
        self.concat_tensors(&tensors, dimension, name)
    }

    #[inline]
    pub fn concat_tensors(
        &self,
        tensors: &ns::Array<graph::Tensor>,
        dimension: ns::Integer,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_concatTensors_dimension_name(self, tensors, dimension, name) }
    }

    #[inline]
    pub fn expand_dims(
        &self,
        tensor: &graph::Tensor,
        axis: isize,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_expandDimsOfTensor_axis_name(self, tensor, axis, name) }
    }

    #[inline]
    pub fn expand_dims_axes(
        &self,
        tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_expandDimsOfTensor_axes_name(self, tensor, axes, name) }
    }

    #[inline]
    pub fn reshape(
        &self,
        tensor: &graph::Tensor,
        shape: &mps::Shape,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_reshapeTensor_withShape_name(self, tensor, shape, name) }
    }

    #[inline]
    pub fn transpose_with_dimension(
        &self,
        tensor: &graph::Tensor,
        dimention: ns::UInteger,
        with_dimention: ns::UInteger,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe {
            rsel_transposeTensor_dimension_withDimension_name(
                self,
                tensor,
                dimention,
                with_dimention,
                name,
            )
        }
    }

    #[inline]
    pub fn cast(
        &self,
        tensor: &graph::Tensor,
        to_type: mps::DataType,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_castTensor_toType_name(self, tensor, to_type, name) }
    }

    #[inline]
    pub fn slice_tensor(
        &self,
        tensor: &graph::Tensor,
        dimension: ns::UInteger,
        start: ns::Integer,
        length: ns::Integer,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe {
            rsel_sliceTensor_dimension_start_length_name(
                self, tensor, dimension, start, length, name,
            )
        }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_broadcastTensor_toShape_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        shape: &mps::Shape,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_concatTensors_dimension_name(
        graph: &graph::Graph,
        tensors: &ns::Array<graph::Tensor>,
        dimension: ns::Integer,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_expandDimsOfTensor_axis_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        axis: isize,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_expandDimsOfTensor_axes_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_reshapeTensor_withShape_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        shape: &mps::Shape,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_transposeTensor_dimension_withDimension_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        dimention: ns::UInteger,
        with_dimention: ns::UInteger,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_castTensor_toType_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        to_type: mps::DataType,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_sliceTensor_dimension_start_length_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        dimension: ns::UInteger,
        start: ns::Integer,
        length: ns::Integer,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

}
