use crate::{arc, define_obj_type, ml, ns, objc};

define_obj_type!(
    #[doc(alias = "MLMultiArrayShapeConstraint")]
    pub MultiArrayShapeConstraint(ns::Id)
);

impl MultiArrayShapeConstraint {
    #[objc::msg_send(type)]
    pub fn type_(&self) -> ml::MultiArrayShapeConstraintType;

    #[objc::msg_send(sizeRangeForDimension)]
    pub fn size_range_for_dimension(&self) -> arc::R<ns::Array<ns::Value>>;

    #[objc::msg_send(enumeratedShapes)]
    pub fn enumerated_shapes(&self) -> arc::R<ns::Array<ns::Array<ns::Number>>>;
}
