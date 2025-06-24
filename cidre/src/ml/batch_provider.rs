use crate::{arc, define_obj_type, ml, ns, objc};

#[objc::protocol(MLBatchProvider)]
pub trait BatchProvider: objc::Obj {
    #[objc::msg_send(count)]
    fn count(&self) -> isize;

    #[objc::msg_send(featuresAtIndex:)]
    fn features_at_index(&self, index: isize) -> arc::R<ml::AnyFeatureProvider>;
}

define_obj_type!(
    pub AnyBatchProvider(ns::Id)
);

impl BatchProvider for AnyBatchProvider {}
