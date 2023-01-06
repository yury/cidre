use crate::{arc, mps::graph, ns};

impl graph::Graph {
    #[inline]
    pub fn mean(
        &self,
        tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_meanOfTensor_axes_name(self, tensor, axes, name) }
    }

    #[inline]
    pub fn variance(
        &self,
        tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_varianceOfTensor_axes_name(self, tensor, axes, name) }
    }

    #[inline]
    pub fn variance_mean(
        &self,
        tensor: &graph::Tensor,
        mean: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_varianceOfTensor_meanTensor_axes_name(self, tensor, mean, axes, name) }
    }

    #[inline]
    pub fn normalize(
        &self,
        tensor: &graph::Tensor,
        mean: &graph::Tensor,
        variance: &graph::Tensor,
        gamma: Option<&graph::Tensor>,
        beta: Option<&graph::Tensor>,
        epsilon: f32,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe {
            rsel_normalizationWithTensor_meanTensor_varianceTensor_gammaTensor_betaTensor_epsilon_name(
                self, tensor, mean, variance, gamma, beta, epsilon, name
            )
        }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_meanOfTensor_axes_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_varianceOfTensor_axes_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_varianceOfTensor_meanTensor_axes_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        mean_tensor: &graph::Tensor,
        axes: &ns::Array<ns::Number>,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_normalizationWithTensor_meanTensor_varianceTensor_gammaTensor_betaTensor_epsilon_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        mean_tensor: &graph::Tensor,
        variance_tensor: &graph::Tensor,
        gamma_tensor: Option<&graph::Tensor>,
        beta_tensor: Option<&graph::Tensor>,
        epsilon: f32,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    //-(MPSGraphTensor *) normalizationWithTensor:(MPSGraphTensor *) tensor
    //                                 meanTensor:(MPSGraphTensor *) mean
    //                             varianceTensor:(MPSGraphTensor *) variance
    //                                gammaTensor:(MPSGraphTensor * _Nullable) gamma
    //                                 betaTensor:(MPSGraphTensor * _Nullable) beta
    //                                    epsilon:(float) epsilon
    //                                       name:(NSString * _Nullable) name
    //MPS_SWIFT_NAME( normalize(_:mean:variance:gamma:beta:epsilon:name:) );

}
