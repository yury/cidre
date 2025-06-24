use crate::{arc, define_obj_type, ml, ns, objc};

#[objc::protocol(MLFeatureProvider)]
pub trait FeatureProvider: objc::Obj {
    #[objc::msg_send(featureNames)]
    fn feature_names(&self) -> arc::R<ns::Set<ns::String>>;

    #[objc::msg_send(featureValueForName:)]
    fn feature_value_for_name(&self, name: &ns::String) -> Option<arc::R<ml::FeatureValue>>;
}

define_obj_type!(
    pub AnyFeatureProvider(ns::Id)
);

impl FeatureProvider for AnyFeatureProvider {}
