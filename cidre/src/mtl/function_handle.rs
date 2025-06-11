use crate::{
    arc, define_obj_type, mtl, ns,
    objc::{self, Obj},
};

pub trait FnHandle: Obj {
    #[objc::msg_send(functionType)]
    fn fn_type(&self) -> mtl::FnType;

    #[objc::msg_send(name)]
    fn name(&self) -> arc::R<ns::String>;

    #[objc::msg_send(device)]
    fn device(&self) -> arc::R<mtl::Device>;
}

define_obj_type!(
    pub AnyFnHandle(ns::Id)
);

impl FnHandle for AnyFnHandle {}
