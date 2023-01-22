use crate::{define_mtl, define_obj_type, mtl, ns, objc};

define_obj_type!(ArgumentEncoder(ns::Id));

impl ArgumentEncoder {
    define_mtl!(device, label, set_label);

    #[objc::msg_send(encodedLength)]
    pub fn encoded_length(&self) -> usize;

    #[objc::msg_send(aligment)]
    pub fn aligment(&self) -> usize;

    #[objc::msg_send(setArgumentBuffer:offset:)]
    pub fn set_argument_buffer(&mut self, buffer: Option<&mtl::Buffer>, offset: usize);

    #[objc::msg_send(setTexture:atIndex:)]
    pub fn set_texture(&mut self, texture: Option<&mtl::Texture>, at_index: usize);
}
