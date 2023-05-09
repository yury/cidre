use crate::{arc, define_cls, define_obj_type, mlc, objc};

define_obj_type!(BatchNormalizationLayer(mlc::Layer));

impl BatchNormalizationLayer {
    define_cls!(MLC_BATCH_NORMALIZATION_LAYER);

    /// The number of feature channels
    #[objc::msg_send(featureChannelCount)]
    pub fn feature_channel_count(&self) -> usize;

    // @property (readonly, nonatomic, retain) MLCTensor *mean;c
    // pub fn mean(&self) -> &mlc::Tensor;
}

extern "C" {
    static MLC_BATCH_NORMALIZATION_LAYER: &'static objc::Class<BatchNormalizationLayer>;
}
