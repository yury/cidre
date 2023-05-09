use crate::{define_obj_type, mlc, ns, objc};

define_obj_type!(Optimizer(ns::Id));

impl Optimizer {
    /// The learning rate.  This property is 'readwrite' so that callers can implement a 'decay' during training
    #[objc::msg_send(learningRate)]
    pub fn learning_rate(&self) -> f32;

    #[objc::msg_send(setLearningRate:)]
    pub fn set_learning_rate(&mut self, value: f32);

    #[objc::msg_send(gradientRescale)]
    pub fn grad_rescale(&self) -> f32;

    #[objc::msg_send(setGradientRescale:)]
    pub fn set_grad_rescale(&mut self, value: f32);

    #[objc::msg_send(appliesGradientClipping)]
    pub fn applies_grad_clipping(&self) -> f32;

    #[objc::msg_send(setAppliesGradientClipping:)]
    pub fn set_applies_grad_clipping(&mut self, value: f32);

    #[objc::msg_send(gradientClipMax)]
    pub fn grad_clip_max(&self) -> f32;

    #[objc::msg_send(gradientClipMin)]
    pub fn grad_clip_min(&self) -> f32;

    #[objc::msg_send(regularizationScale)]
    pub fn regularization_scale(&self) -> f32;

    #[objc::msg_send(gradientClippingType)]
    pub fn grad_clipping_type(&self) -> mlc::GradientClippingType;

    #[objc::msg_send(maximumClippingNorm)]
    pub fn max_clipping_norm(&self) -> f32;

    #[objc::msg_send(customGlobalNorm)]
    pub fn custom_global_norm(&self) -> f32;
}
