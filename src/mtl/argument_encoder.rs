use crate::ns::Id;
use crate::{define_mtl, define_obj_type};

use super::{Buffer, Texture};

define_obj_type!(ArgumentEncoder(Id));

impl ArgumentEncoder {
    define_mtl!(device, label, set_label);

    pub fn encoded_length(&self) -> usize {
        unsafe { rsel_encodedLength(self) }
    }

    pub fn aligment(&self) -> usize {
        unsafe { rsel_aligment(self) }
    }

    #[inline]
    pub fn set_argument_buffer(&mut self, buffer: Option<&Buffer>, offset: usize) {
        unsafe { wsel_setArgumentBuffer(self, buffer, offset) }
    }

    #[inline]
    pub fn set_texture(&mut self, texture: Option<&Texture>, at_index: usize) {
        unsafe { wsel_setTextureAtIndex(self, texture, at_index) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_encodedLength(id: &Id) -> usize;
    fn rsel_aligment(id: &Id) -> usize;
    fn wsel_setArgumentBuffer(id: &mut Id, buffer: Option<&Buffer>, offset: usize);
    fn wsel_setTextureAtIndex(id: &mut Id, texture: Option<&Texture>, at_index: usize);

}
