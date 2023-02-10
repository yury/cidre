use crate::{arc, mps, mps::graph, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum ResizeMode {
    Nearest = 0,
    Bilinear = 1,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(usize)]
pub enum ResizeNearestRoundingMode {
    RoundPreferCeil = 0,
    RoundPreferFloor = 1,
    Ceil = 2,
    Floor = 3,
}

impl graph::Graph {
    #[objc::msg_send(resizeTensor:size:mode:centerResult:alignCorners:layout:name:)]
    pub fn resize_ar(
        &self,
        tensor: &graph::Tensor,
        size: &mps::Shape,
        mode: graph::ResizeMode,
        center_result: bool,
        align_corners: bool,
        layout: graph::TensorNamedDataLayout,
        name: Option<&ns::String>,
    ) -> &'ar graph::Tensor;

    #[objc::rar_retain]
    pub fn resize(
        &self,
        tensor: &graph::Tensor,
        size: &mps::Shape,
        mode: graph::ResizeMode,
        center_result: bool,
        align_corners: bool,
        layout: graph::TensorNamedDataLayout,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;
}
