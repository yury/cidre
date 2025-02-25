use crate::{arc, define_cls, define_obj_type, mps::graph, ns, objc};

define_obj_type!(
    #[doc(alias = "MPSGraphConvolution2dDescriptor")]
    pub Conv2dOpDesc(ns::Id)
);

impl Conv2dOpDesc {
    define_cls!(MPS_GRAPH_CONVOLUTION_2D_OP_DESCRIPTOR);

    #[objc::msg_send(descriptorWithStrideInX:strideInY:dilationRateInX:dilationRateInY:groups:paddingLeft:paddingRight:paddingTop:paddingBottom:paddingStyle:dataLayout:weightsLayout:)]
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
    pub fn conv_2d(
        &self,
        source: &graph::Tensor,
        weights: &graph::Tensor,
        descriptor: &Conv2dOpDesc,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;
}

#[link(name = "mpsg", kind = "static")]
unsafe extern "C" {
    static MPS_GRAPH_CONVOLUTION_2D_OP_DESCRIPTOR: &'static objc::Class<Conv2dOpDesc>;
}
