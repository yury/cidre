use crate::{arc, define_cls, define_obj_type, mlc, ns, objc};

define_obj_type!(pub TensorDesc(ns::Id));
impl TensorDesc {
    define_cls!(MLC_TENSOR_DESCRIPTOR);

    /// The tensor data type.  The default is mlc::DataType::F32.
    #[objc::msg_send(dataType)]
    pub fn dtype(&self) -> mlc::DType;

    /// The number of dimensions in the tensor
    #[objc::msg_send(dimensionCount)]
    pub fn ndim(&self) -> usize;

    /// The size in each dimension
    #[objc::msg_send(shape)]
    pub fn shape(&self) -> &ns::Array<ns::Number>;

    #[objc::msg_send(stride)]
    pub fn stride(&self) -> &ns::Array<ns::Number>;

    /// The allocation size in bytes for a tensor.
    #[objc::msg_send(tensorAllocationSizeInBytes)]
    pub fn tensor_alloc_size(&self) -> usize;

    #[objc::msg_send(sequenceLengths)]
    pub fn seq_lens(&self) -> &ns::Array<ns::Number>;

    /// Specifies whether the sequences are sorted or not.
    #[objc::msg_send(sortedSequences)]
    pub fn sorted_seqs(&self) -> bool;

    #[objc::msg_send(batchSizePerSequenceStep)]
    pub fn batch_size_per_seq_step(&self) -> Option<&ns::Array<ns::Number>>;

    #[objc::msg_send2(maxTensorDimensions)]
    pub fn max_ndim() -> usize;

    #[objc::msg_send2(descriptorWithShape:dataType:)]
    pub fn with_shape_dt(
        shape: &ns::Array<ns::Number>,
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>>;

    pub fn with_shape<const N: usize>(
        shape: [i32; N],
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>> {
        let shape: arc::R<ns::Array<ns::Number>> = shape.into();

        Self::with_shape_dt(&shape, data_type)
    }

    #[objc::msg_send2(
        descriptorWithShape:
        sequenceLengths:sortedSequences:
        dataType:
    )]
    pub fn with_shape_seqs_dt(
        shape: &ns::Array<ns::Number>,
        sequence_lens: &ns::Array<ns::Number>,
        sequence_sorted: bool,
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>>;

    #[objc::msg_send2(descriptorWithWidth:height:featureChannelCount:batchSize:)]
    pub fn with_size_feature_channels_batch_size(
        width: usize,
        height: usize,
        feature_channel_count: usize,
        batch_size: usize,
    ) -> Option<arc::R<Self>>;

    #[objc::msg_send2(descriptorWithWidth:height:featureChannelCount:batchSize:dataType:)]
    pub fn with_size_feature_channels_batch_size_dt(
        width: usize,
        height: usize,
        feature_channel_count: usize,
        batch_size: usize,
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>>;

    /// This method is provided as an easy to use API to create a weight tensor.
    #[objc::msg_send2(
        convolutionWeightsDescriptorWithWidth:height:
        inputFeatureChannelCount:outputFeatureChannelCount:
        dataType:
    )]
    pub fn conv_weights_with_size_features_dt(
        width: usize,
        height: usize,
        input_feature_channel_count: usize,
        output_feature_channel_count: usize,
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>>;

    #[objc::msg_send2(
        convolutionWeightsDescriptorWithInputFeatureChannelCount:outputFeatureChannelCount:
        dataType:
    )]
    pub fn conv_weights_with_features_dt(
        input_feature_channel_count: usize,
        output_feature_channel_count: usize,
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>>;

    #[objc::msg_send2(convolutionBiasesDescriptorWithFeatureChannelCount:dataType:)]
    pub fn conv_biases_with_feature_channel_count_dt(
        feature_channle_count: usize,
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>>;
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_TENSOR_DESCRIPTOR: &'static objc::Class<TensorDesc>;
}

#[cfg(test)]
mod tests {
    use crate::mlc;

    #[test]
    fn basics() {
        let desc = mlc::TensorDesc::with_shape([-1, 2, 3, 4], mlc::DType::F16).unwrap();
        assert_eq!(4, desc.ndim());
        assert_eq!(4, mlc::TensorDesc::max_ndim());
        println!("shape {:?}", desc.shape());
        println!("max {}", mlc::TensorDesc::max_ndim());
    }
}
