use crate::{arc, define_obj_type, ns, objc};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum Mutability {
    #[default]
    Default = 0,
    Mutable = 1,
    Immutable = 2,
}

define_obj_type!(
    PipelineBufDescriptor(ns::Id),
    MTL_PIPELINE_BUFFER_DESCRIPTOR
);

impl PipelineBufDescriptor {
    #[objc::msg_send(mutability)]
    pub fn mutability(&self) -> Mutability;

    #[objc::msg_send(setMutability:)]
    pub fn set_mutability(&mut self, value: Mutability);
}

define_obj_type!(PipelineBufDescriptorArray(ns::Id));
impl PipelineBufDescriptorArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn obj_at(&self, index: usize) -> &PipelineBufDescriptor;

    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn obj_mut_at(&mut self, index: usize) -> &mut PipelineBufDescriptor;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set_obj_at(&mut self, buffer: Option<&PipelineBufDescriptor>, at_index: usize);
}

impl std::ops::Index<usize> for PipelineBufDescriptorArray {
    type Output = PipelineBufDescriptor;

    fn index(&self, index: usize) -> &Self::Output {
        self.obj_at(index)
    }
}

impl std::ops::IndexMut<usize> for PipelineBufDescriptorArray {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.obj_mut_at(index)
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_PIPELINE_BUFFER_DESCRIPTOR: &'static objc::Class<PipelineBufDescriptor>;
}
