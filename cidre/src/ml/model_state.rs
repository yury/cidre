use crate::{arc, ml, ns, objc};

impl ml::Model {
    #[objc::available(macos = 15.0, ios = 18.0, watchos = 11.0, tvos = 18.0)]
    #[objc::msg_send(newState)]
    pub fn new_state(&self) -> arc::R<ml::State>;

    #[objc::msg_send(predictionFromFeatures:usingState:error:)]
    pub unsafe fn prediction_from_features_using_state_err<'ear, F: ml::FeatureProvider>(
        &self,
        input_features: &F,
        state: &ml::State,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ml::AnyFeatureProvider>>;

    #[objc::available(macos = 15.0, ios = 18.0, watchos = 11.0, tvos = 18.0)]
    pub fn prediction_from_features_using_state<'ear, F: ml::FeatureProvider>(
        &self,
        input_features: &F,
        state: &ml::State,
    ) -> ns::Result<'ear, arc::R<ml::AnyFeatureProvider>> {
        ns::if_none(|err| unsafe {
            self.prediction_from_features_using_state_err(input_features, state, err)
        })
    }

    #[objc::msg_send(predictionFromFeatures:usingState:options:error:)]
    pub unsafe fn prediction_from_features_using_state_opts_err<'ear, F: ml::FeatureProvider>(
        &self,
        input_features: &F,
        state: &ml::State,
        options: &ml::PredictionOpts,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ml::AnyFeatureProvider>>;

    #[objc::available(macos = 15.0, ios = 18.0, watchos = 11.0, tvos = 18.0)]
    pub fn prediction_from_features_using_state_opts<'ear, F: ml::FeatureProvider>(
        &self,
        input_features: &F,
        state: &ml::State,
        options: &ml::PredictionOpts,
    ) -> ns::Result<'ear, arc::R<ml::AnyFeatureProvider>> {
        ns::if_none(|err| unsafe {
            self.prediction_from_features_using_state_opts_err(input_features, state, options, err)
        })
    }
}
