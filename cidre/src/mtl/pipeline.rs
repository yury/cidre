use crate::{arc, define_obj_type, ns, objc};

#[doc(alias = "MTLMutability")]
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum Mutability {
    #[default]
    Default = 0,
    Mutable = 1,
    Immutable = 2,
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
    pub fn set(&mut self, val: Option<&PipelineBufDesc>, at_index: usize);
}

#[link(name = "mtl", kind = "static")]
unsafe extern "C" {
    static MTL_PIPELINE_BUFFER_DESCRIPTOR: &'static objc::Class<PipelineBufDesc>;
}
