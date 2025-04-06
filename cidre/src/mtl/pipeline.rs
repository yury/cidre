use crate::{arc, define_obj_type, ns, objc};

/// Specifies whether a buffer will be modified between the time it is bound and a compute
/// or render pipeline is executed in a draw or dispatch.
#[doc(alias = "MTLMutability")]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum Mutability {
    #[default]
    Default = 0,
    Mutable = 1,
    Immutable = 2,
}

#[doc(alias = "MTLShaderValidation")]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ShaderValidation {
    #[default]
    Default = 0,
    Enabled = 1,
    Disabled = 2,
}

define_obj_type!(
    #[doc(alias = "MTLPipelineBufferDescriptor")]
    pub PipelineBufDesc(ns::Id), MTL_PIPELINE_BUFFER_DESCRIPTOR
);

impl PipelineBufDesc {
    #[objc::msg_send(mutability)]
    pub fn mutability(&self) -> Mutability;

    #[objc::msg_send(setMutability:)]
    pub fn set_mutability(&mut self, val: Mutability);
}

define_obj_type!(
    #[doc(alias = "MTLPipelineBufferDescriptorArray")]
    pub PipelineBufDescArray(ns::Id)
);

impl PipelineBufDescArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get(&self, index: usize) -> arc::R<PipelineBufDesc>;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set(&mut self, val: Option<&PipelineBufDesc>, index: usize);
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL_PIPELINE_BUFFER_DESCRIPTOR: &'static objc::Class<PipelineBufDesc>;
}
