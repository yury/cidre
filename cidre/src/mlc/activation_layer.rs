use crate::{arc, define_cls, define_obj_type, mlc, objc};

define_obj_type!(pub ActivationLayer(mlc::Layer));

impl ActivationLayer {
    define_cls!(MLC_ACTIVATION_LAYER);

    #[objc::msg_send(descriptor)]
    pub fn descriptor(&self) -> &mlc::ActivationDesc;

    #[objc::msg_send2(layerWithDescriptor:)]
    pub fn with_descriptor(desc: &mlc::ActivationDesc) -> arc::R<Self>;

    #[objc::msg_send2(reluLayer)]
    pub fn relu() -> arc::R<Self>;

    #[objc::msg_send2(relu6Layer)]
    pub fn relu6() -> arc::R<Self>;

    #[objc::msg_send2(leakyReLULayer)]
    pub fn leaky_relu() -> arc::R<Self>;

    #[objc::msg_send2(leakyReLULayerWithNegativeSlope:)]
    pub fn leaky_relu_neg_slope(slope: f32) -> arc::R<Self>;

    #[objc::msg_send2(linearLayerWithScale:bias:)]
    pub fn linear_scale_bias(scale: f32, bias: f32) -> arc::R<Self>;

    #[objc::msg_send2(sigmoidLayer)]
    pub fn sigmoid() -> arc::R<Self>;

    #[objc::msg_send2(hardSigmoidLayer)]
    pub fn hard_sigmoid() -> arc::R<Self>;

    #[objc::msg_send2(tanhLayer)]
    pub fn tanh() -> arc::R<Self>;

    #[objc::msg_send2(absoluteLayer)]
    pub fn absolute() -> arc::R<Self>;

    #[objc::msg_send2(softPlusLayer)]
    pub fn soft_plus() -> arc::R<Self>;

    #[objc::msg_send2(softPlusLayerWithBeta:)]
    pub fn soft_plus_beta(beta: f32) -> arc::R<Self>;

    #[objc::msg_send2(softSignLayer)]
    pub fn soft_sign() -> arc::R<Self>;

    #[objc::msg_send2(eluLayer)]
    pub fn elu() -> arc::R<Self>;

    #[objc::msg_send2(eluLayerWithA:)]
    pub fn elu_a(a: f32) -> arc::R<Self>;

    #[objc::msg_send2(relunLayerWithA:b:)]
    pub fn relun_ab(a: f32, b: f32) -> arc::R<Self>;

    #[objc::msg_send2(logSigmoidLayer)]
    pub fn log_sigmoid() -> arc::R<Self>;

    #[objc::msg_send2(seluLayer)]
    pub fn selu() -> arc::R<Self>;

    #[objc::msg_send2(celuLayer)]
    pub fn celu() -> arc::R<Self>;

    #[objc::msg_send2(celuLayerWithA:)]
    pub fn celu_a(a: f32) -> arc::R<Self>;

    #[objc::msg_send2(hardShrinkLayer)]
    pub fn hard_shrink() -> arc::R<Self>;

    #[objc::msg_send2(hardShrinkLayerWithA:)]
    pub fn hard_shrink_a(a: f32) -> arc::R<Self>;

    #[objc::msg_send2(softShrinkLayer)]
    pub fn soft_shrink() -> arc::R<Self>;

    #[objc::msg_send2(softShrinkLayerWithA:)]
    pub fn soft_shrink_a(a: f32) -> arc::R<Self>;

    #[objc::msg_send2(tanhShrinkLayer)]
    pub fn tanh_shrink() -> arc::R<Self>;

    #[objc::msg_send2(thresholdLayerWithThreshold:replacement:)]
    pub fn threshold_replacement(threshold: f32, replacement: f32) -> arc::R<Self>;

    #[objc::msg_send2(geluLayer)]
    pub fn gelu() -> arc::R<Self>;

    #[objc::msg_send2(hardSwishLayer)]
    pub fn hard_swish() -> arc::R<Self>;

    #[objc::msg_send2(clampLayerWithMinValue:maxValue:)]
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
