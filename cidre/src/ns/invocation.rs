use crate::{arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSInvocation")]
    pub Invocation(ns::Id)
);

impl Invocation {
    #[objc::msg_send(retainArguments)]
    pub fn retain_args(&mut self);

    #[objc::msg_send(argumentsRetained)]
    pub fn args_retained(&self) -> bool;

    #[objc::msg_send(target)]
    pub fn target(&self) -> Option<arc::R<ns::Id>>;

    #[objc::msg_send(setTarget:)]
    pub fn set_target(&mut self, val: Option<&ns::Id>);

    #[objc::msg_send(invokeWithTarget:)]
    pub fn invoke_with_target(&self, target: &ns::Id);
}
