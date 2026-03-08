use crate::{arc, define_obj_type, ml, ns, objc};

define_obj_type!(
    #[doc(alias = "MLStateConstraint")]
    pub StateConstraint(ns::Id)
);

impl StateConstraint {
    #[objc::msg_send(bufferShape)]
    pub fn buffer_shape(&self) -> arc::R<ns::Array<ns::Number>>;

    #[objc::msg_send(dataType)]
    pub fn data_type(&self) -> ml::MultiArrayDType;
}
