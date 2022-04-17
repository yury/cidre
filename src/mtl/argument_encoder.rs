use crate::ns::Id;
use crate::{define_mtl, define_obj_type, msg_send};

use super::{Buffer, Texture};

define_obj_type!(ArgumentEncoder(Id));

impl ArgumentEncoder {
    define_mtl!(device, label, set_label);

    pub fn encoded_length(&self) -> usize {
        msg_send!("mtl", self, sel_encodedLength)
    }

    pub fn aligment(&self) -> usize {
        msg_send!("mtl", self, sel_aligment)
    }

    #[inline]
    pub fn set_argument_buffer(&mut self, buffer: Option<&Buffer>, offset: usize) {
        msg_send!("mtl", self, sel_setArgumentBuffer_offset, buffer, offset)
    }

    #[inline]
    pub fn set_texture(&mut self, texture: Option<&Texture>, at_index: usize) {
        msg_send!("mtl", self, sel_setTexture_atIndex, texture, at_index)
    }
}
