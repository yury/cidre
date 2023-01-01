use crate::{cf, mps::graph};

impl graph::Graph {
    pub fn addition(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor> {
        unsafe {
            rsel_additionWithPrimaryTensor_secondaryTensor_name(self, primary, secondary, name)
        }
    }

    pub fn multiplication(
        &self,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor> {
        unsafe {
            rsel_multiplicationWithPrimaryTensor_secondaryTensor_name(
                self, primary, secondary, name,
            )
        }
    }
}

extern "C" {
    fn rsel_additionWithPrimaryTensor_secondaryTensor_name(
        graph: &graph::Graph,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor>;

    fn rsel_multiplicationWithPrimaryTensor_secondaryTensor_name(
        graph: &graph::Graph,
        primary: &graph::Tensor,
        secondary: &graph::Tensor,
        name: Option<&cf::String>,
    ) -> cf::Retained<graph::Tensor>;

    //-(MPSGraphTensor *) multiplicationWithPrimaryTensor:(MPSGraphTensor *) primaryTensor
    //                                    secondaryTensor:(MPSGraphTensor *) secondaryTensor
    //                                               name:(NSString * _Nullable) name
    //MPS_SWIFT_NAME( multiplication(_:_:name:) );

}
