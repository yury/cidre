use crate::ns::{self, Id};
use crate::{define_mtl_device_and_label, define_obj_type};

define_obj_type!(ArgumentEncoder(Id));

impl ArgumentEncoder {
    define_mtl_device_and_label!();

    pub fn encoded_length(&self) -> ns::UInteger {
        unsafe { rsel_encodedLength(self) }
    }

    pub fn aligment(&self) -> ns::UInteger {
        unsafe { rsel_aligment(self) }
    }
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    fn rsel_encodedLength(id: &Id) -> ns::UInteger;
    fn rsel_aligment(id: &Id) -> ns::UInteger;
}
