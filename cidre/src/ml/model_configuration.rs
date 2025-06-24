use crate::{api, arc, define_obj_type, ns, objc};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[doc(alias = "MLComputeUnits")]
#[repr(isize)]
pub enum ComputeUnits {
    #[doc(alias = "MLComputeUnitsCPUOnly")]
    Cpu = 0,
    #[doc(alias = "MLComputeUnitsCPUAndGPU")]
    CpuGpu = 1,
    #[doc(alias = "MLComputeUnitsAll")]
    All = 2,
    #[doc(alias = "MLComputeUnitsCPUAndNeuralEngine")]
    CpuNe = 3,
}

define_obj_type!(
    #[doc(alias = "MLModelConfiguration")]
    pub ModelCfg(ns::Id),
    ML_MODEL_CONFIGURATION,
    #[api::available(macos = 10.15, ios = 13.0, watchos = 6.0, tvos = 13.0)]
);

impl ModelCfg {
    #[objc::msg_send(modelDisplayName)]
    pub fn model_display_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setModelDisplayName:)]
    pub fn set_model_display_name(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(computeUnits)]
    pub fn compute_units(&self) -> ComputeUnits;

    #[objc::msg_send(setComputeUnits:)]
    pub fn set_compute_units(&mut self, val: ComputeUnits);
}

#[link(name = "ml", kind = "static")]
unsafe extern "C" {
    static ML_MODEL_CONFIGURATION: &'static objc::Class<ModelCfg>;
}
