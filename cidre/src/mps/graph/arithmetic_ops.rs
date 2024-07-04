use crate::{arc, mps::graph, ns, objc};

impl graph::Graph {
    #[objc::msg_send(additionWithPrimaryTensor:secondaryTensor:name:)]
    pub fn add(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(clampWithTensor:minValueTensor:maxValueTensor:name:)]
    pub fn clamp(
        &self,
        tensor: &graph::Tensor,
        min: &graph::Tensor,
        max: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    /// Create Multiply op and return the result tensor, it supports broadcasting as well
    ///
    /// resultTensor = primary * secondary
    #[objc::msg_send(multiplicationWithPrimaryTensor:secondaryTensor:name:)]
    pub fn mul(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(roundWithTensor:name:)]
    pub fn round(&self, tensor: &graph::Tensor, name: Option<&ns::String>)
        -> arc::R<graph::Tensor>;

    #[objc::msg_send(squareRootWithTensor:name:)]
    pub fn square_root(
        &self,
        tensor: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(subtractionWithPrimaryTensor:secondaryTensor:name:)]
    pub fn sub(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(divisionWithPrimaryTensor:secondaryTensor:name:)]
    pub fn div(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    #[objc::msg_send(tanhWithTensor:name:)]
    pub fn tanh(&self, tensor: &graph::Tensor, name: Option<&ns::String>) -> arc::R<graph::Tensor>;

    #[objc::msg_send(erfWithTensor:name:)]
    pub fn erf(&self, tensor: &graph::Tensor, name: Option<&ns::String>) -> arc::R<graph::Tensor>;

    #[objc::msg_send(cosWithTensor:name:)]
    pub fn cos(&self, tensor: &graph::Tensor, name: Option<&ns::String>) -> arc::R<graph::Tensor>;

    #[objc::msg_send(sinWithTensor:name:)]
    pub fn sin(&self, tensor: &graph::Tensor, name: Option<&ns::String>) -> arc::R<graph::Tensor>;
}
