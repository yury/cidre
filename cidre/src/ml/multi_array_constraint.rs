use crate::{arc, define_obj_type, ml, ns, objc};

define_obj_type!(
    #[doc(alias = "MLMultiArrayConstraint")]
    pub MultiArrayConstraint(ns::Id)
);

impl MultiArrayConstraint {
    #[objc::msg_send(shape)]
    pub fn shape(&self) -> arc::R<ns::Array<ns::Number>>;

    #[objc::msg_send(dataType)]
    pub fn data_type(&self) -> ml::MultiArrayDType;

    #[objc::available(macos = 10.14, ios = 12.0, watchos = 5.0, tvos = 12.0)]
    #[objc::msg_send(shapeConstraint)]
    pub fn shape_constraint(&self) -> arc::R<ml::MultiArrayShapeConstraint>;
}
