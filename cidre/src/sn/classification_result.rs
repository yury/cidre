use crate::{arc, cm, define_obj_type, ns, objc, sn};

define_obj_type!(pub Classification(ns::Id));

impl Classification {
    #[objc::msg_send(identifier)]
    pub fn id(&self) -> &ns::String;

    #[objc::msg_send(confidence)]
    pub fn confidence(&self) -> f64;
}

define_obj_type!(
    pub ClassificationResult(sn::Result),
    SN_CLASSIFICATION_RESULT
);

impl ClassificationResult {
    #[objc::msg_send(classifications)]
    pub fn classifications(&self) -> &ns::Array<Classification>;

    #[objc::msg_send(timeRange)]
    pub fn time_range(&self) -> cm::TimeRange;

    #[objc::msg_send(classificationForIdentifier:)]
    pub fn classification_for_id<'a>(&'a self, id: &ns::String) -> Option<&'a Classification>;
}

extern "C" {
    static SN_CLASSIFICATION_RESULT: &'static objc::Class<ClassificationResult>;
}
