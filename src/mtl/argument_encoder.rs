use crate::ns::Id;
use crate::{define_mtl, define_obj_type};

use super::{Buffer, Texture};

define_obj_type!(ArgumentEncoder(Id));

impl ArgumentEncoder {
    define_mtl!(device, mut label);

    pub fn encoded_length(&self) -> usize {
        unsafe { rsel_encodedLength(self) }
    }

    pub fn aligment(&self) -> usize {
        unsafe { rsel_aligment(self) }
    }

    pub fn set_argument_buffer(&mut self, buffer: Option<&Buffer>, offset: usize) {
        unsafe {
            wsel_setArgumentBuffer(self, buffer, offset)
        }
    }

    pub fn set_texture(&mut self, texture: Option<&Texture>, at_index: usize) {
        unsafe {
            wsel_setTexture(self, texture, at_index)
        }
    }

}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_encodedLength(id: &Id) -> usize;
    fn rsel_aligment(id: &Id) -> usize;
    fn wsel_setArgumentBuffer(id: &mut Id, buffer: Option<&Buffer>, offset: usize);
    fn wsel_setTexture(id: &mut Id, texture: Option<&Texture>, at_index: usize);
}
