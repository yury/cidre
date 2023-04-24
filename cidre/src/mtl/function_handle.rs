use crate::{mtl, ns, objc, objc::Obj};

pub trait FnHandle<T: Obj>: Obj {
    #[objc::msg_send(functionType)]
    fn fn_type(&self) -> mtl::FnType;

    #[objc::msg_send(name)]
    fn name(&self) -> &ns::String;

    #[objc::msg_send(device)]
    fn device(&self) -> &mtl::Device;
}
