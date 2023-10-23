use crate::{arc, define_cls, define_obj_type, mlc, ns, objc};

define_obj_type!(OptimizerDesc(ns::Id));
impl OptimizerDesc {
    define_cls!(MLC_OPTIMIZER_DESCRIPTOR);

    #[objc::msg_send(learningRate)]
    pub fn learning_rate(&self) -> f32;

    #[objc::msg_send(gradientRescale)]
    pub fn grad_rescale(&self) -> f32;

    #[objc::msg_send(appliesGradientClipping)]
    pub fn applies_grad_clipping(&self) -> f32;

    #[objc::msg_send(gradientClipMax)]
    pub fn grad_clip_max(&self) -> f32;

    #[objc::msg_send(gradientClipMin)]
    pub fn grad_clip_min(&self) -> f32;

    #[objc::msg_send(regularizationScale)]
    pub fn regularization_scale(&self) -> f32;

    #[objc::msg_send(regularizationType)]
    pub fn regularization_type(&self) -> mlc::RegularizationType;

    #[objc::msg_send(gradientClippingType)]
    pub fn grad_clipping_type(&self) -> mlc::GradientClippingType;

    #[objc::msg_send(maximumClippingNorm)]
    pub fn max_clipping_norm(&self) -> f32;

    #[objc::msg_send(customGlobalNorm)]
    pub fn custom_global_norm(&self) -> f32;

    #[objc::cls_msg_send(descriptorWithLearningRate:gradientRescale:regularizationType:regularizationScale:)]
    pub fn with_learning_rate_grad_rescale_regularization_ar(
        learning_rate: f32,
        gradient_rescale: f32,
        regularization_type: mlc::RegularizationType,
        regularization_scale: f32,
    ) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_learning_rate_grad_rescale_regularization(
        learning_rate: f32,
        gradient_rescale: f32,
        regularization_type: mlc::RegularizationType,
        regularization_scale: f32,
    ) -> arc::R<Self>;

    #[objc::cls_msg_send(
        descriptorWithLearningRate:gradientRescale:
        appliesGradientClipping:gradientClipMax:gradientClipMin:
        regularizationType:regularizationScale:
    )]
    pub fn with_learning_rate_grad_rescale_clip_regularization_ar(
        learning_rate: f32,
        gradient_rescale: f32,
        applies_grad_clipping: bool,
        grad_clip_max: f32,
        grad_clip_min: f32,
        regularization_type: mlc::RegularizationType,
        regularization_scale: f32,
    ) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_learning_rate_grad_rescale_clip_regularization(
        learning_rate: f32,
        gradient_rescale: f32,
        applies_grad_clipping: bool,
        grad_clip_max: f32,
        grad_clip_min: f32,
        regularization_type: mlc::RegularizationType,
        regularization_scale: f32,
    ) -> arc::R<Self>;

    #[objc::cls_msg_send(
        descriptorWithLearningRate:gradientRescale:
        appliesGradientClipping:gradientClipMax:gradientClipMin:
        maximumClippingNorm:customGlobalNorm:
        regularizationType:regularizationScale:
    )]
    pub fn with_learning_rate_grad_rescale_clip_norm_regularization_ar(
        learning_rate: f32,
        gradient_rescale: f32,
        applies_grad_clipping: bool,
        grad_clip_max: f32,
        grad_clip_min: f32,
        max_clip_norm: f32,
        custom_global_norm: f32,
        regularization_type: mlc::RegularizationType,
        regularization_scale: f32,
    ) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_learning_rate_grad_rescale_clip_norm_regularization(
        learning_rate: f32,
        gradient_rescale: f32,
        applies_grad_clipping: bool,
        grad_clip_max: f32,
        grad_clip_min: f32,
        max_clip_norm: f32,
        custom_global_norm: f32,
        regularization_type: mlc::RegularizationType,
        regularization_scale: f32,
    ) -> arc::R<Self>;
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_OPTIMIZER_DESCRIPTOR: &'static objc::Class<OptimizerDesc>;
}
