use crate::{api, arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "MLPredictionOptions")]
    pub PredictionOpts(ns::Id),
    ML_PREDICTION_OPTIONS,
    #[api::available(macos = 10.13, ios = 11.0, watchos = 4.0, tvos = 11.0)]
);

impl PredictionOpts {
    #[objc::msg_send(usesCPUOnly)]
    pub fn uses_cpu_only(&self) -> bool;

    #[objc::msg_send(setUsesCPUOnly:)]
    pub fn set_uses_cpu_only(&mut self, val: bool);

    #[objc::available(macos = 11.0, ios = 16.0, watchos = 9.0, tvos = 16.0)]
    #[objc::msg_send(outputBackings)]
    pub fn output_backings(&self) -> arc::R<ns::Dictionary<ns::String, ns::Id>>;

    #[objc::available(macos = 11.0, ios = 16.0, watchos = 9.0, tvos = 16.0)]
    #[objc::msg_send(setOutputBackings:)]
    pub fn set_output_backings(&mut self, val: &ns::Dictionary<ns::String, ns::Id>);
}

unsafe extern "C" {
    static ML_PREDICTION_OPTIONS: &'static objc::Class<PredictionOpts>;
}
