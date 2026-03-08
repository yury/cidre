use crate::{arc, define_obj_type, ml, ns, objc};

define_obj_type!(
    #[doc(alias = "MLFeatureDescription")]
    pub FeatureDesc(ns::Id)
);

impl FeatureDesc {
    #[objc::msg_send(name)]
    pub fn name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(type)]
    pub fn type_(&self) -> ml::FeatureType;

    #[objc::msg_send(isOptional)]
    pub fn is_optional(&self) -> bool;

    #[objc::msg_send(isAllowedValue:)]
    pub fn is_allowed_value(&self, val: &ml::FeatureValue) -> bool;

    #[objc::msg_send(multiArrayConstraint)]
    pub fn multi_array_constraint(&self) -> Option<arc::R<ml::MultiArrayConstraint>>;

    #[objc::available(macos = 15.0, ios = 18.0, watchos = 11.0, tvos = 18.0)]
    #[objc::msg_send(stateConstraint)]
    pub fn state_constraint(&self) -> Option<arc::R<ml::StateConstraint>>;
}
