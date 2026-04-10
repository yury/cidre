use crate::{api, arc, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "MTL4CompilerDescriptor")]
    pub CompilerDesc(ns::Id),
    MTL4_COMPILER_DESCRIPTOR,
    #[api::available(
        macos = 26.0,
        ios = 26.0,
        maccatalyst = 26.0,
        tvos = 26.0,
        visionos = 26.0
    )]

);

impl ns::Copying for CompilerDesc {}

define_obj_type!(
    #[doc(alias = "MTL4Compiler")]
    pub Compiler(ns::Id)
);

unsafe extern "C" {
    static MTL4_COMPILER_DESCRIPTOR: &'static objc::Class<CompilerDesc>;
}
