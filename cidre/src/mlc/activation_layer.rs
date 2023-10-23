use crate::{arc, define_cls, define_obj_type, mlc, objc};

define_obj_type!(ActivationLayer(mlc::Layer));

impl ActivationLayer {
    define_cls!(MLC_ACTIVATION_LAYER);

    #[objc::msg_send(descriptor)]
    pub fn descriptor(&self) -> &mlc::ActivationDesc;

    #[objc::cls_msg_send(layerWithDescriptor:)]
    pub fn with_descriptor_ar(desc: &mlc::ActivationDesc) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_descriptor(desc: &mlc::ActivationDesc) -> arc::R<Self>;

    #[objc::cls_msg_send(reluLayer)]
    pub fn relu_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn relu() -> arc::R<Self>;

    #[objc::cls_msg_send(relu6Layer)]
    pub fn relu6_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn relu6() -> arc::R<Self>;

    #[objc::cls_msg_send(leakyReLULayer)]
    pub fn leaky_relu_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn leaky_relu() -> arc::R<Self>;

    #[objc::cls_msg_send(leakyReLULayerWithNegativeSlope:)]
    pub fn leaky_relu_neg_slope_ar(slope: f32) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn leaky_relu_neg_slope(slope: f32) -> arc::R<Self>;

    #[objc::cls_msg_send(linearLayerWithScale:bias:)]
    pub fn linear_scale_bias_ar(scale: f32, bias: f32) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn linear_scale_bias(scale: f32, bias: f32) -> arc::R<Self>;

    #[objc::cls_msg_send(sigmoidLayer)]
    pub fn sigmoid_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn sigmoid() -> arc::R<Self>;

    #[objc::cls_msg_send(hardSigmoidLayer)]
    pub fn hard_sigmoid_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn hard_sigmoid() -> arc::R<Self>;

    #[objc::cls_msg_send(tanhLayer)]
    pub fn tanh_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn tanh() -> arc::R<Self>;

    #[objc::cls_msg_send(absoluteLayer)]
    pub fn absolute_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn absolute() -> arc::R<Self>;

    #[objc::cls_msg_send(softPlusLayer)]
    pub fn soft_plus_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn soft_plus() -> arc::R<Self>;

    #[objc::cls_msg_send(softPlusLayerWithBeta:)]
    pub fn soft_plus_beta_ar(beta: f32) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn soft_plus_beta(beta: f32) -> arc::R<Self>;

    #[objc::cls_msg_send(softSignLayer)]
    pub fn soft_sign_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn soft_sign() -> arc::R<Self>;

    #[objc::cls_msg_send(eluLayer)]
    pub fn elu_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn elu() -> arc::R<Self>;

    #[objc::cls_msg_send(eluLayerWithA:)]
    pub fn elu_a_ar(a: f32) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn elu_a(a: f32) -> arc::R<Self>;

    #[objc::cls_msg_send(relunLayerWithA:b:)]
    pub fn relun_ab_ar(a: f32, b: f32) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn relun_ab(a: f32, b: f32) -> arc::R<Self>;

    #[objc::cls_msg_send(logSigmoidLayer)]
    pub fn log_sigmoid_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn log_sigmoid() -> arc::R<Self>;

    #[objc::cls_msg_send(seluLayer)]
    pub fn selu_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn selu() -> arc::R<Self>;

    #[objc::cls_msg_send(celuLayer)]
    pub fn celu_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn celu() -> arc::R<Self>;

    #[objc::cls_msg_send(celuLayerWithA:)]
    pub fn celu_a_ar(a: f32) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn celu_a(a: f32) -> arc::R<Self>;

    #[objc::cls_msg_send(hardShrinkLayer)]
    pub fn hard_shrink_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn hard_shrink() -> arc::R<Self>;

    #[objc::cls_msg_send(hardShrinkLayerWithA:)]
    pub fn hard_shrink_a_ar(a: f32) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn hard_shrink_a(a: f32) -> arc::R<Self>;

    #[objc::cls_msg_send(softShrinkLayer)]
    pub fn soft_shrink_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn soft_shrink() -> arc::R<Self>;

    #[objc::cls_msg_send(softShrinkLayerWithA:)]
    pub fn soft_shrink_a_ar(a: f32) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn soft_shrink_a(a: f32) -> arc::R<Self>;

    #[objc::cls_msg_send(tanhShrinkLayer)]
    pub fn tanh_shrink_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn tanh_shrink() -> arc::R<Self>;

    #[objc::cls_msg_send(thresholdLayerWithThreshold:replacement:)]
    pub fn threshold_replacement_ar(threshold: f32, replacement: f32) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn threshold_replacement(threshold: f32, replacement: f32) -> arc::R<Self>;

    #[objc::cls_msg_send(geluLayer)]
    pub fn gelu_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn gelu() -> arc::R<Self>;

    #[objc::cls_msg_send(hardSwishLayer)]
    pub fn hard_swish_ar() -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn hard_swish() -> arc::R<Self>;

    #[objc::cls_msg_send(clampLayerWithMinValue:maxValue:)]
    pub fn clamp_ar(min: f32, max: f32) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn clamp(min: f32, max: f32) -> arc::R<Self>;
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_ACTIVATION_LAYER: &'static objc::Class<ActivationLayer>;
}

#[cfg(test)]
mod tests {
    use crate::mlc;

    #[test]
    fn basics() {
        let clamp = mlc::ActivationLayer::clamp(0.0, 1.0);
        println!("clamp {clamp:?}");
    }
}
