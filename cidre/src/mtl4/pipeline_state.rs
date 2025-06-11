use crate::{api, arc, define_obj_type, define_opts, mtl, mtl4, ns, objc};

define_opts!(
    #[doc(alias = "MTL4ShaderReflection")]
    pub ShaderReflection(usize)
);

impl ShaderReflection {
    /// Requests no information.
    pub const NONE: Self = Self(0);

    /// Requests reflection information for bindings.
    pub const BINDING_INFO: Self = Self(1 << 0);

    /// Requests reflection information for buffer types.
    pub const BUF_TYPE_INFO: Self = Self(1 << 1);
}

/// Enumeration for controlling alpha-to-one state of a pipeline state object.
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[doc(alias = "MTL4AlphaToOneState")]
#[repr(isize)]
pub enum AlphaToOneState {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[doc(alias = "MTL4AlphaToCoverageState")]
#[repr(isize)]
pub enum AlphaToCoverageState {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[doc(alias = "MTL4BlendState")]
#[repr(isize)]
pub enum BlendState {
    #[default]
    Disabled = 0,
    Enabled = 1,
    Unspecialized = 2,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
#[doc(alias = "MTL4IndirectCommandBufferSupportState")]
#[repr(isize)]
pub enum IndirectCmdBufSupportState {
    #[default]
    Disabled = 0,
    Enabled = 1,
}

define_obj_type!(
    #[doc(alias = "MTL4PipelineOptions")]
    pub PipelineOpts(ns::Id),
    MTL4_PIPELINE_OPTIONS,
    #[api::available(macos = 26.0, ios = 26.0, tvos = 26.0, visionos = 26.0)]
);

impl PipelineOpts {
    #[objc::msg_send(shaderValidation)]
    pub fn shader_validation(&self) -> mtl::ShaderValidation;

    #[objc::msg_send(setShaderValidation:)]
    pub fn set_shader_validation(&mut self, val: mtl::ShaderValidation);

    #[objc::msg_send(shaderReflection)]
    pub fn shader_reflection(&self) -> mtl4::ShaderReflection;

    #[objc::msg_send(setShaderReflection:)]
    pub fn set_shader_reflection(&mut self, val: mtl4::ShaderReflection);
}

define_obj_type!(
    /// Base type for descriptors you use for building pipeline state objects.
    #[doc(alias = "MTL4PipelineDescriptor")]
    pub PipelineDesc(ns::Id)
);

impl PipelineDesc {
    #[objc::msg_send(label)]
    pub fn label(&self) -> Option<arc::R<ns::String>>;

    #[objc::msg_send(setLabel:)]
    pub fn set_label(&mut self, val: Option<&ns::String>);

    #[objc::msg_send(options)]
    pub fn opts(&self) -> arc::R<mtl4::PipelineOpts>;

    #[objc::msg_send(setOptions:)]
    pub fn set_opts(&mut self, val: &mtl4::PipelineOpts);
}

unsafe extern "C" {
    static MTL4_PIPELINE_OPTIONS: &'static objc::Class<PipelineOpts>;
}
