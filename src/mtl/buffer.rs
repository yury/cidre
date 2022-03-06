use crate::define_obj_type;
use crate::mtl;
use crate::ns::{self, Id};

define_obj_type!(Buffer(mtl::Resource));

impl Buffer {
    pub fn length(&self) -> usize {
        unsafe { rsel_length(self) }
    }
}

extern "C" {
    fn rsel_length(id: &Id) -> ns::UInteger;
}
