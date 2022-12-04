use crate::{define_mtl, define_obj_type, ext_msg_send, mtl, ns};

define_obj_type!(ArgumentEncoder(ns::Id));

impl ArgumentEncoder {
    define_mtl!(device, label, set_label);

    pub fn encoded_length(&self) -> usize {
        ext_msg_send!("mtl", self, sel_encodedLength)
    }

    pub fn aligment(&self) -> usize {
        ext_msg_send!("mtl", self, sel_aligment)
    }

    #[inline]
    pub fn set_argument_buffer(&mut self, buffer: Option<&mtl::Buffer>, offset: usize) {
        ext_msg_send!("mtl", self, sel_setArgumentBuffer_offset, buffer, offset)
    }

    #[inline]
    pub fn set_texture(&mut self, texture: Option<&mtl::Texture>, at_index: usize) {
        ext_msg_send!("mtl", self, sel_setTexture_atIndex, texture, at_index)
    }
}
