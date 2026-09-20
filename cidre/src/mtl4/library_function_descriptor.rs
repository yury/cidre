use crate::{api, arc, define_obj_type, mtl, mtl4, ns, objc};

define_obj_type!(
    #[doc(alias = "MTL4LibraryFunctionDescriptor")]
    pub LibFnDesc(mtl4::FnDesc),
    MTL4_LIBRARY_FUNCTION_DESCRIPTOR,
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
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

unsafe extern "C" {
    static MTL4_LIBRARY_FUNCTION_DESCRIPTOR: &'static objc::Class<LibFnDesc>;
}
