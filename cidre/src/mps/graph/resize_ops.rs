use crate::{arc, cf, mps, mps::graph};

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
    #[inline]
    pub fn resize(
        &self,
        tensor: &graph::Tensor,
        size: &mps::Shape,
        mode: graph::ResizeMode,
        center_result: bool,
        align_corners: bool,
        layout: graph::TensorNamedDataLayout,
        name: Option<&cf::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe {
            rsel_resizeTensor_size_mode_centerResult_alignCorners_layout_name(
                self,
                tensor,
                size,
                mode,
                center_result,
                align_corners,
                layout,
                name,
            )
        }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_resizeTensor_size_mode_centerResult_alignCorners_layout_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        size: &mps::Shape,
        mode: graph::ResizeMode,
        center_result: bool,
        align_corners: bool,
        layout: graph::TensorNamedDataLayout,
        name: Option<&cf::String>,
    ) -> arc::R<graph::Tensor>;
}
