use crate::{arc, define_cls, define_obj_type, mlc, objc};

define_obj_type!(AdamWOptimizer(mlc::Optimizer));
impl AdamWOptimizer {
    define_cls!(MLC_ADAMW_OPTIMIZER);

    #[objc::msg_send(beta1)]
    pub fn beta1(&self) -> f32;

    #[objc::msg_send(beta2)]
    pub fn beta2(&self) -> f32;

    #[objc::msg_send(epsilon)]
    pub fn epsilon(&self) -> f32;

    #[objc::msg_send(usesAMSGrad)]
    pub fn uses_ams_grad(&self) -> bool;

    #[objc::msg_send(timeStep)]
    pub fn time_step(&self) -> usize;

    #[objc::cls_msg_send(optimizerWithDescriptor:)]
    pub fn with_descriptor_ar(desc: &mlc::OptimizerDesc) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_descriptor(desc: &mlc::OptimizerDesc) -> arc::R<Self>;

    #[objc::cls_msg_send(optimizerWithDescriptor:beta1:beta2:epsilon:usesAMSGrad:timeStep:)]
    pub fn with_descriptor_betas_epsilon_uses_ams_grad_time_step_ar(
        desc: &mlc::OptimizerDesc,
        beta1: f32,
        beta2: f32,
        epsilon: f32,
        uses_ams_grad: bool,
        time_step: usize,
    ) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_descriptor_betas_epsilon_uses_ams_grad_time_step(
        desc: &mlc::OptimizerDesc,
        beta1: f32,
        beta2: f32,
        epsilon: f32,
        uses_ams_grad: bool,
        time_step: usize,
    ) -> arc::R<Self>;
}

#[link(name = "mlc", kind = "static")]
extern "C" {
    static MLC_ADAMW_OPTIMIZER: &'static objc::Class<AdamWOptimizer>;
}
