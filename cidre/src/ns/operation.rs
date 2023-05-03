use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(Operation(ns::Id));
impl Operation {
    define_cls!(NS_OPERATION);
}

define_obj_type!(OperationQueue(ns::Id), NS_OPERATION_QUEUE);

impl OperationQueue {}

extern "C" {
    static NS_OPERATION: &'static objc::Class<Operation>;
    static NS_OPERATION_QUEUE: &'static objc::Class<OperationQueue>;
}
