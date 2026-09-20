use crate::{api, arc, define_obj_type, mtl4, ns, objc};

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

impl Compiler {
    #[objc::msg_send(newMachineLearningPipelineStateWithDescriptor:error:)]
    pub unsafe fn new_ml_pipeline_state_err<'ear>(
        &self,
        desc: &mtl4::MlPipelineDesc,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<mtl4::MlPipelineState>>;

    /// Creates a new ML pipeline state with descriptor.
    #[inline]
    pub fn new_ml_pipeline_state<'ear>(
        &self,
        desc: &mtl4::MlPipelineDesc,
    ) -> ns::Result<'ear, arc::R<mtl4::MlPipelineState>> {
        ns::if_none(|err| unsafe { self.new_ml_pipeline_state_err(desc, err) })
    }
}

unsafe extern "C" {
    static MTL4_COMPILER_DESCRIPTOR: &'static objc::Class<CompilerDesc>;
}
