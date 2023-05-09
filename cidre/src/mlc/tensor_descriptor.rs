use crate::{arc, define_cls, define_obj_type, mlc, ns, objc};

define_obj_type!(TensorDescriptor(ns::Id));
impl TensorDescriptor {
    define_cls!(MLC_TENSOR_DESCRIPTOR);

    /// The tensor data type.  The default is mlc::DataType::F32.
    #[objc::msg_send(dataType)]
    pub fn data_type(&self) -> mlc::DataType;

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
        data_type: mlc::DataType,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_shape_dt(
        shape: &ns::Array<ns::Number>,
        data_type: mlc::DataType,
    ) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(
        descriptorWithShape:
        sequenceLengths:sortedSequences:
        dataType:
    )]
    pub fn with_shape_seqs_dt_ar(
        shape: &ns::Array<ns::Number>,
        sequence_lens: &ns::Array<ns::Number>,
        sequence_sorted: bool,
        data_type: mlc::DataType,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_shape_seqs_dt(
        shape: &ns::Array<ns::Number>,
        sequence_lens: &ns::Array<ns::Number>,
        sequence_sorted: bool,
        data_type: mlc::DataType,
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
        data_type: mlc::DataType,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_size_feature_channels_batch_size_dt(
        width: usize,
        height: usize,
        feature_channel_count: usize,
        batch_size: usize,
        data_type: mlc::DataType,
    ) -> Option<arc::R<Self>>;
}

extern "C" {
    static MLC_TENSOR_DESCRIPTOR: &'static objc::Class<TensorDescriptor>;
}
