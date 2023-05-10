use crate::{arc, define_cls, define_obj_type, mlc, objc};

define_obj_type!(BatchNormalizationLayer(mlc::Layer));

impl BatchNormalizationLayer {
    define_cls!(MLC_BATCH_NORMALIZATION_LAYER);

    /// The number of feature channels
    #[objc::msg_send(featureChannelCount)]
    pub fn feature_channel_count(&self) -> usize;

    #[objc::msg_send(mean)]
    pub fn mean(&self) -> &mlc::Tensor;

    #[objc::msg_send(beta)]
    pub fn beta(&self) -> Option<&mlc::Tensor>;

    #[objc::msg_send(gamma)]
    pub fn gamma(&self) -> Option<&mlc::Tensor>;
}

extern "C" {
    static MLC_BATCH_NORMALIZATION_LAYER: &'static objc::Class<BatchNormalizationLayer>;
}
