use crate::{arc, define_obj_type, mtl, mtl4, ns, objc};

define_obj_type!(
    #[doc(alias = "MTL4LibraryFunctionDescriptor")]
    pub LibFnDesc(mtl4::FnDesc)
);

impl LibFnDesc {
    #[objc::msg_send(name)]
    pub fn name(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setName:)]
    pub fn set_name(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(library)]
    pub fn lib(&self) -> Option<arc::R<mtl::Lib>>;

    #[objc::msg_send(setLibrary:)]
    pub fn set_lib(&mut self, val: Option<&mtl::Lib>);
}
