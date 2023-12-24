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

    #[objc::cls_msg_send(maxTensorDimensions)]
    pub fn max_ndim() -> usize;

    #[objc::cls_msg_send(descriptorWithShape:dataType:)]
    pub fn with_shape_dt_ar(
        shape: &ns::Array<ns::Number>,
        data_type: mlc::DType,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_shape_dt(
        shape: &ns::Array<ns::Number>,
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>>;

    pub fn with_shape<const N: usize>(
        shape: [i32; N],
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>> {
        let mut sh: [*const ns::Number; N] = [std::ptr::null(); N];
        for (i, v) in shape.iter().enumerate() {
            sh[i] = ns::Number::tagged_i32(*v);
        }
        let shape = ns::Array::from_slice(unsafe { std::mem::transmute(&sh[..]) });

        Self::with_shape_dt(&shape, data_type)
    }

    #[objc::cls_msg_send(
        descriptorWithShape:
        sequenceLengths:sortedSequences:
        dataType:
    )]
    pub fn with_shape_seqs_dt_ar(
        shape: &ns::Array<ns::Number>,
        sequence_lens: &ns::Array<ns::Number>,
        sequence_sorted: bool,
        data_type: mlc::DType,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_shape_seqs_dt(
        shape: &ns::Array<ns::Number>,
        sequence_lens: &ns::Array<ns::Number>,
        sequence_sorted: bool,
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(descriptorWithWidth:height:featureChannelCount:batchSize:)]
    pub fn with_size_feature_channels_batch_size_ar(
        width: usize,
        height: usize,
        feature_channel_count: usize,
        batch_size: usize,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_size_feature_channels_batch_size(
        width: usize,
        height: usize,
        feature_channel_count: usize,
        batch_size: usize,
    ) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(descriptorWithWidth:height:featureChannelCount:batchSize:dataType:)]
    pub fn with_size_feature_channels_batch_size_dt_ar(
        width: usize,
        height: usize,
        feature_channel_count: usize,
        batch_size: usize,
        data_type: mlc::DType,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_size_feature_channels_batch_size_dt(
        width: usize,
        height: usize,
        feature_channel_count: usize,
        batch_size: usize,
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>>;

    /// This method is provided as an easy to use API to create a weight tensor.
    #[objc::cls_msg_send(
        convolutionWeightsDescriptorWithWidth:height:
        inputFeatureChannelCount:outputFeatureChannelCount:
        dataType:
    )]
    pub fn conv_weights_with_size_features_dt_ar(
        width: usize,
        height: usize,
        input_feature_channel_count: usize,
        output_feature_channel_count: usize,
        data_type: mlc::DType,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn conv_weights_with_size_features_dt(
        width: usize,
        height: usize,
        input_feature_channel_count: usize,
        output_feature_channel_count: usize,
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(
        convolutionWeightsDescriptorWithInputFeatureChannelCount:outputFeatureChannelCount:
        dataType:
    )]
    pub fn conv_weights_with_features_dt_ar(
        input_feature_channel_count: usize,
        output_feature_channel_count: usize,
        data_type: mlc::DType,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn conv_weights_with_features_dt(
        input_feature_channel_count: usize,
        output_feature_channel_count: usize,
        data_type: mlc::DType,
    ) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(convolutionBiasesDescriptorWithFeatureChannelCount:dataType:)]
    pub fn conv_biases_with_feature_channel_count_dt_ar(
        feature_channle_count: usize,
        data_type: mlc::DType,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
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
