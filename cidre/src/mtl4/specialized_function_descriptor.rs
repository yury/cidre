use crate::{arc, define_obj_type, mtl, mtl4, ns, objc};

define_obj_type!(
    #[doc(alias = "MTL4SpecializedFunctionDescriptor")]
    pub SpecializedFnDesc(mtl4::FnDesc)
);

impl SpecializedFnDesc {
    #[objc::msg_send(functionDescriptor)]
    pub fn fn_desc(&self) -> Option<arc::R<mtl4::FnDesc>>;

    #[objc::msg_send(setFunctionDescriptor:)]
    pub fn set_fn_desc(&mut self, val: Option<&mtl4::FnDesc>);

    #[objc::msg_send(specializedName)]
    pub fn specialized_name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setSpecializedName:)]
    pub fn set_specialized_name(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(constantValues)]
    pub fn constant_values(&self) -> Option<arc::R<mtl::FnConstValues>>;

    #[objc::msg_send(setConstantValues:)]
    pub fn set_constant_values(&mut self, val: Option<&mtl::FnConstValues>);
}
