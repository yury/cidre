use crate::{arc, mps::graph, ns};

impl graph::Graph {
    #[inline]
    pub fn add(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe {
            rsel_additionWithPrimaryTensor_secondaryTensor_name(self, primary, secondary, name)
        }
    }

    #[inline]
    pub fn clamp(
        &self,
        tensor: &graph::Tensor,
        min: &graph::Tensor,
        max: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe {
            rsel_clampWithTensor_minValueTensor_maxValueTensor_name(self, tensor, min, max, name)
        }
    }

    /// Create Multiply op and return the result tensor, it supports broadcasting as well
    ///
    /// resultTensor = primary * secondary
    #[inline]
    pub fn mul(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe {
            rsel_multiplicationWithPrimaryTensor_secondaryTensor_name(
                self, primary, secondary, name,
            )
        }
    }

    #[inline]
    pub fn round(
        &self,
        tensor: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_roundWithTensor_name(self, tensor, name) }
    }

    #[inline]
    pub fn square_root(
        &self,
        tensor: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_squareRootWithTensor_name(self, tensor, name) }
    }

    #[inline]
    pub fn subtraction(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe {
            rsel_subtractionWithPrimaryTensor_secondaryTensor_name(self, primary, secondary, name)
        }
    }

    #[inline]
    pub fn tanh(&self, tensor: &graph::Tensor, name: Option<&ns::String>) -> arc::R<graph::Tensor> {
        unsafe { rsel_tanhWithTensor_name(self, tensor, name) }
    }
    #[inline]
    pub fn erf(&self, tensor: &graph::Tensor, name: Option<&ns::String>) -> arc::R<graph::Tensor> {
        unsafe { rsel_erfWithTensor_name(self, tensor, name) }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_additionWithPrimaryTensor_secondaryTensor_name(
        graph: &graph::Graph,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_multiplicationWithPrimaryTensor_secondaryTensor_name(
        graph: &graph::Graph,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_clampWithTensor_minValueTensor_maxValueTensor_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        min: &graph::Tensor,
        max: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_roundWithTensor_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_squareRootWithTensor_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_subtractionWithPrimaryTensor_secondaryTensor_name(
        graph: &graph::Graph,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_tanhWithTensor_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_erfWithTensor_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

}
