use crate::{arc, define_obj_type, ml, ns, objc};

define_obj_type!(
    #[doc(alias = "MLModelDescription")]
    pub ModelDesc(ns::Id)
);

impl ModelDesc {
    #[objc::msg_send(inputDescriptionsByName)]
    pub fn input_descs_by_name(&self) -> arc::R<ns::Dictionary<ns::String, ml::FeatureDesc>>;

    #[objc::msg_send(outputDescriptionsByName)]
    pub fn output_descs_by_name(&self) -> arc::R<ns::Dictionary<ns::String, ml::FeatureDesc>>;

    #[objc::msg_send(stateDescriptionsByName)]
    pub fn state_descs_by_name(&self) -> arc::R<ns::Dictionary<ns::String, ml::FeatureDesc>>;

    #[objc::msg_send(predictedFeatureName)]
    pub fn predicted_feature_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(predictedProbabilitiesName)]
    pub fn predicted_probabilities_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(metadata)]
    pub fn metadata(&self) -> arc::R<ns::Dictionary<ns::String, ns::Id>>;

    #[objc::msg_send(classLabels)]
    pub fn class_labels(&self) -> Option<arc::R<ns::Array<ns::Id>>>;
}

/// MLUpdateAdditions
impl ModelDesc {
    #[objc::msg_send(isUpdatable)]
    pub fn is_updatable(&self) -> bool;

    #[objc::msg_send(trainingInputDescriptionsByName)]
    pub fn training_input_descs_by_name(
        &self,
    ) -> arc::R<ns::Dictionary<ns::String, ml::FeatureDesc>>;
}
