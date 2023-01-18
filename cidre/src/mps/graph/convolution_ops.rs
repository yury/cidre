use crate::{arc, define_obj_type, mps::graph, ns};

define_obj_type!(Convolution2DOpDescriptor(ns::Id));

impl Convolution2DOpDescriptor {
    pub fn with(
        stride_in_x: usize,
        stride_in_y: usize,
        dilation_rate_in_x: usize,
        dilation_rate_in_y: usize,
        groups: usize,
        padding_left: usize,
        padding_right: usize,
        padding_top: usize,
        padding_bottom: usize,
        padding_style: graph::PaddingStyle,
        data_layout: graph::TensorNamedDataLayout,
        weight_layout: graph::TensorNamedDataLayout,
    ) -> Option<arc::R<Self>> {
        unsafe {
            MPSGraphConvolution2DOpDescriptor_descriptorWithStrideInX_strideInY_dilationRateInX_dilationRateInY_groups_paddingLeft_paddingRight_paddingTop_paddingBottom_paddingStyle_dataLayout_weightsLayout(
                stride_in_x,
                stride_in_y,
                dilation_rate_in_x,
                dilation_rate_in_y,
                groups,
                padding_left,
                padding_right,
                padding_top,
                padding_bottom,
                padding_style,
                data_layout,
                weight_layout
            )
        }
    }
}

impl graph::Graph {
    pub fn convolution_2d(
        &self,
        source: &graph::Tensor,
        weights: &graph::Tensor,
        descriptor: &Convolution2DOpDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_convolution2DWithSourceTensor(self, source, weights, descriptor, name) }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn MPSGraphConvolution2DOpDescriptor_descriptorWithStrideInX_strideInY_dilationRateInX_dilationRateInY_groups_paddingLeft_paddingRight_paddingTop_paddingBottom_paddingStyle_dataLayout_weightsLayout(
        stride_in_x: usize,
        stride_in_y: usize,
        dilation_rate_in_x: usize,
        dilation_rate_in_y: usize,
        groups: usize,
        padding_left: usize,
        padding_right: usize,
        padding_top: usize,
        padding_bottom: usize,
        padding_style: graph::PaddingStyle,
        data_layout: graph::TensorNamedDataLayout,
        weight_layout: graph::TensorNamedDataLayout,
    ) -> Option<arc::R<Convolution2DOpDescriptor>>;

    fn rsel_convolution2DWithSourceTensor(
        graph: &graph::Graph,
        source: &graph::Tensor,
        weights: &graph::Tensor,
        descriptor: &Convolution2DOpDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;
}
