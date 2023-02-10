use crate::{arc, define_cls, define_obj_type, mps::graph, ns, objc};

define_obj_type!(Convolution2DOpDescriptor(ns::Id));

impl Convolution2DOpDescriptor {
    define_cls!(MPS_GRAPH_CONVOLUTION_2D_OP_DESCRIPTOR);

    #[objc::cls_msg_send(descriptorWithStrideInX:strideInY:dilationRateInX:dilationRateInY:groups:paddingLeft:paddingRight:paddingTop:paddingBottom:paddingStyle:dataLayout:weightsLayout:)]
    pub fn with_ar(
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
    ) -> Option<&'ar Self>;

    #[objc::cls_rar_retain]
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
    ) -> Option<arc::R<Self>>;
}

impl graph::Graph {
    #[objc::msg_send(convolution2DWithSourceTensor:weightsTensor:descriptor:name:)]
    pub fn convolution_2d_ar(
        &self,
        source: &graph::Tensor,
        weights: &graph::Tensor,
        descriptor: &Convolution2DOpDescriptor,
        name: Option<&ns::String>,
    ) -> &'ar graph::Tensor;

    #[objc::rar_retain]
    pub fn convolution_2d(
        &self,
        source: &graph::Tensor,
        weights: &graph::Tensor,
        descriptor: &Convolution2DOpDescriptor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    static MPS_GRAPH_CONVOLUTION_2D_OP_DESCRIPTOR: &'static objc::Class<Convolution2DOpDescriptor>;
}
